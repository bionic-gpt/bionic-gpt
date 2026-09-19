#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
use crate::routes;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::OpenapiSpec;
use dioxus::prelude::*;

pub fn page(
    team_id: String,
    rbac: Rbac,
    specs: Vec<OpenapiSpec>,
    upload_error: Option<String>,
) -> String {
    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::OpenapiSpecs,
            team_id: team_id.clone(),
            title: "APIs",
            rbac: rbac.clone(),
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "APIs".into(),
                        href: Some(routes::openapi_specs::Index { team_id: team_id.clone() }.to_string()),
                    }]
                }
                if rbac.is_sys_admin {
                    div {
                        class: "ml-auto flex flex-wrap items-center justify-end gap-2",
                        Button {
                            popover_target: "upload-openapi-specs",
                            button_scheme: ButtonScheme::Neutral,
                            "Upload Specs"
                        }
                        Button {
                            button_type: ButtonType::Link,
                            prefix_image_src: "{button_plus_svg.name}",
                            button_scheme: ButtonScheme::Primary,
                            href: routes::openapi_specs::New { team_id: team_id.clone() }.to_string(),
                            "Add OpenAPI Spec"
                        }
                    }
                }
            ),
            div {
                class: "p-4 max-w-5xl w-full mx-auto flex flex-col gap-6",
                SectionIntroduction {
                    header: "APIs".to_string(),
                    subtitle: "Manage the prebuilt OpenAPI specifications available to teams.".to_string(),
                    is_empty: specs.is_empty(),
                    empty_text: "No OpenAPI specs available yet. Add one to get started.".to_string(),
                }

                if let Some(error) = upload_error {
                    Alert {
                        alert_color: AlertColor::Error,
                        class: "whitespace-pre-line",
                        "{error}"
                    }
                }

                p {
                    class: "text-sm text-base-content/70",
                    "System integrations are enabled automatically for every team and cannot be selected as team integrations."
                }

                if !specs.is_empty() {
                    Card {
                        class: "has-data-table",
                        CardHeader { title: "Available Specs" }
                        CardBody {
                            table {
                                class: "table table-sm",
                                thead {
                                    tr {
                                        th { "Title" }
                                        th { "Slug" }
                                        th { "Category" }
                                        th { "Status" }
                                        th { "Updated" }
                                        th { class: "text-right", "Actions" }
                                    }
                                }
                                tbody {
                                    for spec in &specs {
                                        tr {
                                            td { "{spec.title}" }
                                            td {
                                                code { "{spec.slug}" }
                                            }
                                            td {
                                                "{crate::openapi_specs::category_label(spec.category)}"
                                            }
                                            td {
                                                span {
                                                    class: if spec.is_active {
                                                        "badge badge-success badge-outline"
                                                    } else {
                                                        "badge badge-ghost"
                                                    },
                                                    {if spec.is_active { "Active" } else { "Inactive" }}
                                                }
                                            }
                                            td {
                                                class: "text-sm text-base-content/70",
                                                RelativeTime {
                                                    format: RelativeTimeFormat::Relative,
                                                    datetime: spec.updated_at.clone()
                                                }
                                            }
                                            td {
                                                class: "text-right",
                                                if spec.is_system {
                                                    span {
                                                        class: "badge badge-info badge-outline",
                                                        title: "Enabled automatically for every team",
                                                        "System"
                                                    }
                                                } else {
                                                    DropDown {
                                                        direction: Direction::Left,
                                                        button_text: "...",
                                                        DropDownLink {
                                                            href: routes::openapi_specs::Edit { team_id: team_id.clone(), id: spec.id }.to_string(),
                                                            target: "_top",
                                                            "Edit"
                                                        }
                                                        DropDownLink {
                                                            popover_target: format!("delete-openapi-spec-{}-{}", team_id, spec.id),
                                                            href: "#",
                                                            target: "_top",
                                                            "Delete"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        for spec in specs {
                            if !spec.is_system {
                                ConfirmModal {
                                    action: routes::openapi_specs::Delete { team_id: team_id.clone(), id: spec.id }.to_string(),
                                    trigger_id: format!("delete-openapi-spec-{}-{}", team_id, spec.id),
                                    submit_label: "Delete".to_string(),
                                    heading: "Delete this OpenAPI Spec?".to_string(),
                                    warning: format!("Are you sure you want to delete '{}' ({})?", spec.title, spec.slug),
                                    hidden_fields: vec![
                                        ("team_id".into(), team_id.to_string()),
                                        ("id".into(), spec.id.to_string()),
                                    ],
                                }
                            }
                        }
                    }
                }

                if rbac.is_sys_admin {
                    super::upload::Upload { team_id: team_id.clone() }
                }
            }
        }
    };

    crate::render(page)
}
