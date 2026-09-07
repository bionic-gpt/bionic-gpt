use crate::CustomError;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ExtractionResult {
    content: String,
}

pub async fn extract_markdown(bytes: &[u8], file_name: &str) -> Result<String, CustomError> {
    let endpoint = std::env::var("KREUZBERG_API_ENDPOINT")
        .unwrap_or_else(|_| "http://doc-engine:8000".to_string());
    let part = Part::bytes(bytes.to_vec()).file_name(file_name.to_string());
    let response = reqwest::Client::new()
        .post(format!("{}/extract", endpoint.trim_end_matches('/')))
        .multipart(Form::new().part("files", part))
        .send()
        .await
        .map_err(|error| CustomError::ExternalApi(format!("Xberg extraction failed: {error}")))?;

    let status = response.status();
    let body = response.text().await.map_err(|error| {
        CustomError::ExternalApi(format!("Unable to read Xberg response: {error}"))
    })?;
    if !status.is_success() {
        return Err(CustomError::ExternalApi(format!(
            "Xberg extraction failed ({status}): {body}"
        )));
    }

    let results: Vec<ExtractionResult> = serde_json::from_str(&body).map_err(|error| {
        CustomError::ExternalApi(format!("Invalid Xberg extraction response: {error}"))
    })?;
    let content = results
        .first()
        .map(|result| result.content.trim())
        .filter(|content| !content.is_empty())
        .ok_or_else(|| CustomError::ExternalApi("Xberg returned empty content".to_string()))?;

    Ok(content.to_string())
}
