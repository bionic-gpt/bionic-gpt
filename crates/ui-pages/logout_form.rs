#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn LogoutForm(cx: Scope) -> Element {
    cx.render(rsx! {
        form {
            method: "get",
            "data-turbo": "false",
            action: "/oauth2/sign_out",
            input {
                "type": "hidden",
                name: "rd",
                value: "http://localhost:7710/realms/bionic-gpt/protocol/openid-connect/logout"
            }
            Drawer {
                label: "Logout ?",
                trigger_id: "logout-drawer",
                DrawerBody {
                    p {
                        class: "mb-4",
                        "Are you sure you want to log out?"
                    }
                    Alert {
                        alert_color: AlertColor::Warn,
                        "During logout we delete all cookies associated with your account
                        and any private keys stored in local storage."
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Danger,
                        "Logout"
                    }
                }
            }
        }
    })
}
