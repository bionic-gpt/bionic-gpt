#![allow(non_snake_case)]

use assets::files::*;
use daisy_rsx::*;
use dioxus::prelude::*;
use tool_runtime::Reasoning;

fn display_reasoning(reasoning: &[Reasoning]) -> String {
    reasoning
        .iter()
        .map(Reasoning::display_text)
        .filter(|text| !text.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[component]
pub fn ReasoningTimeline(reasoning: Vec<Reasoning>) -> Element {
    let display_text = display_reasoning(&reasoning);

    if display_text.trim().is_empty() {
        return rsx! {};
    }

    rsx! {
        TimeLine {
            TimeLineBadge {
                image_src: ai_svg.name
            }
            TimeLineBody {
                details {
                    class: "collapse collapse-arrow bg-base-200/60 border border-base-300",
                    summary {
                        class: "collapse-title min-h-0 py-3 text-sm font-semibold",
                        "Thinking"
                    }
                    div {
                        class: "collapse-content",
                        pre {
                            class: "whitespace-pre-wrap break-words text-sm leading-relaxed m-0",
                            "{display_text}"
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::display_reasoning;
    use tool_runtime::Reasoning;

    #[test]
    fn display_reasoning_joins_visible_reasoning_blocks() {
        let reasoning = vec![
            Reasoning::summaries(vec!["Checked the request.".to_string()]),
            Reasoning::encrypted("opaque"),
            Reasoning::new("Prepared the answer."),
        ];

        assert_eq!(
            display_reasoning(&reasoning),
            "Checked the request.\n\nPrepared the answer."
        );
    }
}
