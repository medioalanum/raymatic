//! Rendering consumes the semantic publication, never raw source files.
use crate::{
    content::Content,
    diagnostic::{Diagnostic, DiagnosticCode, SemanticObject, Severity, SourceLabel},
};
use pulldown_cmark::{CowStr, Event, Tag};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct PublicationEntry {
    pub title: String,
    pub date: Option<String>,
    pub display_date: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub address: String,
}

pub fn display_date(value: Option<&str>) -> Option<String> {
    let value = value?;
    let mut parts = value.split('-');
    let year: i32 = parts.next()?.parse().ok()?;
    let month: usize = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    let months = [
        "",
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let offsets = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut adjusted_year = year;
    if month < 3 {
        adjusted_year -= 1;
    }
    let weekday_index = (adjusted_year + adjusted_year / 4 - adjusted_year / 100
        + adjusted_year / 400
        + offsets[month - 1]
        + day as i32)
        % 7;
    let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    Some(format!(
        "{} {day} {} {year}",
        weekdays.get(weekday_index as usize)?,
        months.get(month)?,
    ))
}

pub fn page(
    content: &Content,
    template_path: &std::path::Path,
    template: &str,
    recent: &[PublicationEntry],
    previous: Option<&PublicationEntry>,
    next: Option<&PublicationEntry>,
    site: &crate::content::SiteConfig,
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
            dest_url: canonical_destination(dest_url, content.address.as_path()),
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
    let language = content
        .attributes
        .language
        .as_ref()
        .map(|value| value.value.as_str())
        .unwrap_or("en");
    let description = content
        .attributes
        .summary
        .as_ref()
        .map(|value| value.value.as_str())
        .unwrap_or(content.attributes.title.value.as_str());
    let canonical_path = content
        .attributes
        .canonical
        .as_ref()
        .map(|value| value.value.as_str())
        .unwrap_or(content.address.as_path());
    let canonical_url = site.base_url.as_deref().map_or_else(
        || canonical_path.to_owned(),
        |base_url| {
            format!(
                "{}/{}",
                base_url.trim_end_matches('/'),
                canonical_path.trim_start_matches('/')
            )
        },
    );
    let og_title = content
        .attributes
        .og_title
        .as_ref()
        .map(|value| value.value.as_str())
        .unwrap_or(content.attributes.title.value.as_str());
    let og_description = content
        .attributes
        .og_description
        .as_ref()
        .map(|value| value.value.as_str())
        .unwrap_or(description);
    let twitter_card = content
        .attributes
        .twitter_card
        .as_ref()
        .map(|value| value.value.as_str())
        .unwrap_or("summary_large_image");
    let word_count = body.split_whitespace().count();
    let reading_minutes = ((word_count.max(1) as f64) / 200.0).ceil() as usize;
    parsed
        .render(minijinja::context!(
            title => &content.attributes.title.value,
            site_title => site.title.as_deref().unwrap_or("Raymatic publication"),
            site_author => site.author.as_deref(),
            site_description => site.description.as_deref(),
            site_language => site.language.as_deref().unwrap_or("en"),
            base_url => site.base_url.as_deref(),
            social_links => &site.social_links,
            date => content.attributes.date.as_ref().map(|value| &value.value),
            display_date => display_date(content.attributes.date.as_ref().map(|value| value.value.as_str())),
            category => content.attributes.category.as_ref().map(|value| &value.value),
            tags => &content.attributes.tags,
            author => content.attributes.author.as_ref().map(|value| &value.value),
            draft => content.attributes.draft,
            language => language,
            canonical_url => canonical_url,
            description => description,
            og_image => content
                .attributes
                .og_image
                .as_ref()
                .map(|value| &value.value),
            og_title => og_title,
            og_description => og_description,
            twitter_card => twitter_card,
            og_type => if content.kind == crate::content::ContentKind::Article { "article" } else { "website" },
            reading_minutes => reading_minutes,
            previous => previous,
            next => next,
            summary => content.attributes.summary.as_ref().map(|value| &value.value),
            attributes => &content.attributes.custom,
            recent => recent,
            body => markdown
        ))
        .map_err(|error| Box::new(failure(template_path, error.to_string())))
}

fn internal_destination(destination: &str) -> bool {
    !destination.starts_with("//")
        && !destination.starts_with("http:")
        && !destination.starts_with("https:")
        && !destination.starts_with("mailto:")
        && !destination.starts_with('#')
        && !destination.starts_with('?')
}

fn canonical_destination<'a>(destination: CowStr<'a>, source_address: &str) -> CowStr<'a> {
    let suffix_start = destination.find(['?', '#']).unwrap_or(destination.len());
    let (path, suffix) = destination.split_at(suffix_start);
    let path = if path.starts_with('/') {
        path.to_owned()
    } else {
        let base = source_address.trim_end_matches('/');
        let parent = if source_address.ends_with('/') {
            base
        } else {
            base.rsplit_once('/')
                .map(|(parent, _)| parent)
                .unwrap_or("")
        };
        format!("{parent}/{path}")
    };
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
            "a valid MiniJinja template using title, date, category, tags, author, draft, summary, attributes, and body"
                .into(),
        ),
        help: Some("Correct the template syntax.".into()),
    }
}
