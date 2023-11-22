#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct DrawerProps {
    organisation_id: i32,
    email: String,
    user_id: i32,
    trigger_id: String,
}

pub fn RemoveMemberDrawer(cx: Scope<DrawerProps>) -> Element {
    cx.render(rsx! {
        Drawer {
            submit_action: crate::routes::team::delete_route(cx.props.organisation_id),
            label: "Remove this user?",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        h4 {
                            "Are you sure you want to remove '{cx.props.email}' from the team?"
                        }
                    }
                    input {
                        "type": "hidden",
                        "name": "organisation_id",
                        "value": "{cx.props.organisation_id}"
                    }
                    input {
                        "type": "hidden",
                        "name": "user_id",
                        "value": "{cx.props.user_id}"
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
