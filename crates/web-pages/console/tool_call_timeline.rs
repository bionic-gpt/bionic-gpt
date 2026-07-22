#![allow(non_snake_case)]

use assets::files::*;
use daisy_rsx::*;
use dioxus::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use tool_runtime::ToolCall;

#[derive(Debug, Deserialize)]
struct HtmlCanvasPayload {
    #[serde(rename = "type")]
    artifact_type: String,
    version: u32,
    title: Option<String>,
    html: String,
}

fn format_json_string(raw: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) {
        serde_json::to_string_pretty(&value).unwrap_or_else(|_| raw.to_string())
    } else {
        raw.to_string()
    }
}

fn format_json_value(raw: &Value) -> String {
    serde_json::to_string_pretty(raw).unwrap_or_else(|_| raw.to_string())
}

fn display_tool_name(tool_call_id: &Option<String>, tool_call: Option<&ToolCall>) -> String {
    if let Some(call) = tool_call {
        if !call.function.name.is_empty() {
            return call.function.name.clone();
        }
    }
    format!("Tool Call {}", tool_call_id.as_deref().unwrap_or("Unknown"))
}

fn parse_html_canvas(raw: Option<&str>) -> Option<HtmlCanvasPayload> {
    let payload = serde_json::from_str::<HtmlCanvasPayload>(raw?).ok()?;
    if payload.artifact_type == "html_canvas"
        && payload.version == 1
        && !payload.html.trim().is_empty()
    {
        Some(payload)
    } else {
        None
    }
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
        .map(|call| format_json_value(&call.function.arguments));
    let response_body = response
        .as_ref()
        .map(|body| format_json_string(body))
        .filter(|body| !body.trim().is_empty());
    let html_canvas = parse_html_canvas(response.as_deref());

    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: if pending { spinner_svg.name } else { tools_svg.name }
            }
            TimeLineBody {
                div {
                    class: "space-y-3",
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
                    if let Some(canvas) = html_canvas.as_ref() {
                        div {
                            class: "border border-base-300 bg-base-100 rounded-lg overflow-hidden",
                            if let Some(title) = canvas.title.as_ref() {
                                div {
                                    class: "px-3 py-2 border-b border-base-300 text-sm font-semibold",
                                    "{title}"
                                }
                            }
                            iframe {
                                class: "w-full h-96 bg-white",
                                title: canvas.title.as_deref().unwrap_or("HTML canvas"),
                                "sandbox": "",
                                srcdoc: "{canvas.html}"
                            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_canvas() {
        let payload =
            r#"{"type":"html_canvas","version":1,"title":"Preview","html":"<h1>Hello</h1>"}"#;
        let canvas = parse_html_canvas(Some(payload)).expect("expected html canvas");

        assert_eq!(canvas.title.as_deref(), Some("Preview"));
        assert_eq!(canvas.html, "<h1>Hello</h1>");
    }

    #[test]
    fn test_parse_html_canvas_rejects_normal_json() {
        assert!(parse_html_canvas(Some(r#"{"result":"ok"}"#)).is_none());
    }

    #[test]
    fn test_parse_html_canvas_rejects_empty_html() {
        assert!(
            parse_html_canvas(Some(r#"{"type":"html_canvas","version":1,"html":"   "}"#)).is_none()
        );
    }
}
