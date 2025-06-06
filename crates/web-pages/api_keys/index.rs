#![allow(non_snake_case)]
use crate::{
    app_layout::{Layout, SideBar},
    render, ConfirmModal,
};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, ApiKey, Prompt, PromptType as DBPromptType};
use dioxus::prelude::*;

pub fn page(
    rbac: Rbac,
    team_id: i32,
    api_keys: Vec<ApiKey>,
    assistants: Vec<Prompt>,
    models: Vec<Prompt>,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::ApiKeys,
            team_id: team_id,
            rbac: rbac,
            title: "API Keys",
            header: rsx! {
                h3 { "API Keys" }
            },
            if api_keys.is_empty() {
                BlankSlate {
                    heading: "Looks like you don't have any API keys",
                    visual: empty_api_keys_svg.name,
                    description: "API Keys allow you to access our programming interface",
                }
            },

            for item in &api_keys {
                ConfirmModal {
                    action: crate::routes::api_keys::Delete {team_id, id: item.id}.to_string(),
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id),
                    submit_label: "Delete".to_string(),
                    heading: "Delete this API Key?".to_string(),
                    warning: "Are you sure you want to delete this api key?".to_string(),
                    hidden_fields: vec![
                        ("team_id".into(), team_id.to_string()),
                        ("id".into(), item.id.to_string()),
                    ],
                }
            }

            super::form::AssistantForm {
                team_id: team_id,
                prompts: assistants.clone()
            },
            super::form::ModelForm {
                team_id: team_id,
                prompts: models.clone()
            },

            if ! api_keys.is_empty() {

                Card {
                    class: "has-data-table",
                    CardHeader {
                        title: "API Keys"
                    }
                    CardBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "Name" }
                                th { "Type" }
                                th { "API Key" }
                                th { "Assistant/Model" }
                                th {
                                    class: "text-right",
                                    "Action"
                                }
                            }
                            tbody {
                                for key in &api_keys {
                                    tr {
                                        td {
                                            "{key.name}"
                                        }
                                        td {
                                            PromptType {
                                                prompt_type: key.prompt_type
                                            }
                                        }
                                        td {
                                            div {
                                                class: "flex w-full",
                                                Input {
                                                    value: key.api_key.clone(),
                                                    name: "api_key",
                                                    input_type: InputType::Password
                                                }
                                                Button {
                                                    class: "api-keys-toggle-visibility",
                                                    "Show"
                                                }
                                            }
                                        }
                                        td {
                                            "{key.prompt_name}"
                                        }
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    drawer_trigger: format!("delete-trigger-{}-{}",
                                                        key.id, team_id),
                                                    href: "#",
                                                    target: "_top",
                                                    "Delete"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            KeySelector {

            }

            OpenAICompatibility {

            }

            /***CodeExamples {

            }**/
        }
    };

    render(page)
}

#[component]
pub fn PromptType(prompt_type: DBPromptType) -> Element {
    match prompt_type {
        DBPromptType::Model => rsx!(
            Label {
                class: "mr-2 truncate",
                label_role: LabelRole::Info,
                "Model"
            }
        ),
        DBPromptType::Assistant => rsx!(
            Label {
                class: "mr-2 truncate",
                label_role: LabelRole::Highlight,
                "Assistant"
            }
        ),
    }
}

#[component]
fn OpenAICompatibility() -> Element {
    rsx! {
        // OpenAI API Compatibility Card
        Card {
            class: "mt-8 mb-8",
            CardHeader {
                title: "OpenAI API"
            }
            CardBody {
                class: "p-5",
                p { "Our API is compatible with the OpenAI completions API, allowing seamless integration with existing projects and tools." }
                ul { class: "list-disc list-inside mt-4",
                    li { "Use the same endpoints and parameters as OpenAI" }
                    li { "Easy migration from OpenAI to our service" }
                    li { "Access to similar models and capabilities" }
                }
            }
        }
    }
}

#[component]
fn CodeExamples() -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "API Usage Example"
            }
            CardBody {
                class: "p-5",
                p {
                    ""
                }
                div { class: "mt-4",
                    pre {
                        code {
                            "// Example: Using the Assistant API
const response = await fetch('https://app.bionic-gpt.com/v1/chat/completions', {{
    method: 'POST',
    headers: {{
        'Authorization': 'Bearer YOUR_ASSISTANT_KEY',
        'Content-Type': 'application/json'
    }},
    body: JSON.stringify({{
        model: 'assistant',
            messages: [{{ role: 'user', content: 'Hello, how are you?' }}]
    }})
}});

const data = await response.json();
console.log(data.choices[0].message.content);"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn KeySelector() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 gap-8 mb-8 mt-8",
            // Assistant Key Card
            Card {
                CardHeader {
                    title: "Assistant Key"
                }
                CardBody {
                    class: "p-5",
                    p { "Turn any of your assistants into an API" }
                    ul { class: "list-disc list-inside mt-4",
                        li { "Access to pre-configured AI assistants" }
                        li { "Simplified integration process" }
                        li { "Ideal for specific use-cases" }
                    }
                    div { class: "card-actions justify-end mt-4",
                        Button {
                            drawer_trigger: "create-assistant-key",
                            "Create an Assistant Key"
                        }
                    }
                }
            }

            // Model Key Card
            Card {
                CardHeader {
                    title: "Model Key"
                }
                CardBody {
                    class: "p-5",
                    p { "Use existing models for your own projects" }
                    ul { class: "list-disc list-inside mt-4",
                        li { "Full control over AI model parameters" }
                        li { "Flexibility for advanced use-cases" }
                        li { "Limits will be applied to ensure fair use" }
                    }
                    div { class: "card-actions justify-end mt-4",
                        Button {
                            drawer_trigger: "create-model-key",
                            "Create a Model Key"
                        }
                    }
                }
            }
        }
    }
}
