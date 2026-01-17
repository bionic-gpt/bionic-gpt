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

pub fn page(team_id: i32, rbac: Rbac, specs: Vec<OpenapiSpec>) -> String {
    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::OpenapiSpecs,
            team_id,
            title: "OpenAPI Specs",
            rbac: rbac.clone(),
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "OpenAPI Specs".into(),
                        href: Some(routes::openapi_specs::Index { team_id }.to_string()),
                    }]
                }
                if rbac.is_sys_admin {
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        button_scheme: ButtonScheme::Primary,
                        href: routes::openapi_specs::New { team_id }.to_string(),
                        "Add OpenAPI Spec"
                    }
                }
            ),
            div {
                class: "p-4 max-w-5xl w-full mx-auto flex flex-col gap-6",
                SectionIntroduction {
                    header: "OpenAPI Specs".to_string(),
                    subtitle: "Manage the prebuilt OpenAPI specifications available to teams.".to_string(),
                    is_empty: specs.is_empty(),
                    empty_text: "No OpenAPI specs available yet. Add one to get started.".to_string(),
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
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "...",
                                                    DropDownLink {
                                                        href: routes::openapi_specs::Edit { team_id, id: spec.id }.to_string(),
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

                        for spec in specs {
                            ConfirmModal {
                                action: routes::openapi_specs::Delete { team_id, id: spec.id }.to_string(),
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
        }
    };

    crate::render(page)
}
