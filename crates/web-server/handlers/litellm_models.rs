//! Reads the models a LiteLLM proxy serves from its `/model/info` endpoint.
//!
//! A LiteLLM proxy exposes one OpenAI-compatible API for many providers, and
//! every alias it serves comes with metadata: the mode (chat or embedding),
//! the context window and capability flags. Importing that metadata saves
//! administrators from typing a context size and capabilities per model.

use serde::Deserialize;
use std::time::Duration;

/// Context size for aliases whose window the proxy does not report (custom
/// deployments such as a self-hosted vLLM). Matches the manual model form.
pub const FALLBACK_CONTEXT_SIZE: i32 = 2048;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteLlmModelKind {
    Chat,
    Embedding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteLlmModel {
    /// The alias clients send as `model`.
    pub name: String,
    /// The deployment behind the alias, e.g. `azure/gpt-4.1-mini`.
    pub backend: Option<String>,
    pub kind: LiteLlmModelKind,
    pub context_size: i32,
    pub function_calling: bool,
    pub vision: bool,
}

impl LiteLlmModel {
    pub fn description(&self) -> String {
        match &self.backend {
            Some(backend) => format!("Served by LiteLLM ({backend})."),
            None => "Served by LiteLLM.".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct ModelInfoResponse {
    #[serde(default)]
    data: Vec<ModelInfoEntry>,
}

#[derive(Deserialize)]
struct ModelInfoEntry {
    model_name: Option<String>,
    litellm_params: Option<LiteLlmParams>,
    model_info: Option<ModelInfo>,
}

#[derive(Deserialize)]
struct LiteLlmParams {
    model: Option<String>,
}

#[derive(Deserialize, Default)]
struct ModelInfo {
    mode: Option<String>,
    max_input_tokens: Option<i64>,
    max_tokens: Option<i64>,
    supports_function_calling: Option<bool>,
    supports_vision: Option<bool>,
}

/// `/model/info` lives next to `/chat/completions`, so it works whether the
/// configured base URL ends in `/v1` or not.
pub fn model_info_url(base_url: &str) -> String {
    format!("{}/model/info", base_url.trim_end_matches('/'))
}

/// Parses a `/model/info` body into importable models. Chat aliases (and
/// aliases with no mode, which LiteLLM reports for custom deployments) become
/// LLMs, embedding aliases become embeddings models, and other modes (image
/// generation, audio, rerank) are skipped. A load-balanced alias appears once
/// per deployment; the first entry wins.
pub fn parse_model_info(body: &str) -> Result<Vec<LiteLlmModel>, String> {
    let response: ModelInfoResponse = serde_json::from_str(body)
        .map_err(|err| format!("unexpected /model/info response: {err}"))?;

    let mut models: Vec<LiteLlmModel> = Vec::new();
    for entry in response.data {
        let Some(name) = entry.model_name.filter(|name| !name.trim().is_empty()) else {
            continue;
        };
        if models.iter().any(|model| model.name == name) {
            continue;
        }
        let info = entry.model_info.unwrap_or_default();
        let kind = match info.mode.as_deref() {
            None | Some("chat") => LiteLlmModelKind::Chat,
            Some("embedding") => LiteLlmModelKind::Embedding,
            Some(_) => continue,
        };
        let context_size = info
            .max_input_tokens
            .or(info.max_tokens)
            .filter(|tokens| *tokens > 0)
            .map(|tokens| tokens.min(i64::from(i32::MAX)) as i32)
            .unwrap_or(FALLBACK_CONTEXT_SIZE);
        let is_chat = kind == LiteLlmModelKind::Chat;
        models.push(LiteLlmModel {
            name,
            backend: entry.litellm_params.and_then(|params| params.model),
            kind,
            context_size,
            function_calling: is_chat && info.supports_function_calling.unwrap_or(false),
            vision: is_chat && info.supports_vision.unwrap_or(false),
        });
    }
    Ok(models)
}

/// Fetches and parses the proxy's `/model/info`. The key is sent only when
/// set, because a local proxy may run without a master key.
pub async fn fetch_models(
    client: &reqwest::Client,
    base_url: &str,
    api_key: Option<&str>,
) -> Result<Vec<LiteLlmModel>, String> {
    let mut request = client
        .get(model_info_url(base_url))
        .timeout(REQUEST_TIMEOUT);
    if let Some(key) = api_key.filter(|key| !key.is_empty()) {
        request = request.bearer_auth(key);
    }
    let response = request
        .send()
        .await
        .map_err(|err| format!("could not reach the LiteLLM proxy: {err}"))?;
    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|err| format!("could not read the LiteLLM response: {err}"))?;
    if !status.is_success() {
        let snippet: String = body.chars().take(200).collect();
        return Err(format!(
            "LiteLLM returned HTTP {}: {snippet}",
            status.as_u16()
        ));
    }
    parse_model_info(&body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use axum::routing::get;
    use axum::Router;

    const MODEL_INFO: &str = r#"{"data": [
        {"model_name": "gpt-4.1-mini", "litellm_params": {"model": "azure/gpt-4.1-mini"},
         "model_info": {"mode": "chat", "max_input_tokens": 1047576, "max_tokens": 32768,
                        "supports_function_calling": true, "supports_vision": true}},
        {"model_name": "gpt-4.1-mini", "litellm_params": {"model": "openai/gpt-4.1-mini"},
         "model_info": {"mode": "chat", "max_input_tokens": 8000}},
        {"model_name": "house-model", "litellm_params": {"model": "hosted_vllm/qwen3"},
         "model_info": {"mode": null}},
        {"model_name": "emb-small", "litellm_params": {"model": "openai/text-embedding-3-small"},
         "model_info": {"mode": "embedding", "max_input_tokens": 8191,
                        "supports_function_calling": true}},
        {"model_name": "image", "model_info": {"mode": "image_generation"}},
        {"model_name": "", "model_info": {"mode": "chat"}},
        {"model_info": {"mode": "chat"}}
    ]}"#;

    #[test]
    fn parses_chat_and_embedding_aliases() {
        let models = parse_model_info(MODEL_INFO).unwrap();
        let names: Vec<&str> = models.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, ["gpt-4.1-mini", "house-model", "emb-small"]);

        // First deployment of a load-balanced alias wins.
        let chat = &models[0];
        assert_eq!(chat.kind, LiteLlmModelKind::Chat);
        assert_eq!(chat.context_size, 1047576);
        assert!(chat.function_calling && chat.vision);
        assert_eq!(chat.backend.as_deref(), Some("azure/gpt-4.1-mini"));
        assert_eq!(
            chat.description(),
            "Served by LiteLLM (azure/gpt-4.1-mini)."
        );

        // No mode and no window: still a chat model, with the form default.
        let custom = &models[1];
        assert_eq!(custom.kind, LiteLlmModelKind::Chat);
        assert_eq!(custom.context_size, FALLBACK_CONTEXT_SIZE);
        assert!(!custom.function_calling && !custom.vision);

        // Capability flags only apply to chat models.
        let embedding = &models[2];
        assert_eq!(embedding.kind, LiteLlmModelKind::Embedding);
        assert_eq!(embedding.context_size, 8191);
        assert!(!embedding.function_calling);
    }

    #[test]
    fn falls_back_to_max_tokens_and_rejects_bad_bodies() {
        let models = parse_model_info(
            r#"{"data": [{"model_name": "a", "model_info": {"mode": "chat", "max_tokens": 4096}}]}"#,
        )
        .unwrap();
        assert_eq!(models[0].context_size, 4096);
        assert!(parse_model_info("<html>").is_err());
        assert!(parse_model_info("{}").unwrap().is_empty());
    }

    #[test]
    fn model_info_url_handles_trailing_slash_and_v1() {
        assert_eq!(
            model_info_url("http://localhost:4000/v1/"),
            "http://localhost:4000/v1/model/info"
        );
        assert_eq!(
            model_info_url("http://proxy:4000"),
            "http://proxy:4000/model/info"
        );
    }

    async fn serve(router: Router) -> String {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        format!("http://{addr}/v1")
    }

    #[tokio::test]
    async fn fetch_sends_the_key_only_when_set() {
        let router = Router::new().route(
            "/v1/model/info",
            get(|headers: HeaderMap| async move {
                match headers.get("authorization").and_then(|v| v.to_str().ok()) {
                    Some("Bearer sk-proxy") | None => (axum::http::StatusCode::OK, MODEL_INFO),
                    Some(_) => (axum::http::StatusCode::UNAUTHORIZED, "bad key"),
                }
            }),
        );
        let base_url = serve(router).await;
        let client = reqwest::Client::new();

        let keyed = fetch_models(&client, &base_url, Some("sk-proxy"))
            .await
            .unwrap();
        assert_eq!(keyed.len(), 3);
        let keyless = fetch_models(&client, &base_url, None).await.unwrap();
        assert_eq!(keyless.len(), 3);

        let err = fetch_models(&client, &base_url, Some("sk-wrong"))
            .await
            .unwrap_err();
        assert!(err.contains("HTTP 401"), "{err}");
    }

    #[tokio::test]
    async fn fetch_reports_an_unreachable_proxy() {
        let client = reqwest::Client::new();
        let err = fetch_models(&client, "http://127.0.0.1:9", None)
            .await
            .unwrap_err();
        assert!(err.contains("could not reach"), "{err}");
    }
}
