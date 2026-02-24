use crate::types::ToolDefinition;
use futures_util::StreamExt;
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;
use rig::tool::{ToolDyn, ToolError};
use rig::wasm_compat::WasmBoxedFuture;
use serde_json::{json, Value};
use std::fmt;

const MAX_CONTENT_BYTES: usize = 1000; // final output limit
const MAX_FETCH_BYTES: usize = 64 * 1024; // fetch enough HTML to reach body text

/// Error type returned by the web tool
#[derive(Debug)]
pub enum WebToolError {
    InvalidUrl(String),
    Request(String),
}

impl fmt::Display for WebToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebToolError::InvalidUrl(u) => write!(f, "Invalid URL: {}", u),
            WebToolError::Request(e) => write!(f, "Request error: {}", e),
        }
    }
}

impl std::error::Error for WebToolError {}

/// Fetches the plain text content from the given URL
pub async fn open_url(url: String) -> Result<String, WebToolError> {
    let parsed = Url::parse(&url).map_err(|_| WebToolError::InvalidUrl(url.clone()))?;

    let response = reqwest::get(parsed)
        .await
        .map_err(|e| WebToolError::Request(e.to_string()))?;

    if !response.status().is_success() {
        return Err(WebToolError::Request(format!("HTTP {}", response.status())));
    }

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_ascii_lowercase();

    let is_html = content_type.contains("text/html");
    let is_text = content_type.starts_with("text/") || content_type.is_empty();

    if !is_html && !is_text {
        return Err(WebToolError::Request(format!(
            "Unsupported content type: {}",
            content_type
        )));
    }

    let mut stream = response.bytes_stream();
    let mut buffer: Vec<u8> = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| WebToolError::Request(e.to_string()))?;
        if buffer.len() + chunk.len() > MAX_FETCH_BYTES {
            let remaining = MAX_FETCH_BYTES - buffer.len();
            buffer.extend_from_slice(&chunk[..remaining]);
            break;
        } else {
            buffer.extend_from_slice(&chunk);
        }
    }

    let body = String::from_utf8_lossy(&buffer).to_string();

    let parsed = if is_html {
        html2text::from_read(body.as_bytes(), 120)
            .map_err(|e| WebToolError::Request(e.to_string()))?
    } else {
        body
    };

    Ok(truncate_bytes(parsed, MAX_CONTENT_BYTES))
}

fn truncate_bytes(mut text: String, max_bytes: usize) -> String {
    if text.len() <= max_bytes {
        return text;
    }

    while text.len() > max_bytes {
        text.pop();
    }
    text
}

/// A tool that fetches a URL and returns the page text
pub struct WebTool;

async fn execute_web(arguments: &Value) -> Result<serde_json::Value, serde_json::Value> {
    let url = arguments["url"]
        .as_str()
        .ok_or_else(|| json!({"error": "Missing url"}))?;

    match open_url(url.to_string()).await {
        Ok(content) => Ok(json!({"content": content})),
        Err(e) => Err(json!({"error": e.to_string()})),
    }
}

impl ToolDyn for WebTool {
    fn name(&self) -> String {
        get_open_url_tool().name
    }

    fn definition(&self, _prompt: String) -> WasmBoxedFuture<'_, ToolDefinition> {
        Box::pin(async move { get_open_url_tool() })
    }

    fn call(&self, args: String) -> WasmBoxedFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let arguments: Value = serde_json::from_str(&args).map_err(ToolError::JsonError)?;
            let result = execute_web(&arguments).await.map_err(|err| {
                ToolError::ToolCallError(Box::new(std::io::Error::other(err.to_string())))
            })?;
            serde_json::to_string(&result).map_err(ToolError::JsonError)
        })
    }
}

/// Returns the tool definition for the Open URL tool
pub fn get_open_url_tool() -> ToolDefinition {
    ToolDefinition {
        name: "open_url".to_string(),
        description: "The Open URL tool lets me fetch and read the content of a webpage when you provide a specific link (URL).\n\nHow it works: You give me a URL, and I retrieve the text content from that page. I can then summarize, analyze, or pull out specific info for you.\n\nWhat it’s useful for:\n* Summarizing articles, blog posts, or reports.\n* Extracting important details from a specific webpage.\n* Checking the content of a document or page you want to discuss.\n\nWhat it can’t do:\n* It won’t interact with web forms, download files, or access content behind logins/paywalls.\n* It’s not meant for browsing the web in real time—just for fetching and reading the content of links you provide.".to_string(),
        parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL of the webpage to fetch"
                    }
                },
                "required": ["url"]
            }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_open_url_invalid() {
        let result = open_url("not a url".to_string()).await;
        assert!(result.is_err());
    }
}
