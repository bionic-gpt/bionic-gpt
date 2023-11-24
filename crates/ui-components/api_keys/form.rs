#![allow(non_snake_case)]
use db::Prompt;
use dioxus::prelude::*;
use primer_rsx::{select::SelectOption, *};

#[inline_props]
pub fn Form(cx: Scope, organisation_id: i32, prompts: Vec<Prompt>) -> Element {
    cx.render(rsx!(
        form {
            method: "post",
            action: "{crate::routes::api_keys::new_route(*organisation_id)}",
            Drawer {
                label: "New API Key",
                trigger_id: "create-api-key",
                DrawerBody {
                    div {
                        class: "flex flex-col",
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Production API Key",
                            help_text: "Give your new key a name",
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                        Select {
                            name: "prompt_id",
                            label: "Please select a prompt",
                            label_class: "mt-4",
                            help_text: "All access via this API key will use the above prompt",
                            prompts.iter().map(|prompt| rsx!(
                                SelectOption {
                                    value: "{prompt.id}",
                                    "{prompt.name}"
                                }
                            ))
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Create API Key"
                    }
                }
            }
        }
    ))
}
