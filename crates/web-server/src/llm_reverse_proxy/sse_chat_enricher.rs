//! Run with
//!
//! ```not_rust
//! cargo run -p example-reqwest-response
//! ```
use axum::Error;
use reqwest::RequestBuilder;
use reqwest_eventsource::{Event as ReqwestEvent, EventSource as ReqwestEventSource};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

#[derive(Debug)]
pub enum GenerationEvent {
    Text(String),
    End(String),
}

// Create an SSE connection to the model and intercept the incoming stream.
// Enrich the stream i.e. add results so far.
pub async fn enriched_chat(
    request: RequestBuilder,
    sender: mpsc::Sender<Result<GenerationEvent, Error>>,
    convert_errors_to_chat: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Start streaming
    let mut stream = ReqwestEventSource::new(request)?;

    // Handle streaming events
    while let Some(event) = stream.next().await {
        match event {
            Ok(ReqwestEvent::Open) => tracing::debug!("Connection Open!"),
            Ok(ReqwestEvent::Message(message)) => {
                if message.data.trim() == "[DONE]" {
                    stream.close();
                    if sender
                        .send(Ok(GenerationEvent::End("[DONE]".to_string())))
                        .await
                        .is_err()
                    {
                        break; // Receiver has dropped, stop sending.
                    }
                    break;
                } else {
                    let m: Value = serde_json::from_str(&message.data).unwrap();
                    if sender
                        .send(Ok(GenerationEvent::Text(m.to_string())))
                        .await
                        .is_err()
                    {
                        break; // Receiver has dropped, stop sending.
                    }
                }
            }
            Err(err) => {
                tracing::error!("{}", err);
                if convert_errors_to_chat {
                    sender
                        .send(Ok(GenerationEvent::Text(convert_error_to_chat(err))))
                        .await?;
                    sender
                        .send(Ok(GenerationEvent::End("[DONE]".to_string())))
                        .await?;
                } else if sender.send(Err(axum::Error::new(err))).await.is_err() {
                    break; // Receiver has dropped, stop sending.
                }
                stream.close();
            }
        }
    }

    Ok(())
}
// During a chat setion its good to let the assitant show the error,
// otherwise they get buried in the logs.
fn convert_error_to_chat(err: reqwest_eventsource::Error) -> String {
    format!(
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
              "content": "Unable to complete your request due to the following error ```{}```"
            }},
            "finish_reason": "stop"
          }}
        ]
      }}"#,
        err
    )
}
