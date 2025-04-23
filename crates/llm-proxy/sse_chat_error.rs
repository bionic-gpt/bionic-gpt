use axum::Error;
use serde::Serialize;
use tokio::sync::mpsc;

use super::sse_chat_enricher::{CompletionChunk, GenerationEvent};

// We needed a way to send errors to the users UI console.
// So we use the existing stream and send the message as a chunk.
pub async fn error_to_chat(
    error: &str,
    sender: mpsc::Sender<Result<GenerationEvent, Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
    sender
        .send(Ok(GenerationEvent::Text(string_to_chunk(error))))
        .await?;

    sender
        .send(Ok(GenerationEvent::End(CompletionChunk {
            delta: "[DONE]".to_string(),
            merged: None,
            snapshot: error.to_string(),
        })))
        .await?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct Choice {
    pub index: i32,
    pub delta: ChoiceDelta,
    pub finish_reason: String,
}

#[derive(Debug, Serialize)]
struct Delta {
    id: String,
    object: String,
    created: i32,
    model: String,
    system_fingerprint: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize)]
struct ChoiceDelta {
    role: String,
    content: String,
}

// During a chat setion its good to let the assitant show the error,
// otherwise they get buried in the logs.
pub fn string_to_chunk(content: &str) -> CompletionChunk {
    let delta = Delta {
        id: "chatcmpl-627".to_string(),
        object: "chat.completion.chunk".to_string(),
        created: 1714383347,
        model: "bionic-generated".to_string(),
        system_fingerprint: "fp_ollama".to_string(),
        choices: vec![Choice {
            index: 0,
            delta: ChoiceDelta {
                role: "assistant".to_string(),
                content: content.to_string(),
            },
            finish_reason: "null".to_string(),
        }],
    };
    CompletionChunk {
        delta: serde_json::to_string(&delta).unwrap(),
        merged: None,
        snapshot: "".to_string(),
    }
}
