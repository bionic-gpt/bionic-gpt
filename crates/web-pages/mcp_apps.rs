#![allow(non_snake_case)]

use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct McpServerSummary {
    pub id: i32,
    pub name: String,
    pub server_url: String,
    pub visibility: String,
    pub auth_type: String,
    pub connected: bool,
}

pub fn page(
    team_id: String,
    rbac: Rbac,
    servers: Vec<McpServerSummary>,
    message: Option<String>,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Integrations,
            team_id: team_id.clone(),
            rbac: rbac.clone(),
            title: "MCP Apps".to_string(),
            header: rsx!(Breadcrumb { items: vec![BreadcrumbItem { text: "Integrations".to_string(), href: Some(crate::routes::integrations::Index { team_id: team_id.clone() }.to_string()) }, BreadcrumbItem { text: "MCP Apps".to_string(), href: None }] }),
            div { class: "space-y-4",
                Card {
                    CardHeader { title: "Connect an MCP server" }
                    CardBody {
                        if let Some(message) = message {
                            div { class: "alert alert-error mb-4", "{message}" }
                        }
                        p { class: "mb-4 text-sm text-base-content/70", "Connect a remote Streamable HTTP MCP server. Connected tools can be used by the assistant, and tools that provide an MCP App can show an interactive view in chat." }
                        p { class: "mb-4 text-xs text-base-content/60", "HTTPS is recommended. HTTP endpoints are supported, but their traffic is unencrypted." }
                        form {
                        method: "post",
                        action: crate::routes::mcp_apps::Create { team_id: team_id.clone() }.to_string(),
                        class: "grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4",
                        label {
                            class: "form-control flex min-w-0 flex-col items-stretch gap-1",
                            span { class: "label-text", "Name" }
                            input { class: "input input-bordered w-full", name: "name", required: true, maxlength: "120", placeholder: "Project tracker" }
                        }
                        label {
                            class: "form-control flex min-w-0 flex-col items-stretch gap-1",
                            span { class: "label-text", "Server URL" }
                            input { class: "input input-bordered w-full", name: "server_url", required: true, r#type: "url", placeholder: "https://mcp.example.com/mcp" }
                        }
                        label {
                            class: "form-control flex min-w-0 flex-col items-stretch gap-1",
                            span { class: "label-text", "Authentication" }
                            select { class: "select select-bordered w-full", name: "auth_type", option { value: "none", "No authentication" } option { value: "bearer", "Bearer token" } option { value: "oauth2", "OAuth 2.1" } }
                        }
                        label {
                            class: "form-control flex min-w-0 flex-col items-stretch gap-1",
                            span { class: "label-text", "Bearer token" }
                            input { class: "input input-bordered w-full", name: "bearer_token", r#type: "password", autocomplete: "new-password", placeholder: "Leave blank when not required" }
                        }
                        label {
                            class: "form-control flex min-w-0 flex-col items-stretch gap-1",
                            span { class: "label-text", "OAuth authorization URL" }
                            input { class: "input input-bordered w-full", name: "authorization_url", r#type: "url", placeholder: "https://provider.example/authorize" }
                        }
                        label {
                            class: "form-control flex min-w-0 flex-col items-stretch gap-1",
                            span { class: "label-text", "OAuth token URL" }
                            input { class: "input input-bordered w-full", name: "token_url", r#type: "url", placeholder: "https://provider.example/token" }
                        }
                        label {
                            class: "form-control flex min-w-0 flex-col items-stretch gap-1",
                            span { class: "label-text", "OAuth client ID" }
                            input { class: "input input-bordered w-full", name: "client_id" }
                        }
                        label {
                            class: "form-control flex min-w-0 flex-col items-stretch gap-1",
                            span { class: "label-text", "OAuth client secret" }
                            input { class: "input input-bordered w-full", name: "client_secret", r#type: "password", autocomplete: "new-password" }
                        }
                        label {
                            class: "form-control flex min-w-0 flex-col items-stretch gap-1 md:col-span-2",
                            span { class: "label-text", "OAuth scopes (space separated)" }
                            input { class: "input input-bordered w-full", name: "oauth_scopes", placeholder: "openid profile" }
                        }
                        label {
                            class: "form-control flex min-w-0 flex-col items-stretch gap-1",
                            span { class: "label-text", "Available to" }
                            select { class: "select select-bordered w-full", name: "visibility", option { value: "Private", "Only me" } option { value: "Team", "Everyone in this team" } }
                        }
                        div { class: "md:col-span-2 xl:col-span-4 flex justify-end", Button { button_type: ButtonType::Submit, button_scheme: ButtonScheme::Primary, "Connect server" } }
                        }
                    }
                }
                Card {
                    CardHeader { title: "Connected MCP servers" }
                    CardBody {
                        if servers.is_empty() {
                            p { class: "text-sm text-base-content/60", "No MCP servers are connected yet." }
                        } else {
                            div { class: "divide-y divide-base-300",
                                for server in servers {
                                    div { class: "flex flex-col gap-3 py-4 sm:flex-row sm:items-center sm:justify-between",
                                        div { class: "min-w-0",
                                            h3 { class: "font-semibold", "{server.name}" }
                                            p { class: "break-all text-sm text-base-content/70", "{server.server_url}" }
                                            p { class: "mt-1 text-xs text-base-content/55", "{server.visibility}" }
                                            if server.auth_type == "oauth2" {
                                                p { class: "mt-1 text-xs text-base-content/55", if server.connected { "OAuth connected" } else { "OAuth authorization required" } }
                                            }
                                        }
                                        if server.auth_type == "oauth2" && !server.connected {
                                            a { class: "btn btn-primary", href: crate::routes::mcp_apps::Authorize { team_id: team_id.clone(), id: server.id }.to_string(), "Authorize" }
                                        }
                                        form { method: "post", action: crate::routes::mcp_apps::Delete { team_id: team_id.clone(), id: server.id }.to_string(), Button { button_type: ButtonType::Submit, button_scheme: ButtonScheme::Error, "Disconnect" } }
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
