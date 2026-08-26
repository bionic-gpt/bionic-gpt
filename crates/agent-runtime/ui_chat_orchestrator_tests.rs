#![allow(non_snake_case)]
use crate::chat_request::RigChatRequest;
use crate::result_sink::SaveRequest;
use crate::ui_chat_orchestrator::{
    build_event_stream, stream_chat_with_rig, GenerationEvent, ResultSink,
};
use async_trait::async_trait;
use axum::body::Body;
use axum::http::{header, Response, StatusCode};
use axum::routing::post;
use axum::Router;
use db::ChatStatus;
use rig::completion::{CompletionRequest, Message};
use rig::OneOrMany;
use serde_json::json;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::pin;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tool_runtime::{ToolCall, ToolCallFunction, ToolDefinition};

#[derive(Debug, Clone)]
struct SaveCall {
    snapshot: String,
    tool_calls_len: Option<usize>,
    status: ChatStatus,
}

struct FakeResultSink {
    calls: Mutex<Vec<SaveCall>>,
}

#[async_trait]
impl ResultSink for FakeResultSink {
    async fn save(&self, request: SaveRequest<'_>) {
        self.calls.lock().unwrap().push(SaveCall {
            snapshot: request.snapshot.to_string(),
            tool_calls_len: request.tool_calls.as_ref().map(|calls| calls.len()),
            status: request.status,
        });
    }
}

#[tokio::test]
async fn event_stream_saves_on_end_with_tool_calls() {
    let result_sink = Arc::new(FakeResultSink {
        calls: Mutex::new(Vec::new()),
    });
    let result_sink_dyn: Arc<dyn ResultSink> = result_sink.clone();
    let sub = Arc::new("user-1".to_string());

    let tool_calls = vec![ToolCall {
        id: "call_1".to_string(),
        call_id: None,
        signature: None,
        additional_params: None,
        function: ToolCallFunction {
            name: "do_thing".to_string(),
            arguments: json!({}),
        },
    }];

    let input = tokio_stream::iter(vec![
        Ok(GenerationEvent::Text {
            delta: "delta".to_string(),
        }),
        Ok(GenerationEvent::End {
            snapshot: "final".to_string(),
            tool_calls: Some(tool_calls),
            reasoning: None,
            usage: None,
        }),
    ]);

    let stream = build_event_stream(input, Arc::clone(&result_sink_dyn), 42, sub);
    pin!(stream);
    while stream.next().await.is_some() {}

    let calls = result_sink.calls.lock().unwrap().clone();

    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].snapshot, "final");
    assert_eq!(calls[0].tool_calls_len, Some(1));
    assert_eq!(calls[0].status, ChatStatus::Success);
}

#[tokio::test]
async fn event_stream_saves_on_error() {
    let result_sink = Arc::new(FakeResultSink {
        calls: Mutex::new(Vec::new()),
    });
    let result_sink_dyn: Arc<dyn ResultSink> = result_sink.clone();
    let sub = Arc::new("user-1".to_string());

    let err = axum::Error::new(std::io::Error::other("boom"));
    let input = tokio_stream::iter(vec![Err(err)]);

    let stream = build_event_stream(input, Arc::clone(&result_sink_dyn), 7, sub);
    pin!(stream);
    while stream.next().await.is_some() {}

    let calls = result_sink.calls.lock().unwrap().clone();

    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].status, ChatStatus::Error);
    assert!(calls[0].snapshot.contains("boom"));
}

#[tokio::test]
async fn event_stream_emits_error_event() {
    let result_sink = Arc::new(FakeResultSink {
        calls: Mutex::new(Vec::new()),
    });
    let result_sink_dyn: Arc<dyn ResultSink> = result_sink.clone();
    let sub = Arc::new("user-1".to_string());

    let err = axum::Error::new(std::io::Error::other("boom"));
    let input = tokio_stream::iter(vec![Err(err)]);

    let stream = build_event_stream(input, Arc::clone(&result_sink_dyn), 7, sub);
    pin!(stream);

    let first = stream.next().await.expect("expected one item");
    let event = first.expect("expected Ok(event)");
    let formatted = format!("{:?}", event);
    assert!(formatted.contains("boom"));

    assert!(stream.next().await.is_none());
}

fn tool_enabled_request(base_url: String) -> RigChatRequest {
    provider_request(db::ModelProvider::OpenAICompatible, base_url)
}

fn provider_request(provider_type: db::ModelProvider, base_url: String) -> RigChatRequest {
    RigChatRequest {
        model_name: "test-model".to_string(),
        provider_type,
        base_url,
        api_key: Some("test-key".to_string()),
        completion: CompletionRequest {
            model: None,
            preamble: None,
            chat_history: OneOrMany::one(Message::user("Say hi")),
            documents: vec![],
            tools: vec![ToolDefinition {
                name: "run_bash".to_string(),
                description: "Run shell commands in Bashkit.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "commands": {"type": "string"}
                    },
                    "required": ["commands"]
                }),
            }],
            temperature: None,
            max_tokens: None,
            tool_choice: None,
            additional_params: None,
            output_schema: None,
        },
        model_id: 1,
        user_id: 1,
    }
}

async fn start_mock_provider(app: Router) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    format!("http://{address}/v1")
}

async fn successful_chat_completion() -> Response<Body> {
    let body = concat!(
        "data: {\"id\":\"response-1\",\"model\":\"test-model\",\"choices\":[",
        "{\"index\":0,\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}",
        "],\"usage\":null}\n\n",
        "data: {\"id\":\"response-1\",\"model\":\"test-model\",\"choices\":[",
        "{\"index\":0,\"delta\":{},\"finish_reason\":\"stop\"}",
        "],\"usage\":null}\n\n",
        "data: [DONE]\n\n"
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream")
        .body(Body::from(body))
        .unwrap()
}

async fn successful_ollama_completion() -> Response<Body> {
    let body = concat!(
        "{\"model\":\"test-model\",\"created_at\":\"2026-08-26T00:00:00Z\",\"message\":{\"role\":\"assistant\",\"content\":\"Hello\"},\"done\":false}\n",
        "{\"model\":\"test-model\",\"created_at\":\"2026-08-26T00:00:00Z\",\"message\":{\"role\":\"assistant\",\"content\":\"\"},\"done\":true,\"done_reason\":\"stop\",\"prompt_eval_count\":2,\"eval_count\":1}\n"
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/x-ndjson")
        .body(Body::from(body))
        .unwrap()
}

async fn assert_successful_stream(request: RigChatRequest) {
    let (sender, mut receiver) = mpsc::channel(8);

    stream_chat_with_rig(request, sender)
        .await
        .expect("chat completion stream should succeed");

    let mut text = String::new();
    let mut ended = false;
    while let Some(event) = receiver.recv().await {
        match event.expect("generation event should succeed") {
            GenerationEvent::Text { delta } => text.push_str(&delta),
            GenerationEvent::End { snapshot, .. } => {
                assert_eq!(snapshot, "Hello");
                ended = true;
            }
        }
    }

    assert_eq!(text, "Hello");
    assert!(ended);
}

#[tokio::test]
async fn rig_stream_uses_chat_completions_for_tool_enabled_requests() {
    let base_url = start_mock_provider(
        Router::new().route("/v1/chat/completions", post(successful_chat_completion)),
    )
    .await;
    assert_successful_stream(tool_enabled_request(base_url)).await;
}

#[tokio::test]
async fn rig_stream_dispatches_seeded_openai_compatible_providers() {
    for provider_type in [
        db::ModelProvider::OpenAI,
        db::ModelProvider::Groq,
        db::ModelProvider::OpenRouter,
    ] {
        let base_url = start_mock_provider(
            Router::new().route("/v1/chat/completions", post(successful_chat_completion)),
        )
        .await;

        assert_successful_stream(provider_request(provider_type, base_url)).await;
    }
}

#[tokio::test]
async fn rig_stream_dispatches_ollama_to_native_chat_endpoint() {
    let base_url =
        start_mock_provider(Router::new().route("/api/chat", post(successful_ollama_completion)))
            .await;

    assert_successful_stream(provider_request(db::ModelProvider::Ollama, base_url)).await;
}

async fn model_not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            r#"{"error":{"message":"model does not exist","type":"invalid_request_error","code":"model_not_found"}}"#,
        ))
        .unwrap()
}

#[tokio::test]
async fn rig_stream_preserves_provider_error_body() {
    let base_url =
        start_mock_provider(Router::new().route("/v1/chat/completions", post(model_not_found)))
            .await;
    let (sender, _receiver) = mpsc::channel(8);

    let error = stream_chat_with_rig(tool_enabled_request(base_url), sender)
        .await
        .expect_err("provider error should fail the stream");
    let message = error.to_string();

    assert!(message.contains("404"));
    assert!(message.contains("model does not exist"));
    assert!(message.contains("model_not_found"));
}
