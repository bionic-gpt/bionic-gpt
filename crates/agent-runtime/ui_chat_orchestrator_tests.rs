#![allow(non_snake_case)]
use crate::chat_types::{
    ChatCompletionChoiceDelta, ChatCompletionDelta, ChatCompletionMessageDelta,
};
use crate::stream_assembler::GenerationEvent;
use crate::ui_chat_orchestrator::{build_event_stream, ResultSink};
use async_trait::async_trait;
use db::ChatStatus;
use std::sync::{Arc, Mutex};
use tokio::pin;
use tokio_stream::StreamExt;
use tool_runtime::{ToolCall, ToolCallFunction};

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
    async fn save(
        &self,
        snapshot: &str,
        tool_calls: Option<Vec<ToolCall>>,
        _chat_id: i32,
        _sub: &str,
        status: ChatStatus,
    ) {
        self.calls.lock().unwrap().push(SaveCall {
            snapshot: snapshot.to_string(),
            tool_calls_len: tool_calls.as_ref().map(|calls| calls.len()),
            status,
        });
    }
}

fn sample_delta_with_tool_calls() -> ChatCompletionDelta {
    ChatCompletionDelta {
        id: "id".to_string(),
        object: "chat.completion.chunk".to_string(),
        created: 0,
        model: "model".to_string(),
        choices: vec![ChatCompletionChoiceDelta {
            index: 0,
            finish_reason: None,
            delta: ChatCompletionMessageDelta {
                role: None,
                content: None,
                name: None,
                tool_call_id: None,
                tool_calls: Some(vec![ToolCall {
                    id: "call_1".to_string(),
                    index: Some(0),
                    r#type: "function".to_string(),
                    function: ToolCallFunction {
                        name: "do_thing".to_string(),
                        arguments: "{}".to_string(),
                    },
                }]),
            },
        }],
        usage: None,
    }
}

#[tokio::test]
async fn event_stream_saves_on_end_with_tool_calls() {
    let result_sink = Arc::new(FakeResultSink {
        calls: Mutex::new(Vec::new()),
    });
    let result_sink_dyn: Arc<dyn ResultSink> = result_sink.clone();
    let sub = Arc::new("user-1".to_string());

    let text_chunk = crate::stream_assembler::CompletionChunk {
        delta: "delta".to_string(),
        merged: None,
        snapshot: "final".to_string(),
    };
    let end_chunk = crate::stream_assembler::CompletionChunk {
        delta: "[DONE]".to_string(),
        merged: Some(sample_delta_with_tool_calls()),
        snapshot: "final".to_string(),
    };

    let input = tokio_stream::iter(vec![
        Ok(GenerationEvent::Text(text_chunk)),
        Ok(GenerationEvent::End(end_chunk)),
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
