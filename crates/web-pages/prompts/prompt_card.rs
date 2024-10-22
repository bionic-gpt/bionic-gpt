#![allow(non_snake_case)]
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

#[component]
pub fn PromptCard(team_id: i32, rbac: Rbac, prompt: Prompt) -> Element {
    rsx! {
        Box {
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
                    class: "mt-3 flex flex-row justify-between",
                    a {
                        class: "btn btn-primary btn-sm",
                        href: crate::routes::prompts::NewChat{team_id, prompt_id: prompt.id}.to_string(),
                        "Chat"
                    }
                    if rbac.can_edit_prompt(&prompt) {
                        div {
                            class: "flex gap-1",
                            Button {
                                drawer_trigger: format!("delete-trigger-{}-{}", prompt.id, team_id),
                                button_scheme: ButtonScheme::Danger,
                                "Delete"
                            }
                            Button {
                                modal_trigger: format!("edit-prompt-form-{}", prompt.id),
                                "Edit"
                            }
                        }
                    }
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
