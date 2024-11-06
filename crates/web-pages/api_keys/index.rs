#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, ApiKey, Prompt};
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32, api_keys: Vec<ApiKey>, prompts: Vec<Prompt>) -> Element {
    rsx! {
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
            super::form::Form {
                team_id: team_id,
                prompts: prompts.clone()
            },

            if ! api_keys.is_empty() {

                Box {
                    class: "has-data-table",
                    BoxHeader {
                        title: "API Keys"
                    }
                    BoxBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "Name" }
                                th { "API Key" }
                                th { "Prompt" }
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

            CodeExamples {

            }
        }
    }
}

#[component]
fn OpenAICompatibility() -> Element {
    rsx! {
        // OpenAI API Compatibility Card
        Box {
            class: "mt-8 mb-8",
            BoxBody {
                h2 { class: "card-title", "OpenAI API Compatibility" }
                p { "Our API keys are fully compatible with the OpenAI API, allowing seamless integration with existing projects and tools." }
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
        Box {
            BoxBody {
                h2 { class: "card-title", "API Usage Examples" }
                div { class: "mockup-code mt-4",
                    pre {
                        code {
                            "// Example: Using the Assistant API
const response = await fetch('https://api.example.com/v1/assistants', {{
method: 'POST',
headers: {{
'Authorization': 'Bearer YOUR_ASSISTANT_KEY',
'Content-Type': 'application/json'
}},
body: JSON.stringify({{
model: 'gpt-3.5-turbo',
messages: [{{ role: 'user', content: 'Hello, how are you?' }}]
}})
}});

const data = await response.json();
console.log(data.choices[0].message.content);"
                        }
                    }
                }
                div { class: "mockup-code mt-4",
                    pre {
                        code {
                            "// Example: Using the Model API
const response = await fetch('https://api.example.com/v1/completions', {{
method: 'POST',
headers: {{
'Authorization': 'Bearer YOUR_MODEL_KEY',
'Content-Type': 'application/json'
}},
body: JSON.stringify({{
model: 'text-davinci-003',
prompt: 'Translate the following English text to French: \"Hello, world!\"',
max_tokens: 60
}})
}});

const data = await response.json();
console.log(data.choices[0].text);"
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
            Box {
                BoxBody {
                    h2 {
                        class: "card-title",
                        "Assistant Key"
                    }
                    p { "Choose this option if you want to use our AI assistants with predefined capabilities." }
                    ul { class: "list-disc list-inside mt-4",
                        li { "Access to pre-configured AI assistants" }
                        li { "Simplified integration process" }
                        li { "Ideal for specific use-cases" }
                    }
                    div { class: "card-actions justify-end mt-4",
                        Button {
                            prefix_image_src: "{button_plus_svg.name}",
                            drawer_trigger: "create-api-key",
                            "Create an Assistant Key"
                        }
                    }
                }
            }

            // Model Key Card
            Box {
                BoxBody {
                    h2 { class: "card-title", "Model Key" }
                    p { "Choose this option if you want direct access to our AI models for custom implementations." }
                    ul { class: "list-disc list-inside mt-4",
                        li { "Full control over AI model parameters" }
                        li { "Flexibility for advanced use-cases" }
                        li { "Suitable for experienced developers" }
                    }
                    div { class: "card-actions justify-end mt-4",
                        Button {
                            prefix_image_src: "{button_plus_svg.name}",
                            drawer_trigger: "create-api-key",
                            "Create a Model Key"
                        }
                    }
                }
            }
        }
    }
}
