//! Product diagnostic data and its terminal presentation boundary.
use crate::content::{Address, SourceFile};
use std::{fmt::Write, ops::Range, path::PathBuf};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DiagnosticCode(pub &'static str);

#[derive(Debug)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub summary: String,
    pub explanation: Option<String>,
    pub primary: Option<SourceLabel>,
    pub related: Vec<SourceLabel>,
    pub object: Option<SemanticObject>,
    pub expected: Option<String>,
    pub help: Option<String>,
}

#[derive(Debug)]
pub struct SourceLabel {
    pub path: PathBuf,
    pub span: Option<Range<usize>>,
    pub source: Option<SourceFile>,
    pub message: Option<String>,
}

#[derive(Debug)]
pub enum SemanticObject {
    Publication,
    Content { address: Option<Address> },
    Presentation,
    Asset { relative_path: PathBuf },
    Reference { target: String },
}

pub fn render(diagnostic: &Diagnostic) -> String {
    let severity = match diagnostic.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    };
    let mut text = format!(
        "{severity}[{}]: {}\n",
        diagnostic.code.0, diagnostic.summary
    );
    for label in diagnostic.primary.iter().chain(&diagnostic.related) {
        let _ = write!(text, "  --> {}", label.path.display());
        if let (Some(source), Some(span)) = (&label.source, &label.span)
            && source.path == label.path
            && source.text.get(span.clone()).is_some()
            && let Some((line, column)) = source.line_column(span.start)
        {
            let _ = write!(text, ":{line}:{column}");
        }
        if let Some(message) = &label.message {
            let _ = write!(text, ": {message}");
        }
        text.push('\n');
    }
    for (prefix, value) in [
        ("", &diagnostic.explanation),
        ("expected: ", &diagnostic.expected),
        ("help: ", &diagnostic.help),
    ] {
        if let Some(value) = value {
            let _ = writeln!(text, "  {prefix}{value}");
        }
    }
    text
}
