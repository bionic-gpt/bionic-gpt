#![allow(non_snake_case)]

use assets::files::*;
use daisy_rsx::*;
use dioxus::prelude::*;
use tool_runtime::ToolCall;

fn format_json_string(raw: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) {
        serde_json::to_string_pretty(&value).unwrap_or_else(|_| raw.to_string())
    } else {
        raw.to_string()
    }
}

fn display_tool_name(tool_call_id: &Option<String>, tool_call: Option<&ToolCall>) -> String {
    if let Some(call) = tool_call {
        if !call.function.name.is_empty() {
            return call.function.name.clone();
        }
    }
    format!("Tool Call {}", tool_call_id.as_deref().unwrap_or("Unknown"))
}

#[component]
pub fn ToolCallTimeline(
    chat_id: i64,
    pending: bool,
    tool_call_id: Option<String>,
    tool_call: Option<ToolCall>,
    response: Option<String>,
) -> Element {
    let display_name = display_tool_name(&tool_call_id, tool_call.as_ref());
    let modal_tool_name = display_name.clone();
    let trigger_suffix = tool_call_id
        .clone()
        .filter(|id| !id.is_empty())
        .unwrap_or_else(|| chat_id.to_string());
    let trigger_id = format!("tool-call-details-{}", trigger_suffix);
    let request_body = tool_call
        .as_ref()
        .map(|call| format_json_string(&call.function.arguments));
    let response_body = response
        .as_ref()
        .map(|body| format_json_string(body))
        .filter(|body| !body.trim().is_empty());

    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: if pending { spinner_svg.name } else { tools_svg.name }
            }
            TimeLineBody {
                div {
                    class: "flex items-center gap-2",
                    Badge {
                        badge_style: BadgeStyle::Outline,
                        badge_size: BadgeSize::Sm,
                        "Tool Call:"
                        strong {
                            class: "ml-2",
                            "{display_name}"
                        }
                    }
                    Button {
                        class: "btn-xs",
                        button_style: ButtonStyle::Outline,
                        button_shape: ButtonShape::Circle,
                        popover_target: trigger_id.clone(),
                        button_scheme: ButtonScheme::Neutral,
                        img {
                            class: "svg-icon",
                            src: button_plus_svg.name
                        }
                        span {
                            class: "sr-only",
                            "View tool call details"
                        }
                    }
                }
            }
        }
        Modal {
            trigger_id: trigger_id.clone(),
            ModalBody {
                h3 {
                    class: "font-bold text-lg mb-4",
                    "Tool Call Details"
                }
                dl {
                    class: "space-y-4",
                    if let Some(call) = tool_call.as_ref() {
                        div {
                            class: "space-y-2",
                            dt { class: "font-semibold text-sm uppercase text-base-content/70", "Tool" }
                            dd { class: "text-sm break-words", "{modal_tool_name}" }
                        }
                        div {
                            class: "space-y-2",
                            dt { class: "font-semibold text-sm uppercase text-base-content/70", "Call ID" }
                            dd { class: "text-sm break-all", "{call.id}" }
                        }
                    } else if let Some(id) = tool_call_id.clone() {
                        div {
                            class: "space-y-2",
                            dt { class: "font-semibold text-sm uppercase text-base-content/70", "Call ID" }
                            dd { class: "text-sm break-all", "{id}" }
                        }
                    }
                    div {
                        class: "space-y-2",
                        dt { class: "font-semibold text-sm uppercase text-base-content/70", "Request" }
                        if let Some(body) = request_body.as_ref() {
                            pre {
                                class: "json bg-base-200 p-4 rounded overflow-auto max-h-96 text-sm",
                                "{body}"
                            }
                        } else {
                            dd {
                                class: "text-sm text-base-content/70",
                                "No request payload available."
                            }
                        }
                    }
                    div {
                        class: "space-y-2",
                        dt { class: "font-semibold text-sm uppercase text-base-content/70", "Response" }
                        if let Some(body) = response_body.as_ref() {
                            pre {
                                class: "json bg-base-200 p-4 rounded overflow-auto max-h-96 text-sm",
                                "{body}"
                            }
                        } else if pending {
                            dd {
                                class: "text-sm text-base-content/70",
                                "Awaiting tool response..."
                            }
                        } else {
                            dd {
                                class: "text-sm text-base-content/70",
                                "No response recorded."
                            }
                        }
                    }
                }
                ModalAction {
                    Button {
                        class: "cancel-modal",
                        button_scheme: ButtonScheme::Warning,
                        "Close"
                    }
                }
            }
        }
    }
}
