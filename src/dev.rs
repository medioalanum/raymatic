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
    report_rebuild(&root, &state)?;
    start_server(root.clone(), state.clone())?;
    eprintln!("Preview available at http://{ADDRESS}");
    let (sender, receiver) = mpsc::channel();
    let watch_root = root.clone();
    let mut watcher = notify::recommended_watcher(move |result: notify::Result<notify::Event>| {
        if let Ok(event) = result
            && event
                .paths
                .iter()
                .any(|path| source_path(&watch_root, path))
        {
            let _ = sender.send(());
        }
    })
    .map_err(|error| AppError::Operational(format!("Cannot start filesystem watcher: {error}")))?;
    watcher
        .watch(&root, RecursiveMode::Recursive)
        .map_err(|error| {
            AppError::Operational(format!("Cannot watch {}: {error}", root.display()))
        })?;
    loop {
        receiver
            .recv()
            .map_err(|_| AppError::Operational("Filesystem watcher stopped unexpectedly".into()))?;
        while receiver.recv_timeout(DEBOUNCE).is_ok() {}
        report_rebuild(&root, &state)?;
    }
}

fn source_path(root: &Path, path: &Path) -> bool {
    let Ok(relative) = path.strip_prefix(root) else {
        return false;
    };
    matches!(
        relative.components().next(),
        Some(Component::Normal(component))
            if matches!(component.to_str(), Some("content" | "presentation" | "assets"))
    )
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
    let url = request.url().split('?').next().unwrap_or("/");
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
    let mut path = root.join(PREVIEW_DIRECTORY).join(relative);
    if path.is_dir() {
        path = path.join("index.html");
    }
    match fs::read(&path) {
        Ok(mut bytes) => {
            let is_html = path
                .extension()
                .is_some_and(|extension| extension == "html");
            if is_html {
                bytes.extend_from_slice(reload_script().as_bytes());
            }
            let mut response = tiny_http::Response::from_data(bytes);
            if is_html {
                response.add_header(
                    tiny_http::Header::from_bytes("Content-Type", "text/html; charset=utf-8")
                        .expect("static HTML content type is valid"),
                );
            }
            let _ = request.respond(response);
        }
        Err(_) => {
            let _ = request.respond(tiny_http::Response::empty(404));
        }
    }
}

fn reload_script() -> &'static str {
    "<script>(()=>{let r;setInterval(async()=>{try{let n=await fetch('/_raymatic/revision',{cache:'no-store'}).then(x=>x.text());if(r!==undefined&&n!==r)location.reload();r=n}catch(_){}} ,500)})()</script>"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpStream,
    };

    #[test]
    fn preview_serves_publication_addresses_over_http() {
        let root = tempfile::tempdir().unwrap();
        crate::project::create(&root.path().join("site")).unwrap();
        let root = root.path().join("site");
        fs::write(root.join("content/index.md"),
            "+++\ntitle = \"Home\"\n+++\n\n[About](/about) [Notes](/notes/rust/?from=home#ownership)\n").unwrap();
        fs::write(
            root.join("content/about.md"),
            "+++\ntitle = \"About\"\n+++\n\nAbout the publication.\n",
        )
        .unwrap();
        fs::write(
            root.join("content/article.md"),
            "+++\ntitle = \"Rust\"\naddress = \"/notes/rust/\"\n+++\n\n# Ownership\n",
        )
        .unwrap();
        fs::create_dir(root.join("assets")).unwrap();
        fs::write(root.join("assets/example.txt"), "static example").unwrap();
        let mut state = DevState::default();
        assert_eq!(rebuild_once(&root, &mut state).unwrap(), Rebuild::Updated);
        let state = Arc::new(Mutex::new(state));
        let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
        let address = server.server_addr().to_ip().unwrap();
        let cases = [
            ("/", 200, "href=\"/about/\""),
            ("/about/", 200, "About the publication."),
            ("/about", 200, "About the publication."),
            ("/notes/rust/?from=home", 200, "Ownership"),
            ("/about/index.html?from=home", 200, "About the publication."),
            ("/example.txt?download=1", 200, "static example"),
            ("/_raymatic/revision?poll=1", 200, "1"),
            ("/missing/", 404, ""),
            ("/../content/index.md", 404, ""),
        ];
        let worker = thread::spawn(move || {
            for _ in 0..cases.len() {
                let request = server
                    .recv_timeout(Duration::from_secs(5))
                    .unwrap()
                    .unwrap();
                respond(request, &root, &state);
            }
        });
        for (url, status, body) in cases {
            let mut stream = TcpStream::connect(address).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            write!(
                stream,
                "GET {url} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
            )
            .unwrap();
            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            assert!(
                response.starts_with(&format!("HTTP/1.1 {status}")),
                "{url}: {response}"
            );
            assert!(response.contains(body), "{url}: {response}");
            if status == 200 && !url.starts_with("/example.txt") && !url.starts_with("/_raymatic/")
            {
                assert!(
                    response.contains("Content-Type: text/html; charset=utf-8"),
                    "{url}: {response}"
                );
                assert!(
                    response.contains("/_raymatic/revision"),
                    "{url}: {response}"
                );
            }
        }
        worker.join().unwrap();
    }

    #[test]
    fn source_events_include_optional_assets_but_ignore_generated_files() {
        let root = Path::new("/publication");
        assert!(source_path(
            root,
            Path::new("/publication/content/index.md")
        ));
        assert!(source_path(
            root,
            Path::new("/publication/presentation/page.html")
        ));
        assert!(source_path(root, Path::new("/publication/assets/site.css")));
        assert!(source_path(root, Path::new("/publication/assets")));
        assert!(!source_path(
            root,
            Path::new("/publication/output/index.html")
        ));
        assert!(!source_path(
            root,
            Path::new("/publication/.raymatic-preview/index.html")
        ));
        assert!(!source_path(root, Path::new("/other/assets/site.css")));
    }
}
