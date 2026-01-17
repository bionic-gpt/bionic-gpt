#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::Category;
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, categories: Vec<Category>) -> String {
    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::Categories,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Categories",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem { text: "Categories".into(), href: None }]
                }
                if rbac.is_sys_admin {
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        popover_target: "new-category-form",
                        button_scheme: ButtonScheme::Primary,
                        "Add Category"
                    }
                }
            ),
            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                SectionIntroduction {
                    header: "Categories".to_string(),
                    subtitle: "Organize your assistants with custom categories.".to_string(),
                    is_empty: categories.is_empty(),
                    empty_text: "No categories defined yet.".to_string(),
                }
                if !categories.is_empty() {
                    Card {
                        class: "mt-5 has-data-table",
                        CardHeader { title: "Categories" }
                        CardBody {
                            table {
                                class: "table table-sm",
                                thead {
                                    th { "Name" }
                                    th { "Description" }
                                    th { class: "text-right", "Action" }
                                }
                                tbody {
                                    for category in &categories {
                                        tr {
                                            td { "{category.name}" }
                                            td { "{category.description}" }
                                            td {
                                                class: "text-right",
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "...",
                                                    DropDownLink {
                                                        popover_target: format!("edit-trigger-{}-{}", category.id, team_id),
                                                        href: "#",
                                                        target: "_top",
                                                        "Edit"
                                                    }
                                                    DropDownLink {
                                                        popover_target: format!("delete-trigger-{}-{}", category.id, team_id),
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
                        for category in categories {
                            ConfirmModal {
                                action: crate::routes::categories::Delete { team_id, id: category.id }.to_string(),
                                trigger_id: format!("delete-trigger-{}-{}", category.id, team_id),
                                submit_label: "Delete".to_string(),
                                heading: "Delete this Category?".to_string(),
                                warning: "Are you sure you want to delete this Category?".to_string(),
                                hidden_fields: vec![
                                    ("team_id".into(), team_id.to_string()),
                                    ("id".into(), category.id.to_string()),
                                ],
                            }
                            super::upsert::Upsert {
                                id: Some(category.id),
                                trigger_id: format!("edit-trigger-{}-{}", category.id, team_id),
                                name: category.name,
                                description: category.description,
                                team_id
                            }
                        }
                    }
                }
                if rbac.is_sys_admin {
                    super::upsert::Upsert {
                        id: None,
                        trigger_id: "new-category-form",
                        name: "".to_string(),
                        description: "".to_string(),
                        team_id
                    }
                }
            }
        }
    };
    crate::render(page)
}
