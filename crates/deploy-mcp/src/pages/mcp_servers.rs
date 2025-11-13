use dioxus::prelude::*;
use serde_json::Value;
use tracing::warn;

use crate::mcp_specs::all_specs;

use crate::components::extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE};
use crate::components::footer::Footer;
use crate::components::navigation::Section;
use crate::layouts::layout::Layout;

#[derive(Clone, PartialEq, Eq)]
pub struct IntegrationSpec {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub logo_url: Option<String>,
    pub endpoints: Vec<Endpoint>,
}

struct IntegrationMetadata {
    title: String,
    description: Option<String>,
    version: Option<String>,
    logo_url: Option<String>,
}

impl IntegrationSpec {
    pub fn detail_path(&self) -> String {
        format!("/mcp-servers/{}/", self.slug)
    }

    pub fn folder_name(&self) -> String {
        format!("mcp-servers/{}", self.slug)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub method: String,
    pub path: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: Vec<Parameter>,
    pub request_body_content: Vec<String>,
    pub responses: Vec<Response>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub location: Option<String>,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Response {
    pub status: String,
    pub description: Option<String>,
}

pub fn load_integration_specs() -> Vec<IntegrationSpec> {
    let mut specs = Vec::new();

    for spec in all_specs() {
        let value: Value = match serde_json::from_str(spec.json) {
            Ok(value) => value,
            Err(err) => {
                warn!("failed to parse integration {}: {}", spec.slug, err);
                continue;
            }
        };

        let metadata = match parse_metadata(&value, spec.slug) {
            Some(metadata) => metadata,
            None => {
                warn!(
                    "failed to load integration {}: unable to read metadata",
                    spec.slug
                );
                continue;
            }
        };

        let mut endpoints = parse_endpoints(&value);
        endpoints.sort_by(|a, b| a.path.cmp(&b.path).then(a.method.cmp(&b.method)));

        specs.push(IntegrationSpec {
            slug: spec.slug.to_string(),
            title: metadata.title,
            description: metadata.description,
            version: metadata.version,
            logo_url: metadata.logo_url,
            endpoints,
        });
    }

    specs.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    specs
}

fn parse_metadata(spec: &Value, fallback_title: &str) -> Option<IntegrationMetadata> {
    let info = spec.get("info")?.as_object()?;

    let title = info
        .get("title")
        .and_then(|title| title.as_str())
        .map(|title| title.to_string())
        .unwrap_or_else(|| fallback_title.to_string());

    let description = info
        .get("description")
        .and_then(|description| description.as_str())
        .map(|description| description.to_string());

    let version = info
        .get("version")
        .and_then(|version| version.as_str())
        .map(|version| version.to_string());

    let logo_url = info
        .get("x-logo")
        .and_then(|logo| match logo {
            Value::Object(obj) => obj.get("url").and_then(|url| url.as_str()),
            Value::String(url) => Some(url.as_str()),
            _ => None,
        })
        .map(|url| url.to_string());

    Some(IntegrationMetadata {
        title,
        description,
        version,
        logo_url,
    })
}

fn parse_endpoints(spec: &Value) -> Vec<Endpoint> {
    let mut endpoints = Vec::new();

    let paths = match spec.get("paths").and_then(|paths| paths.as_object()) {
        Some(paths) => paths,
        None => return endpoints,
    };

    let methods = ["get", "post", "put", "delete", "patch", "options", "head"];

    for (path, path_item) in paths {
        if let Some(path_obj) = path_item.as_object() {
            for method in methods {
                if let Some(operation) = path_obj.get(method).and_then(|value| value.as_object()) {
                    endpoints.push(Endpoint {
                        method: method.to_uppercase(),
                        path: path.clone(),
                        summary: operation
                            .get("summary")
                            .and_then(|summary| summary.as_str())
                            .map(|summary| summary.to_string()),
                        description: operation
                            .get("description")
                            .and_then(|description| description.as_str())
                            .map(|description| description.to_string()),
                        operation_id: operation
                            .get("operationId")
                            .and_then(|id| id.as_str())
                            .map(|id| id.to_string()),
                        parameters: parse_parameters(operation),
                        request_body_content: parse_request_body(operation),
                        responses: parse_responses(operation),
                    });
                }
            }
        }
    }

    endpoints
}

fn parse_parameters(operation: &serde_json::Map<String, Value>) -> Vec<Parameter> {
    let mut parameters = Vec::new();

    if let Some(array) = operation
        .get("parameters")
        .and_then(|params| params.as_array())
    {
        for param in array {
            if let Some(param_obj) = param.as_object() {
                let name = param_obj
                    .get("name")
                    .and_then(|name| name.as_str())
                    .unwrap_or("Unnamed parameter")
                    .to_string();
                let location = param_obj
                    .get("in")
                    .and_then(|location| location.as_str())
                    .map(|location| location.to_string());
                let required = param_obj
                    .get("required")
                    .and_then(|required| required.as_bool())
                    .unwrap_or(false);
                let description = param_obj
                    .get("description")
                    .and_then(|description| description.as_str())
                    .map(|description| description.to_string());

                parameters.push(Parameter {
                    name,
                    location,
                    required,
                    description,
                });
            }
        }
    }

    parameters
}

fn parse_request_body(operation: &serde_json::Map<String, Value>) -> Vec<String> {
    let mut content_types = Vec::new();

    if let Some(body) = operation
        .get("requestBody")
        .and_then(|body| body.as_object())
    {
        if let Some(content) = body.get("content").and_then(|content| content.as_object()) {
            for content_type in content.keys() {
                content_types.push(content_type.to_string());
            }
        }
    }

    content_types
}

fn parse_responses(operation: &serde_json::Map<String, Value>) -> Vec<Response> {
    let mut responses = Vec::new();

    if let Some(response_map) = operation
        .get("responses")
        .and_then(|responses| responses.as_object())
    {
        for (status, response) in response_map {
            if let Some(response_obj) = response.as_object() {
                responses.push(Response {
                    status: status.to_string(),
                    description: response_obj
                        .get("description")
                        .and_then(|description| description.as_str())
                        .map(|description| description.to_string()),
                });
            } else {
                responses.push(Response {
                    status: status.to_string(),
                    description: None,
                });
            }
        }
    }

    responses
}

pub fn index_page(integrations: &[IntegrationSpec]) -> String {
    let body = rsx! {
        div {
            class: "mt-20 mx-auto lg:max-w-5xl p-6 space-y-12",
            section {
                class: "space-y-4 text-center",
                h1 { class: "text-4xl font-bold", "Managed MCP Servers" }
                p {
                    class: "text-lg text-base-content/80",
                    "Browse fully-managed MCP servers that connect Deploy to the systems your assistants rely on."
                }
            }
            section {
                class: "grid gap-6 sm:grid-cols-2",
                if integrations.is_empty() {
                    div {
                        class: "col-span-full rounded-lg border border-dashed border-base-300 p-8 text-center text-base-content/70",
                        "No MCP servers available yet. Check back soon."
                    }
                } else {
                    for integration in integrations {
                        div {
                            class: "card h-full border border-base-200 bg-base-100 shadow-sm",
                            if let Some(logo) = integration.logo_url.as_deref() {
                                figure {
                                    class: "flex items-center justify-center border-b border-base-200 bg-base-200/40 p-6",
                                    img {
                                        class: "max-h-20", alt: "{integration.title} logo", src: "{logo}" }
                                }
                            }
                            div {
                                class: "card-body gap-4",
                                h2 { class: "card-title", "{integration.title}" }
                                p {
                                    class: "text-sm text-base-content/80",
                                    "{integration.description.as_deref().unwrap_or(\"No description provided.\")}" }
                                div {
                                    class: "flex flex-wrap items-center gap-2 text-xs text-base-content/70",
                                    if let Some(version) = integration.version.as_deref() {
                                        span { class: "badge badge-outline", "v{version}" }
                                    }
                                    span { class: "badge badge-outline", "{integration.endpoints.len()} endpoints" }
                                }
                                div {
                                    class: "card-actions justify-end",
                                    a { class: "btn btn-primary btn-sm", href: "{integration.detail_path()}", "View endpoints" }
                                }
                            }
                        }
                    }
                }
            }
        }
        ExtraFooter {
            title: EXTRA_FOOTER_TITLE.to_string(),
            image: "/docs/mcp-servers.png".to_string(),
            cta: "Get Started".to_string(),
            cta_url: crate::routes::marketing::Index {}.to_string(),
        }
        Footer { margin_top: "mt-0" }
    };

    let page = rsx! {
        Layout {
            title: "Managed MCP Servers | Deploy".to_string(),
            description: "Discover managed MCP servers maintained by Deploy for production AI assistants.".to_string(),
            url: Some("https://deploy.run/mcp-servers".to_string()),
            section: Section::McpServers,
            mobile_menu: None,
            image: None,
            children: body,
        }
    };

    crate::render(page)
}

pub fn detail_page(integration: &IntegrationSpec) -> String {
    let description = integration
        .description
        .as_deref()
        .unwrap_or("No description provided.");
    let version = integration
        .version
        .clone()
        .unwrap_or_else(|| "N/A".to_string());

    let body = rsx! {
        div {
            class: "mt-16 mx-auto lg:max-w-5xl p-6 space-y-12",
            nav {
                a { class: "link link-primary", href: crate::routes::marketing::McpServers {}.to_string(), "← Back to MCP servers" }
            }
            section {
                class: "rounded-xl border border-base-200 bg-base-100 p-8 shadow-sm space-y-4",
                h1 { class: "text-4xl font-bold", "{integration.title}" }
                p { class: "text-lg text-base-content/80", "{description}" }
                div {
                    class: "flex flex-wrap items-center gap-3 text-sm text-base-content/70",
                    span { class: "badge badge-outline", "Version {version}" }
                    span { class: "badge badge-outline", "{integration.endpoints.len()} endpoints" }
                }
                if let Some(logo) = integration.logo_url.as_deref() {
                    div {
                        class: "pt-4",
                        img { class: "max-h-24", alt: "{integration.title} logo", src: "{logo}" }
                    }
                }
            }
            section {
                class: "space-y-6",
                h2 { class: "text-3xl font-semibold", "Endpoints" }
                if integration.endpoints.is_empty() {
                    p { class: "text-base-content/70", "This MCP server does not expose any endpoints." }
                } else {
                    for endpoint in integration.endpoints.clone() {
                        EndpointCard { endpoint }
                    }
                }
            }
        }
        ExtraFooter {
            title: EXTRA_FOOTER_TITLE.to_string(),
            image: "/docs/mcp-servers.png".to_string(),
            cta: "Get Started".to_string(),
            cta_url: crate::routes::marketing::Index {}.to_string(),
        }
        Footer { margin_top: "mt-0" }
    };

    let page = rsx! {
        Layout {
            title: format!("{} | MCP Server | Deploy", integration.title),
            description: description.to_string(),
            url: Some(format!("https://deploy.run{}", integration.detail_path())),
            section: Section::McpServers,
            mobile_menu: None,
            image: integration.logo_url.clone(),
            children: body,
        }
    };

    crate::render(page)
}

#[component]
fn EndpointCard(endpoint: Endpoint) -> Element {
    let description = endpoint
        .description
        .as_deref()
        .or(endpoint.summary.as_deref())
        .unwrap_or("No description provided.");

    rsx! {
        article {
            class: "card border border-base-200 bg-base-100 shadow-sm",
            div {
                class: "card-body space-y-4",
                div {
                    class: "flex flex-wrap items-center gap-3",
                    span { class: "badge badge-primary badge-outline", "{endpoint.method}" }
                    code { class: "rounded bg-base-200 px-2 py-1 text-sm", "{endpoint.path}" }
                    if let Some(operation_id) = endpoint.operation_id.as_deref() {
                        span { class: "text-xs text-base-content/60", "{operation_id}" }
                    }
                }
                p { class: "text-sm text-base-content/80", "{description}" }
                if !endpoint.parameters.is_empty() {
                    div {
                        class: "space-y-2",
                        h3 { class: "text-base font-semibold", "Parameters" }
                        div {
                            class: "overflow-x-auto",
                            table {
                                class: "table table-zebra w-full",
                                thead {
                                    tr {
                                        th { "Name" }
                                        th { "In" }
                                        th { "Required" }
                                        th { "Description" }
                                    }
                                }
                                tbody {
                                    for param in endpoint.parameters {
                                        tr {
                                            td { "{param.name}" }
                                            td { "{param.location.as_deref().unwrap_or(\"-\")}" }
                                            td { if param.required { "Yes" } else { "No" } }
                                            td {
                                                class: "max-w-xs whitespace-pre-wrap text-sm",
                                                "{param.description.as_deref().unwrap_or(\"\")}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if !endpoint.request_body_content.is_empty() {
                    div {
                        class: "space-y-2",
                        h3 { class: "text-base font-semibold", "Request body" }
                        ul {
                            class: "list-disc space-y-1 pl-6 text-sm",
                            for content_type in endpoint.request_body_content {
                                li { code { "{content_type}" } }
                            }
                        }
                    }
                }
                if !endpoint.responses.is_empty() {
                    div {
                        class: "space-y-2",
                        h3 { class: "text-base font-semibold", "Responses" }
                        div {
                            class: "overflow-x-auto",
                            table {
                                class: "table table-compact w-full",
                                thead {
                                    tr {
                                        th { "Status" }
                                        th { "Description" }
                                    }
                                }
                                tbody {
                                    for response in endpoint.responses {
                                        tr {
                                            td { "{response.status}" }
                                            td {
                                                class: "max-w-xs whitespace-pre-wrap text-sm",
                                                "{response.description.as_deref().unwrap_or(\"\")}"
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
}
