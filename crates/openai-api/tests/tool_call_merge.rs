use openai_api::{
    ChatCompletionChoiceDelta, ChatCompletionDelta, ChatCompletionMessageDelta,
    ChatCompletionMessageRole, ToolCall, ToolCallFunction,
};

#[test]
fn merge_tool_calls_with_indexes() {
    let mut base = ChatCompletionDelta {
        id: "id".to_string(),
        object: "obj".to_string(),
        created: 0,
        model: "gpt".to_string(),
        usage: None,
        choices: vec![ChatCompletionChoiceDelta {
            index: 0,
            finish_reason: None,
            delta: ChatCompletionMessageDelta {
                role: Some(ChatCompletionMessageRole::Assistant),
                content: None,
                name: None,
                tool_call_id: None,
                tool_calls: Some(vec![ToolCall {
                    id: "call_a".to_string(),
                    index: Some(0),
                    r#type: "function".to_string(),
                    function: ToolCallFunction {
                        name: "func_a".to_string(),
                        arguments: "arg_a".to_string(),
                    },
                }]),
            },
        }],
    };

    let delta = ChatCompletionDelta {
        id: "id".to_string(),
        object: "obj".to_string(),
        created: 0,
        model: "gpt".to_string(),
        usage: None,
        choices: vec![ChatCompletionChoiceDelta {
            index: 0,
            finish_reason: None,
            delta: ChatCompletionMessageDelta {
                role: Some(ChatCompletionMessageRole::Assistant),
                content: None,
                name: None,
                tool_call_id: None,
                tool_calls: Some(vec![ToolCall {
                    id: "call_b".to_string(),
                    index: Some(1),
                    r#type: "function".to_string(),
                    function: ToolCallFunction {
                        name: "func_b".to_string(),
                        arguments: "arg_b".to_string(),
                    },
                }]),
            },
        }],
    };

    base.merge(delta).unwrap();

    let tool_calls = base.choices[0].delta.tool_calls.as_ref().unwrap();
    assert_eq!(tool_calls.len(), 2);
    assert_eq!(tool_calls[0].id, "call_a");
    assert_eq!(tool_calls[0].index, Some(0));
    assert_eq!(tool_calls[1].id, "call_b");
    assert_eq!(tool_calls[1].index, Some(1));
}
