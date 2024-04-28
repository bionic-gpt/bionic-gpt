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
) -> Result<(), Box<dyn std::error::Error>> {
    // Start streaming
    let mut stream = ReqwestEventSource::new(request)?;

    // Handle streaming events
    while let Some(event) = stream.next().await {
        match event {
            Ok(ReqwestEvent::Open) => println!("Connection Open!"),
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
                println!("Error: {}", err);
                stream.close();
                if sender.send(Err(axum::Error::new(err))).await.is_err() {
                    break; // Receiver has dropped, stop sending.
                }
            }
        }
    }

    Ok(())
}
