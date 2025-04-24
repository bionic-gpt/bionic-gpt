#![allow(non_snake_case)]
use daisy_rsx::{select::SelectOption, *};
use dioxus::prelude::*;

#[component]
pub fn Form(
    id: Option<i32>,
    team_id: i32,
    name: String,
    trigger_id: String,
    integration_type: String,
) -> Element {
    rsx!(
        Drawer {
            submit_action: crate::routes::integrations::New{team_id}.to_string(),
            label: "Add an Integration",
            trigger_id: "{trigger_id}",
            DrawerBody {

                div {
                    class: "flex flex-col mt-3",
                    if let Some(id) = id {
                        input {
                            "type": "hidden",
                            value: "{id}",
                            name: "id"
                        }
                    }

                    Input {
                        input_type: InputType::Text,
                        label_class: "mt-4",
                        name: "name",
                        label: "Name",
                        help_text: "Make the name memorable and imply it's usage.",
                        value: name,
                        required: true
                    }

                    Select {
                        name: "integration_type",
                        label: "Is this model for MCP Server",
                        label_class: "mt-4",
                        help_text: "Curently we only support MCP Servers",
                        value: integration_type.clone(),
                        SelectOption {
                            value: "MCP_Server",
                            selected_value: integration_type.clone(),
                            "MCP Server"
                        }
                    }
                }
            }

            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Primary,
                    "Save"
                }
            }
        }
    )
}
