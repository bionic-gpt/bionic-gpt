use db::PromptFlagType;
use openai_api::{BionicChatCompletionRequest, ChatCompletionMessage};
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use serde::Deserialize;

/// Result of running chat moderation.
pub enum ModerationVerdict {
    Safe,
    Unsafe(PromptFlagType),
}

/// Send a moderation request to the Guard model.
/// Takes the model information and the chat messages.
/// Returns `Ok(())` if the request succeeded with a 200 response
/// otherwise returns the status code of the failed request.
pub async fn moderate_chat(
    base_url: &str,
    api_key: Option<&str>,
    model_name: &str,
    messages: Vec<ChatCompletionMessage>,
) -> Result<ModerationVerdict, StatusCode> {
    let completion = BionicChatCompletionRequest {
        model: model_name.to_string(),
        stream: None,
        max_tokens: None,
        messages,
        temperature: None,
        tools: None,
        tool_choice: None,
    };

    let client = reqwest::Client::new();
    let mut request = client
        .post(format!("{}/chat/completions", base_url))
        .header(CONTENT_TYPE, "application/json");
    if let Some(key) = api_key {
        request = request.header(AUTHORIZATION, format!("Bearer {}", key));
    }
    let resp = request
        .body(serde_json::to_string(&completion).unwrap())
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    if !resp.status().is_success() {
        return Err(resp.status());
    }

    #[derive(Deserialize)]
    struct ResponseChoice {
        message: ChatCompletionMessage,
    }

    #[derive(Deserialize)]
    struct CompletionResponse {
        choices: Vec<ResponseChoice>,
    }

    let CompletionResponse { choices } = resp.json().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    let content = choices
        .first()
        .and_then(|c| c.message.content.clone())
        .unwrap_or_default();
    let content = content.trim();

    if content.to_lowercase().starts_with("safe") {
        Ok(ModerationVerdict::Safe)
    } else {
        // Expect format "unsafe\nS1" etc
        let code = content.split_whitespace().last().unwrap_or("");
        let flag = match code {
            "S1" => PromptFlagType::S1,
            "S2" => PromptFlagType::S2,
            "S3" => PromptFlagType::S3,
            "S4" => PromptFlagType::S4,
            "S5" => PromptFlagType::S5,
            "S6" => PromptFlagType::S6,
            "S7" => PromptFlagType::S7,
            "S8" => PromptFlagType::S8,
            "S9" => PromptFlagType::S9,
            "S10" => PromptFlagType::S10,
            "S11" => PromptFlagType::S11,
            "S12" => PromptFlagType::S12,
            "S13" => PromptFlagType::S13,
            "S14" => PromptFlagType::S14,
            _ => return Err(StatusCode::BAD_GATEWAY),
        };
        Ok(ModerationVerdict::Unsafe(flag))
    }
}
