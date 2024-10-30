#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use dioxus::prelude::*;

pub mod routes {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/:team_id/security")]
    pub struct Index {
        pub team_id: i32,
    }
}

#[component]
pub fn SecurityPage(rbac: Rbac, team_id: i32) -> Element {
    rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::ApiKeys,
            team_id: team_id,
            rbac: rbac,
            title: "Encryption and Security",
            header: rsx! {
                h3 { "Encryption and Security" }
            },
            div { class: "container mx-auto p-4",
                h1 { class: "text-3xl font-bold mb-6",
                    "Choose Your Encryption Option"
                }

                div { class: "alert alert-info mb-6",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        class: "stroke-current shrink-0 w-6 h-6",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
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

                div { class: "space-y-4",
                    // Not Encrypted
                    div { class: "form-control",
                        label { class: "label cursor-pointer",
                            input {
                                r#type: "radio",
                                name: "encryption-option",
                                class: "radio radio-primary",
                                value: "Not Encrypted",
                                checked: true,
                            }
                            span { class: "label-text ml-4 flex-grow",
                                span { class: "font-bold",
                                    "Not Encrypted"
                                }
                                br {}
                                "No encryption is being applied to your data. This option provides no additional security for your information."
                            }
                        }
                    }

                    // Encrypted with Bionic Keys
                    div { class: "form-control",
                        label { class: "label cursor-not-allowed",
                            input {
                                r#type: "radio",
                                name: "encryption-option",
                                class: "radio radio-primary",
                                value: "Encrypted with Bionic Keys",
                                disabled: true
                            }
                            span { class: "label-text ml-4 flex-grow opacity-50",
                                span { class: "font-bold",
                                    "Encrypted with Bionic Keys"
                                }
                                br {}
                                "Your data is encrypted using our advanced Bionic Keys. This option ensures GDPR compliance and provides robust protection for sensitive information."
                            }
                        }
                    }

                    // Using Customer Keys
                    div { class: "form-control",
                        label { class: "label cursor-not-allowed",
                            input {
                                r#type: "radio",
                                name: "encryption-option",
                                class: "radio radio-primary",
                                value: "Using Customer Keys",
                                disabled: true
                            }
                            span { class: "label-text ml-4 flex-grow opacity-50",
                                span { class: "font-bold",
                                    "Using Customer Keys (BYOK)"
                                }
                                br {}
                                "Bring Your Own Key (BYOK) for maximum control over your data encryption. Contact support for this advanced option."
                            }
                        }
                    }
                }
            }
        }
    }
}
