use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderValue},
    middleware::Next,
    response::Response,
};
use http_body_util::BodyExt;
use std::{convert::Infallible, time::Instant};

const RENDER_TIME_HEADER: &str = "X-Render-Time";
const HTML_CACHE_CONTROL: HeaderValue =
    HeaderValue::from_static("private, max-age=10, must-revalidate");

pub async fn annotate_render_time(req: Request, next: Next) -> Result<Response, Infallible> {
    let start = Instant::now();
    let mut response = next.run(req).await;
    let elapsed_ms = start.elapsed().as_millis();

    if let Ok(value) = HeaderValue::from_str(&format!("{elapsed_ms}ms")) {
        response.headers_mut().insert(RENDER_TIME_HEADER, value);
    }

    if !is_html_response(&response) {
        return Ok(response);
    }

    let comment = format!("\n<!-- render_time: {elapsed_ms}ms -->");
    let (mut parts, body) = response.into_parts();

    if parts.headers.get(header::CACHE_CONTROL).is_none() {
        parts
            .headers
            .insert(header::CACHE_CONTROL, HTML_CACHE_CONTROL.clone());
    }

    match body.collect().await {
        Ok(collected) => {
            let mut bytes = collected.to_bytes().to_vec();
            bytes.extend_from_slice(comment.as_bytes());
            let response = Response::from_parts(parts, Body::from(bytes));
            Ok(response)
        }
        Err(err) => {
            tracing::warn!("failed to append render time comment: {err}");
            // Fall back to sending the original headers with an empty body.
            let response = Response::from_parts(parts, Body::empty());
            Ok(response)
        }
    }
}

fn is_html_response(response: &Response) -> bool {
    response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.starts_with("text/html"))
        .unwrap_or(false)
}
