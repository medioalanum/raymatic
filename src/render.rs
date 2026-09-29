//! Rendering consumes the semantic publication, never raw source files.
use crate::{
    content::Content,
    diagnostic::{Diagnostic, DiagnosticCode, SemanticObject, Severity, SourceLabel},
};
use pulldown_cmark::{CowStr, Event, LinkType, Tag};

pub fn page(
    content: &Content,
    template_path: &std::path::Path,
    template: &str,
) -> Result<String, Box<Diagnostic>> {
    let body = content
        .body
        .source
        .text
        .get(content.body.body_range.clone())
        .expect("body range was derived from source");
    let parser = pulldown_cmark::Parser::new(body).map(|event| match event {
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) if internal_destination(&dest_url) => Event::Start(Tag::Link {
            link_type,
            dest_url: canonical_destination(dest_url),
            title,
            id,
        }),
        event => event,
    });
    let mut markdown = String::new();
    pulldown_cmark::html::push_html(&mut markdown, parser);
    let environment = minijinja::Environment::new();
    let parsed = environment
        .template_from_str(template)
        .map_err(|error| Box::new(failure(template_path, error.to_string())))?;
    parsed
        .render(minijinja::context!(
            title => &content.attributes.title.value,
            date => content.attributes.date.as_ref().map(|value| &value.value),
            category => content.attributes.category.as_ref().map(|value| &value.value),
            summary => content.attributes.summary.as_ref().map(|value| &value.value),
            body => markdown
        ))
        .map_err(|error| Box::new(failure(template_path, error.to_string())))
}

fn internal_destination(destination: &str) -> bool {
    destination.starts_with('/') && !destination.starts_with("//")
}

fn canonical_destination(destination: CowStr<'_>) -> CowStr<'_> {
    let suffix_start = destination.find(['?', '#']).unwrap_or(destination.len());
    let (path, suffix) = destination.split_at(suffix_start);
    if path == "/" || path.ends_with('/') {
        destination
    } else {
        CowStr::Boxed(format!("{path}/{suffix}").into_boxed_str())
    }
}

fn failure(path: &std::path::Path, explanation: String) -> Diagnostic {
    Diagnostic {
        severity: Severity::Error,
        code: DiagnosticCode("RENDER001"),
        summary: "Cannot render presentation template".into(),
        explanation: Some(explanation),
        primary: Some(SourceLabel {
            path: path.into(),
            span: None,
            source: None,
            message: None,
        }),
        related: vec![],
        object: Some(SemanticObject::Presentation),
        expected: Some(
            "a valid MiniJinja template using title, date, category, summary, and body".into(),
        ),
        help: Some("Correct the template syntax.".into()),
    }
}
