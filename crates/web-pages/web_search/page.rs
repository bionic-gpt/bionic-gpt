#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use crate::SectionIntroduction;
use daisy_rsx::*;
use db::authz::Rbac;
use db::OpenapiSpec;
use dioxus::prelude::*;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    specs: Vec<OpenapiSpec>,
    selected_spec_id: Option<i32>,
) -> String {
    let rows: Vec<Element> = specs
        .iter()
        .map(|spec| {
            let is_selected = selected_spec_id == Some(spec.id);
            rsx!(
                tr {
                    td { "{spec.title}" }
                    td { code { "{spec.slug}" } }
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
                        if is_selected {
                            span { class: "badge badge-info badge-outline", "Selected" }
                        } else {
                            span { class: "text-base-content/60", "-" }
                        }
                    }
                    td {
                        class: "text-right",
                        form {
                            method: "post",
                            action: crate::routes::web_search::Select { team_id, id: spec.id }.to_string(),
                            Button {
                                button_type: ButtonType::Submit,
                                button_scheme: ButtonScheme::Primary,
                                button_size: ButtonSize::Small,
                                disabled: !spec.is_active || is_selected,
                                "Select"
                            }
                        }
                    }
                }
            )
        })
        .collect();

    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::WebSearch,
            team_id,
            title: "Web Search",
            rbac: rbac.clone(),
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Web Search".into(),
                        href: None,
                    }]
                }
            ),
            div {
                class: "p-4 max-w-5xl w-full mx-auto flex flex-col gap-6",
                SectionIntroduction {
                    header: "Web Search".to_string(),
                    subtitle: "Pick the OpenAPI spec used for web search tooling.".to_string(),
                    is_empty: specs.is_empty(),
                    empty_text: "No Web Search specs available yet.".to_string(),
                }

                if !specs.is_empty() {
                    Card {
                        class: "has-data-table",
                        CardHeader { title: "Web Search Specs" }
                        CardBody {
                            table {
                                class: "table table-sm",
                                thead {
                                    tr {
                                        th { "Title" }
                                        th { "Slug" }
                                        th { "Status" }
                                        th { "Selected" }
                                        th { class: "text-right", "Action" }
                                    }
                                }
                                tbody {
                                    for row in rows {
                                        {row}
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
