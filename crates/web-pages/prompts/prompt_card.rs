#![allow(non_snake_case)]
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

#[component]
pub fn PromptCard(team_id: i32, rbac: Rbac, prompt: Prompt) -> Element {
    rsx! {
        Box {
            class: "cursor-pointer hover:bg-base-200",
            drawer_trigger: format!("view-trigger-{}-{}", prompt.id, team_id),
            BoxHeader {
                class: "truncate ellipses flex justify-between",
                title: "{prompt.name}",
                super::visibility::VisLabel {
                    visibility: prompt.visibility
                }
            }
            BoxBody {
                p {
                    class: "text-sm",
                    "{prompt.description}"
                }
                div {
                    class: "mt-3 text-xs flex justify-center gap-1",
                    "Last update",
                    RelativeTime {
                        format: RelativeTimeFormat::Relative,
                        datetime: "{prompt.updated_at}"
                    }
                }
            }
        }
    }
}
