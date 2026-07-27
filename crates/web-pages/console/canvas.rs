#![allow(non_snake_case)]

use daisy_rsx::*;
use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct GeneratedOutputPayload {
    pub id: i32,
    pub path: String,
    pub file_name: String,
    pub mime_type: String,
    pub size: i64,
}

#[derive(Debug, Deserialize)]
struct ToolOutputPayload {
    outputs: Option<Vec<GeneratedOutputPayload>>,
}

pub fn parse_generated_outputs(raw: Option<&str>) -> Vec<GeneratedOutputPayload> {
    serde_json::from_str::<ToolOutputPayload>(raw.unwrap_or_default())
        .ok()
        .and_then(|payload| payload.outputs)
        .unwrap_or_default()
        .into_iter()
        .filter(|output| !output.path.trim().is_empty())
        .collect()
}

pub fn is_canvas_output(output: &GeneratedOutputPayload) -> bool {
    output.file_name == "CANVAS.md" && output.path.ends_with("/CANVAS.md")
}

fn canvas_title(path: &str) -> String {
    path.strip_suffix("/CANVAS.md")
        .and_then(|path| path.rsplit('/').next())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("Canvas")
        .replace(['-', '_'], " ")
}

#[component]
pub fn CanvasOutput(team_id: String, output: GeneratedOutputPayload) -> Element {
    let title = canvas_title(&output.path);
    let src = crate::routes::console::GeneratedOutputCanvas {
        team_id,
        id: output.id,
    }
    .to_string();

    rsx! {
        div {
            class: "border border-base-300 bg-base-100 rounded-lg overflow-hidden",
            div {
                class: "px-3 py-2 border-b border-base-300 text-sm font-semibold capitalize",
                "{title}"
            }
            iframe {
                class: "w-full h-[45vh] min-h-[720px] bg-white",
                title: "{title}",
                "sandbox": "",
                src: "{src}"
            }
        }
    }
}

#[component]
pub fn GeneratedFiles(outputs: Vec<GeneratedOutputPayload>) -> Element {
    let files = outputs
        .into_iter()
        .filter(|output| !is_canvas_output(output))
        .collect::<Vec<_>>();

    rsx! {
        if !files.is_empty() {
            div {
                class: "rounded-lg border border-base-300 bg-base-100 p-3",
                div {
                    class: "text-sm font-semibold",
                    "Generated files"
                }
                ul {
                    class: "mt-2 space-y-2",
                    for output in files.iter() {
                        li {
                            class: "flex flex-wrap items-center gap-2 text-sm",
                            Badge {
                                badge_style: BadgeStyle::Outline,
                                badge_size: BadgeSize::Sm,
                                "{output.file_name}"
                            }
                            span {
                                class: "font-mono text-xs text-base-content/70 break-all",
                                "{output.path}"
                            }
                            span {
                                class: "text-xs text-base-content/60",
                                "{output.mime_type} - {output.size} bytes"
                            }
                        }
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
    fn test_parse_generated_outputs() {
        let payload = r#"{"stdout":"","outputs":[{"id":1,"path":"/home/user/output/report.html","file_name":"report.html","mime_type":"text/html","size":42}]}"#;
        let outputs = parse_generated_outputs(Some(payload));

        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].path, "/home/user/output/report.html");
        assert_eq!(outputs[0].file_name, "report.html");
    }

    #[test]
    fn test_canvas_output_detection() {
        let output = GeneratedOutputPayload {
            id: 1,
            path: "/home/user/output/demo/CANVAS.md".to_string(),
            file_name: "CANVAS.md".to_string(),
            mime_type: "text/markdown".to_string(),
            size: 42,
        };

        assert!(is_canvas_output(&output));
    }
}
