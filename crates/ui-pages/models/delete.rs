#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[inline_props]
pub fn DeleteDrawer(cx: Scope<DrawerProps>, team_id: i32, id: i32, trigger_id: String) -> Element {
    cx.render(rsx! {
        Drawer {
            submit_action: crate::routes::models::delete_route(*team_id, *id),
            label: "Delete this Model?",
            trigger_id: trigger_id,
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        p {
                            "Are you sure you want to delete this Model?"
                        }
                    }
                    input {
                        "type": "hidden",
                        "name": "team_id",
                        "value": "{*team_id}"
                    }
                    input {
                        "type": "hidden",
                        "name": "id",
                        "value": "{*id}"
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
    })
}
