#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn RemoveMemberDrawer(
    cx: Scope,
    team_id: i32,
    email: String,
    user_id: i32,
    trigger_id: String,
) -> Element {
    cx.render(rsx! {
        Drawer {
            submit_action: crate::routes::team::delete_route(*team_id),
            label: "Remove this user?",
            trigger_id: &trigger_id,
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        h4 {
                            "Are you sure you want to remove '{email}' from the team?"
                        }
                    }
                    input {
                        "type": "hidden",
                        "name": "team_id",
                        "value": "{*team_id}"
                    }
                    input {
                        "type": "hidden",
                        "name": "user_id",
                        "value": "{*user_id}"
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Danger,
                    "Remove User"
                }
            }
        }
    })
}
