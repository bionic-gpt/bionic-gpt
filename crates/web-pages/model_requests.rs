#![allow(non_snake_case)]

use crate::app_layout::{AdminLayout, SideBar};
use crate::routes;
use crate::SectionIntroduction;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{ChatStatus, ModelRequestDetail, ModelRequestSummary};
use dioxus::prelude::*;

pub fn page(
    team_id: String,
    rbac: Rbac,
    requests: Vec<ModelRequestSummary>,
    next_before_id: Option<i64>,
) -> String {
    let index_href = routes::model_requests::Index {
        team_id: team_id.clone(),
    }
    .to_string();

    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::ModelRequests,
            team_id: team_id.clone(),
            rbac,
            title: "Requests",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Requests".into(),
                        href: Some(index_href.clone()),
                    }]
                }
            ),
            div {
                class: "p-4 max-w-7xl w-full mx-auto flex flex-col gap-6",
                SectionIntroduction {
                    header: "LLM Requests".to_string(),
                    subtitle: "Inspect the final payload sent to a configured chat model.".to_string(),
                    is_empty: requests.is_empty(),
                    empty_text: "No model requests have been recorded for this team yet.".to_string(),
                }

                if !requests.is_empty() {
                    Card {
                        class: "has-data-table",
                        CardHeader { title: "Recent Requests" }
                        CardBody {
                            div {
                                class: "overflow-x-auto",
                                table {
                                    class: "table table-sm",
                                    thead {
                                        tr {
                                            th { "When" }
                                            th { "Model" }
                                            th { "User" }
                                            th { "Status" }
                                            th { "Endpoint" }
                                            th { class: "text-right", "Chat" }
                                        }
                                    }
                                    tbody {
                                        for request in requests {
                                            tr {
                                                td {
                                                    RelativeTime {
                                                        format: RelativeTimeFormat::Relative,
                                                        datetime: &request.created_at
                                                    }
                                                }
                                                td {
                                                    a {
                                                        class: "link link-hover font-medium",
                                                        href: routes::model_requests::View {
                                                            team_id: team_id.clone(),
                                                            id: request.id,
                                                        }.to_string(),
                                                        "{request.model_name}"
                                                    }
                                                    div {
                                                        class: "text-xs text-base-content/60",
                                                        "{request.provider_type}"
                                                    }
                                                }
                                                td { "{request.email}" }
                                                td {
                                                    span {
                                                        class: status_badge_class(request.status),
                                                        {status_label(request.status)}
                                                    }
                                                }
                                                td {
                                                    code {
                                                        class: "text-xs break-all",
                                                        "{request.method} {request.uri}"
                                                    }
                                                }
                                                td {
                                                    class: "text-right font-mono text-xs",
                                                    "{request.chat_id}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(before_id) = next_before_id {
                        div {
                            class: "flex justify-end",
                            a {
                                class: "btn btn-neutral btn-sm",
                                href: format!("{}?before_id={}", index_href, before_id),
                                "Older requests"
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}

pub fn detail_page(team_id: String, rbac: Rbac, request: ModelRequestDetail) -> String {
    let pretty_body = serde_json::from_str::<serde_json::Value>(&request.body)
        .and_then(|value| serde_json::to_string_pretty(&value))
        .unwrap_or_else(|_| request.body.clone());

    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::ModelRequests,
            team_id: team_id.clone(),
            rbac,
            title: "Request Details",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Requests".into(),
                            href: Some(routes::model_requests::Index { team_id: team_id.clone() }.to_string()),
                        },
                        BreadcrumbItem {
                            text: format!("Request {}", request.id),
                            href: None,
                        },
                    ]
                }
            ),
            div {
                class: "p-4 max-w-6xl w-full mx-auto flex flex-col gap-6",
                Card {
                    CardHeader { title: "Request Details" }
                    CardBody {
                        dl {
                            class: "grid grid-cols-1 md:grid-cols-[12rem_1fr] gap-x-6 gap-y-3 text-sm",
                            dt { class: "font-semibold", "Sent" }
                            dd {
                                RelativeTime {
                                    format: RelativeTimeFormat::Relative,
                                    datetime: &request.created_at
                                }
                            }
                            dt { class: "font-semibold", "Model" }
                            dd { "{request.model_name} ({request.provider_type})" }
                            dt { class: "font-semibold", "User" }
                            dd { "{request.email}" }
                            dt { class: "font-semibold", "Conversation / Chat" }
                            dd { class: "font-mono", "{request.conversation_id} / {request.chat_id}" }
                            dt { class: "font-semibold", "Chat status" }
                            dd {
                                span {
                                    class: status_badge_class(request.status),
                                    {status_label(request.status)}
                                }
                            }
                            dt { class: "font-semibold", "Endpoint" }
                            dd { code { class: "break-all", "{request.method} {request.uri}" } }
                        }
                    }
                }

                Card {
                    CardHeader { title: "Payload" }
                    CardBody {
                        p {
                            class: "text-sm text-base-content/70 mb-3",
                            "Authorization headers and configured provider credentials are not stored."
                        }
                        pre {
                            class: "bg-base-200 rounded-box p-4 overflow-auto text-xs leading-relaxed whitespace-pre min-h-32",
                            code { "{pretty_body}" }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}

fn status_label(status: ChatStatus) -> &'static str {
    match status {
        ChatStatus::Pending => "Pending",
        ChatStatus::InProgress => "In progress",
        ChatStatus::Success => "Success",
        ChatStatus::Cancelled => "Cancelled",
        ChatStatus::Error => "Error",
    }
}

fn status_badge_class(status: ChatStatus) -> &'static str {
    match status {
        ChatStatus::Success => "badge badge-success badge-outline",
        ChatStatus::Error | ChatStatus::Cancelled => "badge badge-error badge-outline",
        ChatStatus::Pending | ChatStatus::InProgress => "badge badge-ghost",
    }
}
