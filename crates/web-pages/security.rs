#![allow(non_snake_case)]
use crate::{
    app_layout::{Layout, SideBar},
    hero::Hero,
};
use assets::files::info_svg;
use daisy_rsx::{TabContainer, TabPanel};
use db::authz::Rbac;
use dioxus::prelude::*;

pub mod routes {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/security")]
    pub struct Index {
        pub team_id: i32,
    }
}

#[component]
pub fn SecurityPage(rbac: Rbac, team_id: i32) -> Element {
    rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Security,
            team_id: team_id,
            rbac: rbac,
            title: "Encryption and Security",
            header: rsx! {
                h3 { "Security and Encryption" }
            },

            Hero {
                heading: "Encryption and Security".to_string(),
                subheading: "Protecting Sensitive Data in Generative AI and RAG Through Encryption and Security.".to_string()
            }
            div {
                class: "mt-12 mx-auto max-w-3xl overflow-x-clip px-4",
                TabContainer {

                    TabPanel {
                        name: "security-tabs",
                        tab_name: "Encryption at Rest",
                        checked: true,
                        EncryptionTab {}
                    }

                    TabPanel {
                        name: "security-tabs",
                        tab_name: "Confidential Compute",
                        ConfidentialTab {}
                    }
                }
            }
        }
    }
}

#[component]
fn ConfidentialTab() -> Element {
    rsx! {

        h1 { class: "text-3xl font-bold mt-12 mb-12",
            "Encryption of Data in Use"
        }
    }
}

#[component]
fn EncryptionTab() -> Element {
    rsx! {

        h1 { class: "text-3xl font-bold mt-12 mb-12",
            "Encryption at Rest"
        }

        div { class: "alert alert-info mb-6",
            img {
                class: "svg-icon w-8 h-8",
                src: info_svg.name,
            }
            div {
                h3 { class: "font-bold",
                    "AES-256 and Encryption at Rest"
                }
                p { class: "text-sm",
                    "AES-256 is a strong encryption algorithm used for data protection. Encryption at rest ensures your data is secure when stored. These methods are crucial for maintaining data confidentiality and integrity."
                }
            }
        }

        div {
            class: "space-y-4",

            BigSelectorEnabled {
                title: "Not Encrypted",
                message: "No encryption is being applied to your data. This option provides no additional security for your information."
            }

            BigSelector {
                title: "Encrypted with Bionic Key",
                message: "Your data is encrypted using our advanced Bionic Keys. This option ensures GDPR compliance and provides robust protection for sensitive information."
            }

            BigSelector {
                title: "Using Customer Keys (BYOK)",
                message: "Bring Your Own Key (BYOK) for maximum control over your data encryption. Contact support for this advanced option."
            }
        }
    }
}

#[component]
fn BigSelectorEnabled(title: String, message: String) -> Element {
    rsx! {
        div { class: "form-control",
            label { class: "label cursor-pointer",
                input {
                    r#type: "radio",
                    name: "encryption-option",
                    class: "radio radio-primary",
                    value: "{title}",
                    checked: true
                }
                span { class: "label-text ml-4 flex-grow",
                    span { class: "font-bold",
                        "{title}"
                    }
                    br {}
                    "{message}"
                }
            }
        }
    }
}

#[component]
fn BigSelector(title: String, message: String) -> Element {
    rsx! {
        div { class: "form-control",
            label { class: "label cursor-not-allowed",
                input {
                    r#type: "radio",
                    name: "encryption-option",
                    class: "radio radio-primary",
                    value: "{title}",
                    disabled: true
                }
                span { class: "label-text ml-4 flex-grow opacity-50",
                    span { class: "font-bold",
                        "{title}"
                    }
                    br {}
                    "{message}"
                }
            }
        }
    }
}
