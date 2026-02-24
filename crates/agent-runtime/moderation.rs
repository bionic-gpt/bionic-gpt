use db::PromptFlagType;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use rig::message::{AssistantContent, Message, UserContent};
use serde::Deserialize;

pub fn strip_tool_data(messages: &[Message]) -> Vec<Message> {
    messages
        .iter()
        .cloned()
        .filter_map(|m| match m {
            Message::User { content } => {
                let kept: Vec<UserContent> = content
                    .into_iter()
                    .filter(|c| !matches!(c, UserContent::ToolResult(_)))
                    .collect();

                if kept.is_empty() {
                    None
                } else {
                    Some(Message::User {
                        content: rig::OneOrMany::many(kept)
                            .unwrap_or_else(|_| rig::OneOrMany::one(UserContent::text(""))),
                    })
                }
            }
            Message::Assistant { id, content } => {
                let kept: Vec<AssistantContent> = content
                    .into_iter()
                    .filter(|c| !matches!(c, AssistantContent::ToolCall(_)))
                    .collect();

                if kept.is_empty() {
                    None
                } else {
                    Some(Message::Assistant {
                        id,
                        content: rig::OneOrMany::many(kept)
                            .unwrap_or_else(|_| rig::OneOrMany::one(AssistantContent::text(""))),
                    })
                }
            }
        })
        .collect()
}

pub enum ModerationVerdict {
    Safe,
    Unsafe(PromptFlagType),
}

pub async fn moderate_chat(
    base_url: &str,
    api_key: Option<&str>,
    model_name: &str,
    messages: Vec<Message>,
) -> Result<ModerationVerdict, StatusCode> {
    let completion = serde_json::json!({
        "model": model_name,
        "messages": messages,
    });

    let client = reqwest::Client::new();
    let mut request = client
        .post(format!("{}/chat/completions", base_url))
        .header(CONTENT_TYPE, "application/json");
    if let Some(key) = api_key {
        request = request.header(AUTHORIZATION, format!("Bearer {}", key));
    }

    let resp = request
        .body(completion.to_string())
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    if !resp.status().is_success() {
        return Err(resp.status());
    }

    #[derive(Deserialize)]
    struct ResponseChoice {
        message: Message,
    }

    #[derive(Deserialize)]
    struct CompletionResponse {
        choices: Vec<ResponseChoice>,
    }

    let CompletionResponse { choices } = resp.json().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    let content = choices
        .first()
        .and_then(|c| match &c.message {
            Message::Assistant { content, .. } => content.iter().find_map(|item| match item {
                AssistantContent::Text(text) => Some(text.text.clone()),
                AssistantContent::ToolCall(_) | AssistantContent::Reasoning(_) => None,
            }),
            Message::User { .. } => None,
        })
        .unwrap_or_default();

    let content = content.trim();

    if content.to_lowercase().starts_with("safe") {
        Ok(ModerationVerdict::Safe)
    } else {
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
