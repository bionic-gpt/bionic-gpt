#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde_json::Value;
use tool_runtime::ToolCall;

use super::canvas::{is_canvas_output, parse_generated_outputs, CanvasOutput, GeneratedFiles};

#[derive(Clone, Debug, PartialEq)]
pub struct ActivityAction {
    pub chat_id: i64,
    pub pending: bool,
    pub tool_call_id: Option<String>,
    pub tool_call: Option<ToolCall>,
    pub response: Option<String>,
}

fn format_json_string(raw: &str) -> String {
    if let Ok(value) = serde_json::from_str::<Value>(raw) {
        serde_json::to_string_pretty(&value).unwrap_or_else(|_| raw.to_string())
    } else {
        raw.to_string()
    }
}

fn format_json_value(raw: &Value) -> String {
    serde_json::to_string_pretty(raw).unwrap_or_else(|_| raw.to_string())
}

fn raw_tool_name(action: &ActivityAction) -> String {
    action
        .tool_call
        .as_ref()
        .map(|call| call.function.name.trim())
        .filter(|name| !name.is_empty())
        .unwrap_or("Unknown tool")
        .to_string()
}

fn humanize_identifier(name: &str) -> String {
    let mut output = String::new();
    let mut previous_was_lowercase = false;

    for character in name.chars() {
        if matches!(character, '_' | '-' | '.') {
            if !output.ends_with(' ') {
                output.push(' ');
            }
            previous_was_lowercase = false;
        } else {
            if character.is_uppercase() && previous_was_lowercase {
                output.push(' ');
            }
            output.extend(character.to_lowercase());
            previous_was_lowercase = character.is_lowercase() || character.is_ascii_digit();
        }
    }

    let normalized = output.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut characters = normalized.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().collect::<String>() + characters.as_str(),
        None => "Unknown tool".to_string(),
    }
}

fn action_label(action: &ActivityAction) -> String {
    let tool_name = raw_tool_name(action);
    if tool_name == "run_bash" {
        "Ran command".to_string()
    } else {
        format!("Used {}", humanize_identifier(&tool_name))
    }
}

fn command_preview(action: &ActivityAction) -> Option<String> {
    if raw_tool_name(action) != "run_bash" {
        return None;
    }

    action
        .tool_call
        .as_ref()
        .and_then(|call| call.function.arguments.get("commands"))
        .and_then(Value::as_str)
        .filter(|command| !command.trim().is_empty())
        .map(str::to_string)
}

#[component]
pub fn AssistantActivity(
    team_id: String,
    reasoning: Vec<String>,
    actions: Vec<ActivityAction>,
) -> Element {
    if reasoning.is_empty() && actions.is_empty() {
        return rsx! {};
    }

    let pending = actions.iter().any(|action| action.pending);
    let action_count = actions.len();
    let summary = match (pending, action_count, reasoning.is_empty()) {
        (true, 0, _) => "Working".to_string(),
        (true, count, _) => format!(
            "Working · {count} {}",
            if count == 1 { "action" } else { "actions" }
        ),
        (false, 0, false) => "Thinking".to_string(),
        (false, count, _) => format!("{count} {}", if count == 1 { "action" } else { "actions" }),
    };

    let generated_outputs = actions
        .iter()
        .flat_map(|action| parse_generated_outputs(action.response.as_deref()))
        .collect::<Vec<_>>();
    let canvas_outputs = generated_outputs
        .iter()
        .filter(|output| is_canvas_output(output))
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        div {
            class: "assistant-activity space-y-3",
            details {
                class: "group rounded-lg text-sm text-base-content/70",
                summary {
                    class: "flex w-fit cursor-pointer list-none items-center gap-2 rounded-md py-1 pr-2 font-medium hover:text-base-content [&::-webkit-details-marker]:hidden",
                    span {
                        class: "text-xs transition-transform group-open:rotate-90",
                        "▶"
                    }
                    span { "{summary}" }
                }
                div {
                    class: "mt-2 ml-1 space-y-4 border-l border-base-300 pl-4",
                    if !reasoning.is_empty() {
                        div {
                            class: "space-y-2",
                            div { class: "font-medium text-base-content/80", "Thinking" }
                            for text in reasoning.iter() {
                                p {
                                    class: "whitespace-pre-wrap break-words text-sm leading-relaxed",
                                    "{text}"
                                }
                            }
                        }
                    }
                    for action in actions.iter() {
                        div {
                            class: "min-w-0 space-y-2",
                            div {
                                class: "flex items-start gap-2",
                                span {
                                    class: if action.pending { "text-base-content/50" } else { "text-success" },
                                    if action.pending { "…" } else { "✓" }
                                }
                                div {
                                    class: "min-w-0",
                                    div { class: "font-medium text-base-content/80", "{action_label(action)}" }
                                    if let Some(command) = command_preview(action) {
                                        pre {
                                            class: "mt-1 max-h-32 overflow-auto whitespace-pre-wrap break-words rounded-md bg-base-200 p-2 font-mono text-xs",
                                            "{command}"
                                        }
                                    }
                                }
                            }
                            details {
                                class: "ml-6",
                                summary {
                                    class: "w-fit cursor-pointer text-xs hover:text-base-content",
                                    "Technical details"
                                }
                                dl {
                                    class: "mt-2 space-y-3 text-xs",
                                    div {
                                        dt { class: "font-semibold text-base-content/60", "Tool" }
                                        dd { class: "break-words font-mono", "{raw_tool_name(action)}" }
                                    }
                                    if let Some(call) = action.tool_call.as_ref() {
                                        div {
                                            dt { class: "font-semibold text-base-content/60", "Call ID" }
                                            dd { class: "break-all font-mono", "{call.id}" }
                                        }
                                        div {
                                            dt { class: "font-semibold text-base-content/60", "Request" }
                                            pre {
                                                class: "json mt-1 max-h-80 overflow-auto rounded-md bg-base-200 p-3 whitespace-pre-wrap break-words",
                                                "{format_json_value(&call.function.arguments)}"
                                            }
                                        }
                                    } else if let Some(id) = action.tool_call_id.as_ref() {
                                        div {
                                            dt { class: "font-semibold text-base-content/60", "Call ID" }
                                            dd { class: "break-all font-mono", "{id}" }
                                        }
                                    }
                                    div {
                                        dt { class: "font-semibold text-base-content/60", "Response" }
                                        if let Some(response) = action.response.as_ref().filter(|response| !response.trim().is_empty()) {
                                            pre {
                                                class: "json mt-1 max-h-80 overflow-auto rounded-md bg-base-200 p-3 whitespace-pre-wrap break-words",
                                                "{format_json_string(response)}"
                                            }
                                        } else if action.pending {
                                            dd { "Awaiting tool response…" }
                                        } else {
                                            dd { "No response recorded." }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for output in canvas_outputs {
                CanvasOutput {
                    team_id: team_id.clone(),
                    output
                }
            }
            GeneratedFiles {
                team_id: team_id.clone(),
                outputs: generated_outputs
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tool_runtime::ToolCallFunction;

    fn action(name: &str, arguments: Value) -> ActivityAction {
        ActivityAction {
            chat_id: 1,
            pending: false,
            tool_call_id: Some("call-1".to_string()),
            tool_call: Some(ToolCall::from_wire(
                "call-1",
                ToolCallFunction::new(name.to_string(), arguments),
            )),
            response: None,
        }
    }

    #[test]
    fn labels_run_bash_without_exposing_implementation_name() {
        let action = action("run_bash", json!({"commands": "ls -la"}));
        assert_eq!(action_label(&action), "Ran command");
        assert_eq!(command_preview(&action).as_deref(), Some("ls -la"));
    }

    #[test]
    fn unknown_tools_get_a_safe_humanized_label() {
        let action = action("customer_api_searchRecords", json!({}));
        assert_eq!(action_label(&action), "Used Customer api search records");
        assert_eq!(raw_tool_name(&action), "customer_api_searchRecords");
    }
}
