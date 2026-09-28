//! Single evaluation entry for check, build and dev. No output mutations.
use crate::{
    AppError,
    content::{Publication, SourceFile},
    diagnostic::Diagnostic,
    output::OutputPlan,
    project,
};
use std::{
    path::Path,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct PipelineSuccess {
    pub publication: Publication,
    pub output: OutputPlan,
    pub timings: PipelineTimings,
}

#[derive(Debug, Default)]
pub struct PipelineTimings {
    // None means the stage has not run; it is not a measured zero duration.
    pub discovery: Option<Duration>,
    pub parsing: Option<Duration>,
    pub semantic: Option<Duration>,
    pub validation: Option<Duration>,
    pub rendering: Option<Duration>,
    pub planning: Option<Duration>,
    pub commit: Option<Duration>,
    pub total: Duration,
}

#[derive(Debug)]
pub struct PipelineFailure {
    pub error: AppError,
    pub timings: Box<PipelineTimings>,
}

impl From<PipelineFailure> for AppError {
    fn from(failure: PipelineFailure) -> Self {
        failure.error
    }
}

pub fn evaluate(root: &Path) -> Result<PipelineSuccess, PipelineFailure> {
    let started = Instant::now();
    let mut timings = PipelineTimings::default();
    let discovered = timed(&mut timings.discovery, || project::discover(root));
    let discovered = match discovered {
        Ok(value) => value,
        Err(error) => return failure(error.into(), started, timings),
    };
    let mut diagnostics = Vec::new();
    let content = timed(&mut timings.parsing, || {
        parse_content(&discovered.content, &mut diagnostics)
    });
    let template = match project::read_text(&discovered.presentation) {
        Ok(value) => value,
        Err(error) => return failure(error.into(), started, timings),
    };
    let publication = Publication {
        root: root.into(),
        content,
        presentation: crate::content::Presentation {
            source: SourceFile {
                path: discovered.presentation.clone(),
                text: template.clone().into(),
            },
            template,
        },
        assets: vec![],
    };
    timings.semantic = Some(Duration::ZERO);
    timings.semantic = Some(Duration::ZERO);
    diagnostics.extend(timed(&mut timings.validation, || {
        crate::validate::publication(&publication)
    }));
    let rendered = timed(&mut timings.rendering, || {
        render_pages(&publication, &mut diagnostics)
    });
    sort_diagnostics(&mut diagnostics);
    if !diagnostics.is_empty() {
        return failure(AppError::InvalidPublication(diagnostics), started, timings);
    }
    let output = match timed(&mut timings.planning, || crate::output::plan_html(rendered)) {
        Ok(value) => value,
        Err(error) => return failure(error, started, timings),
    };
    timings.total = started.elapsed();
    Ok(PipelineSuccess {
        publication,
        output,
        timings,
    })
}

fn parse_content(
    discovered: &[project::DiscoveredContent],
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<crate::content::Content> {
    let mut content = Vec::new();
    for file in discovered {
        match project::read_text(&file.path) {
            Ok(text) => match crate::content::parse(
                &file.relative_path,
                SourceFile {
                    path: file.path.clone(),
                    text: text.into(),
                },
            ) {
                Ok(item) => content.push(item),
                Err(diagnostic) => diagnostics.push(*diagnostic),
            },
            Err(error) => diagnostics.push(environment_diagnostic(error)),
        }
    }
    content
}

fn render_pages(
    publication: &Publication,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<(String, std::path::PathBuf, crate::content::Address)> {
    let mut pages = Vec::new();
    for content in &publication.content {
        match crate::render::page(
            content,
            &publication.presentation.source.path,
            &publication.presentation.template,
        ) {
            Ok(html) => pages.push((html, content.source.path.clone(), content.address.clone())),
            Err(diagnostic) => diagnostics.push(*diagnostic),
        }
    }
    pages
}

fn environment_diagnostic(error: project::EnvironmentError) -> Diagnostic {
    Diagnostic {
        severity: crate::diagnostic::Severity::Error,
        code: crate::diagnostic::DiagnosticCode("PROJECT001"),
        summary: "Invalid project structure".into(),
        explanation: Some(error.to_string()),
        primary: None,
        related: vec![],
        object: Some(crate::diagnostic::SemanticObject::Publication),
        expected: Some("the required content and presentation files".into()),
        help: Some("Create the missing conventional directory or file.".into()),
    }
}

fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|left, right| {
        let left_path = left.primary.as_ref().map(|label| &label.path);
        let right_path = right.primary.as_ref().map(|label| &label.path);
        left_path
            .cmp(&right_path)
            .then(left.code.0.cmp(right.code.0))
            .then(left.summary.cmp(&right.summary))
    });
}

fn timed<T>(slot: &mut Option<Duration>, work: impl FnOnce() -> T) -> T {
    let started = Instant::now();
    let value = work();
    *slot = Some(started.elapsed());
    value
}
fn failure(
    error: AppError,
    started: Instant,
    mut timings: PipelineTimings,
) -> Result<PipelineSuccess, PipelineFailure> {
    timings.total = started.elapsed();
    Err(PipelineFailure {
        error,
        timings: Box::new(timings),
    })
}
