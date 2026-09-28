//! Coarse rebuild, preview snapshot, blocking server and revision polling.
use crate::{AppError, diagnostic::Diagnostic, output, pipeline};
use notify::{RecursiveMode, Watcher};
use std::{
    fs,
    path::{Component, Path, PathBuf},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::{Duration, Instant},
};

const PREVIEW_DIRECTORY: &str = ".raymatic-preview";
const ADDRESS: &str = "127.0.0.1:3000";
const DEBOUNCE: Duration = Duration::from_millis(75);

#[derive(Debug, Default)]
pub struct DevState {
    pub revision: u64,
    pub last_diagnostics: Vec<Diagnostic>,
    pub last_successful_rebuild: Option<Duration>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Rebuild {
    Updated,
    Invalid,
}

pub fn run(root: &Path) -> Result<(), AppError> {
    let root = root.to_path_buf();
    let state = Arc::new(Mutex::new(DevState::default()));
    start_server(root.clone(), state.clone())?;
    report_rebuild(&root, &state)?;
    eprintln!("Preview available at http://{ADDRESS}");
    let (sender, receiver) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |_| {
        let _ = sender.send(());
    })
    .map_err(|error| AppError::Operational(format!("Cannot start filesystem watcher: {error}")))?;
    for directory in ["content", "presentation"] {
        watcher
            .watch(&root.join(directory), RecursiveMode::Recursive)
            .map_err(|error| {
                AppError::Operational(format!(
                    "Cannot watch {}: {error}",
                    root.join(directory).display()
                ))
            })?;
    }
    loop {
        receiver
            .recv()
            .map_err(|_| AppError::Operational("Filesystem watcher stopped unexpectedly".into()))?;
        while receiver.recv_timeout(DEBOUNCE).is_ok() {}
        report_rebuild(&root, &state)?;
    }
}

pub fn rebuild_once(root: &Path, state: &mut DevState) -> Result<Rebuild, AppError> {
    let started = Instant::now();
    match pipeline::evaluate(root) {
        Ok(success) => {
            output::commit_named(root, PREVIEW_DIRECTORY, &success.output)?;
            state.revision += 1;
            state.last_diagnostics.clear();
            state.last_successful_rebuild = Some(started.elapsed());
            Ok(Rebuild::Updated)
        }
        Err(failure) => match failure.error {
            AppError::InvalidPublication(diagnostics) => {
                state.last_diagnostics = diagnostics;
                Ok(Rebuild::Invalid)
            }
            error => Err(error),
        },
    }
}

fn report_rebuild(root: &Path, state: &Arc<Mutex<DevState>>) -> Result<(), AppError> {
    let mut state = state
        .lock()
        .map_err(|_| AppError::Internal("Development state lock poisoned"))?;
    match rebuild_once(root, &mut state)? {
        Rebuild::Updated => eprintln!("Preview updated (revision {}).", state.revision),
        Rebuild::Invalid => {
            for diagnostic in &state.last_diagnostics {
                eprint!("{}", crate::diagnostic::render(diagnostic));
            }
        }
    }
    Ok(())
}

fn start_server(root: PathBuf, state: Arc<Mutex<DevState>>) -> Result<(), AppError> {
    let server = tiny_http::Server::http(ADDRESS).map_err(|error| {
        AppError::Operational(format!("Cannot start preview server at {ADDRESS}: {error}"))
    })?;
    thread::spawn(move || {
        for request in server.incoming_requests() {
            respond(request, &root, &state);
        }
    });
    Ok(())
}

fn respond(request: tiny_http::Request, root: &Path, state: &Arc<Mutex<DevState>>) {
    let url = request.url();
    if url == "/_raymatic/revision" {
        let revision = state.lock().map(|state| state.revision).unwrap_or_default();
        let _ = request.respond(tiny_http::Response::from_string(revision.to_string()));
        return;
    }
    let relative = if url == "/" {
        PathBuf::from("index.html")
    } else {
        PathBuf::from(url.trim_start_matches('/'))
    };
    if relative.components().any(|part| {
        matches!(
            part,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        let _ = request.respond(tiny_http::Response::empty(404));
        return;
    }
    let path = root.join(PREVIEW_DIRECTORY).join(relative);
    match fs::read(&path) {
        Ok(mut bytes) => {
            if path
                .extension()
                .is_some_and(|extension| extension == "html")
            {
                bytes.extend_from_slice(reload_script().as_bytes());
            }
            let _ = request.respond(tiny_http::Response::from_data(bytes));
        }
        Err(_) => {
            let _ = request.respond(tiny_http::Response::empty(404));
        }
    }
}

fn reload_script() -> &'static str {
    "<script>(()=>{let r;setInterval(async()=>{try{let n=await fetch('/_raymatic/revision',{cache:'no-store'}).then(x=>x.text());if(r!==undefined&&n!==r)location.reload();r=n}catch(_){}} ,500)})()</script>"
}
