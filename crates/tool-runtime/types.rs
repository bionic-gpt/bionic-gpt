use serde::{Deserialize, Serialize};

pub type ToolDefinition = rig::completion::ToolDefinition;

pub type Reasoning = rig::message::Sealed<rig::message::Reasoning>;
pub type ToolCallFunction = rig::message::ToolFunction;
pub type ToolName = rig::message::ToolName;
pub type ToolCall = rig::message::ToolCall;
pub type ToolResult = rig::message::ToolResult;
pub type ToolResultContent = rig::message::ToolResultContent;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StoredAssistantToolState {
    #[serde(default)]
    pub reasoning: Vec<Reasoning>,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

pub fn parse_tool_calls(tool_calls_json: Option<&str>) -> Vec<ToolCall> {
    parse_assistant_tool_state(tool_calls_json).tool_calls
}

pub fn parse_reasoning(tool_calls_json: Option<&str>) -> Vec<Reasoning> {
    parse_assistant_tool_state(tool_calls_json).reasoning
}

pub fn serialize_assistant_tool_state(
    tool_calls: Option<&[ToolCall]>,
    reasoning: Option<&[Reasoning]>,
) -> Option<String> {
    let tool_calls = tool_calls.unwrap_or_default();
    let reasoning = reasoning.unwrap_or_default();

    if tool_calls.is_empty() && reasoning.is_empty() {
        return None;
    }

    if reasoning.is_empty() {
        return serde_json::to_string(tool_calls).ok();
    }

    serde_json::to_string(&StoredAssistantToolState {
        reasoning: reasoning.to_vec(),
        tool_calls: tool_calls.to_vec(),
    })
    .ok()
}

fn parse_assistant_tool_state(tool_calls_json: Option<&str>) -> StoredAssistantToolState {
    let Some(s) = tool_calls_json else {
        return StoredAssistantToolState::default();
    };

    let Ok(value) = serde_json::from_str::<serde_json::Value>(s) else {
        return StoredAssistantToolState::default();
    };
    let (tool_calls, reasoning) = match &value {
        serde_json::Value::Array(_) => (value.clone(), serde_json::Value::Array(Vec::new())),
        serde_json::Value::Object(state) => (
            state
                .get("tool_calls")
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(Vec::new())),
            state
                .get("reasoning")
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(Vec::new())),
        ),
        _ => return StoredAssistantToolState::default(),
    };

    StoredAssistantToolState {
        tool_calls: parse_tool_call_values(tool_calls),
        reasoning: parse_reasoning_values(reasoning),
    }
}

fn parse_tool_call_values(value: serde_json::Value) -> Vec<ToolCall> {
    let serde_json::Value::Array(calls) = value else {
        return Vec::new();
    };

    calls
        .into_iter()
        .filter_map(|mut call| {
            let object = call.as_object_mut()?;
            if let Some(old_id) = object.get("id").and_then(serde_json::Value::as_str) {
                let old_id = old_id.to_owned();
                let provider = object.remove("provider").unwrap_or(serde_json::Value::Null);
                let call_id = provider
                    .get("call_id")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or(&old_id)
                    .to_owned();
                let item_id = provider
                    .get("item_id")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned);
                let mut provider_id = serde_json::Map::new();
                provider_id.insert("call_id".into(), serde_json::Value::String(call_id));
                if let Some(item_id) = item_id {
                    provider_id.insert("item_id".into(), serde_json::Value::String(item_id));
                }
                object.insert("id".into(), serde_json::json!({ "provider": provider_id }));
            }
            serde_json::from_value(call).ok()
        })
        .collect()
}

fn parse_reasoning_values(value: serde_json::Value) -> Vec<Reasoning> {
    let serde_json::Value::Array(items) = value else {
        return Vec::new();
    };

    items
        .into_iter()
        .filter_map(|item| {
            if let Ok(reasoning) = serde_json::from_value::<Reasoning>(item.clone()) {
                return Some(reasoning);
            }

            // Older stored reasoning has no issuer. Do not replay it to a new
            // provider: its origin cannot be established safely.
            serde_json::from_value::<rig::message::Reasoning>(item)
                .ok()
                .map(|reasoning| reasoning.sealed(rig::message::Issuer::new("legacy/unknown")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_tool_calls_migrate_provider_ids_into_call_ids() {
        let legacy = serde_json::json!([{
            "id": "local-call-id",
            "provider": {"call_id": "provider-call-id", "item_id": "provider-item-id"},
            "function": {"name": "run_bash", "arguments": {"commands": "pwd"}},
            "signature": null,
            "additional_params": null
        }]);

        let encoded = legacy.to_string();
        let calls = parse_tool_calls(Some(&encoded));

        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].id.to_string(), "provider-call-id");
        let provider = calls[0].id.provider().expect("provider id should persist");
        assert_eq!(provider.call_id, "provider-call-id");
        assert_eq!(provider.item_id.as_deref(), Some("provider-item-id"));
        assert_eq!(calls[0].function.name.as_str(), "run_bash");
    }

    #[test]
    fn legacy_unsealed_reasoning_is_not_replayed_to_an_unrelated_provider() {
        let legacy = serde_json::json!({
            "reasoning": [{
                "id": "reasoning-1",
                "content": [{"type": "text", "content": {"text": "private thought"}}]
            }],
            "tool_calls": []
        });

        let encoded = legacy.to_string();
        let reasoning = parse_reasoning(Some(&encoded));

        assert_eq!(reasoning.len(), 1);
        assert_eq!(reasoning[0].issuer().as_str(), "legacy/unknown");
        assert!(reasoning[0]
            .open(&rig::message::Issuer::from_static("openai"))
            .is_none());
    }
}
