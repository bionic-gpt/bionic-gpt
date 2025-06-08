#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn LogoutForm() -> Element {
    let logout_url = std::env::var("LOGOUT_URL").unwrap_or(
        "http://keycloak:7810/realms/bionic-gpt/protocol/openid-connect/logout".to_string(),
    );
    let signout_url = std::env::var("SIGNOUT_URL").unwrap_or("/oauth2/sign_out".to_string());
    rsx! {
        form {
            method: "post",
            "data-turbo": "false",
            action: signout_url,
            input {
                "type": "hidden",
                name: "rd",
                value: "{logout_url}"
            }
            Modal {
                trigger_id: "logout-trigger",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Logout ?"
                    }
                    p {
                        class: "mb-4",
                        "Are you sure you want to log out?"
                    }
                    Alert {
                        alert_color: AlertColor::Warn,
                        "During logout we delete all cookies associated with your account
                        and any private keys stored in local storage."
                    }
                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Error,
                            "Logout"
                        }
                    }
                }
            }
        }
    }
}
