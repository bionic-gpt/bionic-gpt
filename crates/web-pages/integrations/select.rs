#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::i18n;
use crate::routes;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrebuiltSpec {
    pub file_name: String,
    pub title: String,
    pub description: Option<String>,
    pub spec_json: String,
    pub logo_data_url: Option<String>,
}

pub fn page(team_id: i32, rbac: Rbac, specs: Vec<PrebuiltSpec>) -> String {
    let private_visibility = crate::visibility_to_string(db::Visibility::Private);
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Integrations,
            team_id: team_id,
            rbac: rbac.clone(),
            title: crate::i18n::integrations().to_string(),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: crate::i18n::integrations().into(),
                            href: Some(routes::integrations::Index { team_id }.to_string()),
                        },
                        BreadcrumbItem {
                            text: "Select Integration".into(),
                            href: None,
                        }
                    ]
                }
                Button {
                    button_type: ButtonType::Link,
                    button_scheme: ButtonScheme::Primary,
                    href: routes::integrations::New { team_id }.to_string(),
                    "Add Custom"
                }
            ),

            Card {
                CardHeader {
                    title: format!("Select a {}", i18n::integration())
                }
                CardBody {
                    if specs.is_empty() {
                        div {
                            class: "alert alert-warning",
                            "No pre-built integrations are available right now."
                        }
                    } else {
                        div {
                            class: "grid grid-cols-1 gap-4 md:grid-cols-2",
                            for spec in specs {
                                Card {
                                    class: "bg-base-100 shadow border border-base-300 h-full flex flex-col",
                                    CardHeader {
                                        title: spec.title.clone()
                                    }
                                    CardBody {
                                        class: "flex-1 flex flex-col gap-4",
                                        if let Some(logo_url) = spec.logo_data_url.clone() {
                                            div {
                                                class: "flex justify-center",
                                                img {
                                                    class: "h-16 w-auto object-contain",
                                                    src: "{logo_url}",
                                                    alt: format!("{} logo", spec.title),
                                                }
                                            }
                                        }
                                        if let Some(description) = spec.description.clone() {
                                            p {
                                                class: "text-sm text-base-content/80",
                                                "{description}"
                                            }
                                        }
                                        p {
                                            class: "text-xs text-base-content/60",
                                            "Source: {spec.file_name}.json"
                                        }
                                        div {
                                            class: "mt-auto",
                                            form {
                                                method: "post",
                                                action: routes::integrations::New { team_id }.to_string(),
                                                input {
                                                    r#type: "hidden",
                                                    name: "visibility",
                                                    value: private_visibility.clone(),
                                                }
                                                textarea {
                                                    class: "hidden",
                                                    name: "openapi_spec",
                                                    "{spec.spec_json}"
                                                }
                                                Button {
                                                    button_type: ButtonType::Submit,
                                                    button_scheme: ButtonScheme::Primary,
                                                    "Create Integration"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
