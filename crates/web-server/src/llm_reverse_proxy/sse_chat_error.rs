use axum::Error;
use tokio::sync::mpsc;

use super::sse_chat_enricher::{CompletionChunk, GenerationEvent};

// We needed a way to send errors to the users UI console.
// So we use the existing stream and send the message as a chunk.
pub async fn error_to_chat(
    sender: mpsc::Sender<Result<GenerationEvent, Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
    sender
        .send(Ok(GenerationEvent::Text(string_to_chunk("Hello"))))
        .await?;

    Ok(())
}

// During a chat setion its good to let the assitant show the error,
// otherwise they get buried in the logs.
fn string_to_chunk(content: &str) -> CompletionChunk {
    let escaped_content = content.replace('\n', "\\n");
    let escaped_content = escaped_content.replace('"', "\\\"");
    let json = format!(
        r#"{{
        "id": "chatcmpl-627",
        "object": "chat.completion.chunk",
        "created": 1714383347,
        "model": "llama2",
        "system_fingerprint": "fp_ollama",
        "choices": [
          {{
            "index": 0,
            "delta": {{
              "role": "assistant",
              "content": "{}"
            }},
            "finish_reason": null
          }}
        ]
      }}"#,
        escaped_content
    );
    CompletionChunk {
        delta: json,
        snapshot: "".to_string(),
    }
}
