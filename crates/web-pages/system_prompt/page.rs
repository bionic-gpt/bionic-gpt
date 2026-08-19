#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::RuntimeSetting;
use dioxus::prelude::*;

pub fn page(team_id: String, rbac: Rbac, setting: RuntimeSetting) -> String {
    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::SystemPrompt,
            team_id: team_id.clone(),
            rbac,
            title: "System Prompt",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem { text: "System Prompt".into(), href: None }]
                }
            ),
            div {
                class: "p-4 max-w-4xl w-full mx-auto",
                form {
                    action: crate::routes::system_prompt::Update { team_id: team_id.clone() }.to_string(),
                    method: "post",
                    class: "space-y-6",
                    Card {
                        CardHeader { title: "Default runtime system prompt" }
                        CardBody {
                            class: "flex flex-col gap-4",
                            Fieldset {
                                legend: "Prompt text",
                                help_text: "This system prompt is sent before conversation history. Runtime skill and tool context is appended automatically.",
                                TextArea {
                                    class: "mt-3 w-full font-mono text-sm",
                                    name: "value",
                                    rows: "18",
                                    required: true,
                                    "{setting.value}"
                                }
                            }
                            div {
                                class: "rounded border border-base-300 bg-base-200 p-4 text-sm text-base-content/80",
                                p { class: "font-semibold", "Runtime additions" }
                                p { class: "mt-1", "Available skills and tool discovery instructions are generated from the current runtime state and appended after this prompt." }
                            }
                            div {
                                class: "text-xs text-base-content/60",
                                "Last updated {setting.updated_at}"
                            }
                        }
                    }
                    div {
                        class: "flex justify-end",
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Save System Prompt"
                        }
                    }
                }
            }
        }
    };
    crate::render(page)
}
