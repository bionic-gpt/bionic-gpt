#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn DeleteDrawer(
    team_id: i32,
    conversation_id: i64,
    prompt_id: i32,
    trigger_id: String,
) -> Element {
    rsx! {
        Drawer {
            submit_action: crate::routes::prompts::DeleteConv{team_id, prompt_id, conversation_id}.to_string(),
            label: "Delete this Conversation?",
            trigger_id: trigger_id,
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        p {
                            "Are you sure you want to delete this Conversation?"
                        }
                    }
                    input {
                        "type": "hidden",
                        "name": "team_id",
                        "value": "{team_id}"
                    }
                    input {
                        "type": "hidden",
                        "name": "id",
                        "value": "{conversation_id}"
                    }
                    input {
                        "type": "hidden",
                        "name": "prompt_id",
                        "value": "{prompt_id}"
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Danger,
                    "Delete"
                }
            }
        }
    }
}
