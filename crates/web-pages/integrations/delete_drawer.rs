#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn DeleteDrawer(team_id: i32, id: i32, trigger_id: String) -> Element {
    rsx! {
        Drawer {
            submit_action: crate::routes::integrations::Delete{team_id, id}.to_string(),
            label: "Delete this Integration?",
            trigger_id: trigger_id,
            DrawerBody {
                div {
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        p {
                            "Are you sure you want to delete this Integration?"

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
                        "value": "{id}"
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
