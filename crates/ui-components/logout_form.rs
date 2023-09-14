#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

pub fn LogoutForm(cx: Scope) -> Element {
    cx.render(rsx! {
        form {
            method: "post",
            "data-turbo": "false",
            action: "/auth/sign_out",
            Drawer {
                label: "Logout ?",
                trigger_id: "logout-drawer",
                DrawerBody {
                    p {
                        "Are you sure you want to log out?"
                    }
                    Alert {
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
