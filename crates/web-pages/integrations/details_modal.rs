#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use integrations::{IntegrationTool, ToolScope};

#[component]
pub fn DetailsModal(integration: IntegrationTool, trigger_id: String) -> Element {
    rsx!(
        Modal {
            trigger_id: trigger_id,
            ModalBody {
                class: "flex flex-col justify-between md:w-full max-w-[64rem] h-full",
                TabContainer {
                    TabPanel {
                        checked: true,
                        name: "prompt-tabs",
                        tab_name: "Tools",
                        div {
                            class: "flex flex-col mt-5",
                            h3 { class: "text-lg font-bold", "{integration.title}" }

                            div {
                                class: "mt-4",
                                h4 { class: "font-semibold", "Scope" }
                                p {
                                    match integration.scope {
                                        ToolScope::UserSelectable => "User Selectable",
                                        ToolScope::DocumentIntelligence => "Document Intelligence",
                                    }
                                }
                            }

                            div {
                                class: "mt-4",
                                h4 { class: "font-semibold", "Tool Definitions" }

                                for (index, definition) in integration.definitions.iter().enumerate() {
                                    div {
                                        class: "mt-2 p-3 border rounded",
                                        h5 { class: "font-medium", "Tool #{index + 1}: {definition.function.name}" }

                                        if let Some(description) = &definition.function.description {
                                            p { class: "mt-1 text-sm", "{description}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    TabPanel {
                        name: "prompt-tabs",
                        tab_name: "Definition JSON",
                        div {
                            class: "mt-4 prose",
                            pre {
                                class: "json",
                                "{integration.definitions_json}"
                            }
                        }
                    }
                }

                ModalAction {
                    Button {
                        class: "cancel-modal",
                        button_type: ButtonType::Button,
                        button_scheme: ButtonScheme::Danger,
                        "Close"
                    }
                }
            }
        }
    )
}
