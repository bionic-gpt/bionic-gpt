use std::convert::Infallible;
use std::future::ready;

use axum::extract::FromRequestParts;
use axum::http::header::ACCEPT_LANGUAGE;
use axum::http::request::Parts;

#[derive(Clone, Debug)]
pub struct Locale(pub String);

impl Locale {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn default_locale() -> String {
        let licence = db::Licence::global();
        if licence.default_lang.is_empty() {
            "en".to_string()
        } else {
            licence.default_lang.clone()
        }
    }
}

impl<S> FromRequestParts<S> for Locale
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let locale = parts
            .headers
            .get(ACCEPT_LANGUAGE)
            .and_then(|header| header.to_str().ok())
            .and_then(parse_accept_language)
            .unwrap_or_else(Locale::default_locale);

        ready(Ok(Locale(locale)))
    }
}

fn parse_accept_language(header: &str) -> Option<String> {
    header.split(',').find_map(|entry| {
        let tag = entry.split(';').next()?.trim();
        if tag.is_empty() {
            None
        } else {
            Some(tag.to_string())
        }
    })
}
