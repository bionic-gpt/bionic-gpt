#![allow(non_snake_case)]

use crate::app_layout::{Layout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
use crate::routes;
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, ApiKey};
use dioxus::prelude::*;
use time::format_description::well_known::Rfc3339;

#[derive(Default, Clone)]
pub struct NewKeyForm {
    pub name: String,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct GeneratedKey {
    pub name: String,
    pub value: String,
}

#[derive(Clone, PartialEq)]
struct McpKeyDisplay {
    id: i32,
    name: String,
    created_at: String,
    hash_suffix: String,
}

fn format_created_at(datetime: time::OffsetDateTime) -> String {
    datetime
        .format(&Rfc3339)
        .unwrap_or_else(|_| datetime.to_string())
}

fn mask_hash(hash: &str) -> String {
    if hash.is_empty() {
        return "Unknown".to_string();
    }

    let len = hash.chars().count();
    let suffix_len = suffix_length(len);
    let suffix: String = hash
        .chars()
        .rev()
        .take(suffix_len)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    format!("••••{}", suffix)
}

fn suffix_length(len: usize) -> usize {
    match len {
        0..=4 => len,
        5..=8 => 4,
        _ => 6,
    }
}

pub fn page(
    rbac: Rbac,
    team_id: i32,
    keys: Vec<ApiKey>,
    form: NewKeyForm,
    generated_key: Option<GeneratedKey>,
) -> String {
    let mut displays: Vec<McpKeyDisplay> = keys
        .into_iter()
        .map(|key| McpKeyDisplay {
            id: key.id,
            name: key.name,
            created_at: format_created_at(key.created_at),
            hash_suffix: mask_hash(&key.api_key),
        })
        .collect();

    displays.sort_by(|a, b| a.name.cmp(&b.name));

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::McpApiKeys,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "MCP API Keys",
            header: rsx!(
                div {
                    class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
                    Breadcrumb {
                        items: vec![
                            BreadcrumbItem {
                                text: crate::i18n::integrations().into(),
                                href: Some(routes::integrations::Index { team_id }.to_string()),
                            },
                            BreadcrumbItem {
                                text: "API Keys".into(),
                                href: Some(routes::mcp_api_keys::Index { team_id }.to_string()),
                            }
                        ]
                    }
                    Button {
                        prefix_image_src: button_plus_svg.name,
                        button_scheme: ButtonScheme::Primary,
                        popover_target: "create-mcp-key",
                        "Create MCP API Key"
                    }
                }
            ),
            div {
                class: "space-y-6",
                if let Some(error) = &form.error {
                    Alert {
                        alert_color: AlertColor::Error,
                        class: "flex items-center gap-2",
                        "{error}"
                    }
                }
                if let Some(created) = generated_key.clone() {
                    Alert {
                        alert_color: AlertColor::Success,
                        class: "flex flex-col gap-3",
                        div { class: "font-semibold", "API Key Created" }
                        div { class: "text-sm opacity-90", "Copy and store the API key for {created.name}. This is the only time it will be shown." }
                        Input {
                            input_type: InputType::Text,
                            value: created.value.clone(),
                            readonly: true,
                            name: "generated-api-key",
                        }
                        div { class: "text-xs opacity-75", "Available to all MCP servers" }
                    }
                }

                if displays.is_empty() {
                    div { class: "text-sm text-base-content/70", "No MCP API keys created yet." }
                } else {
                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "table table-sm",
                            thead {
                                th { "Name" }
                                th { "Key Suffix" }
                                th { "Created" }
                                th { class: "text-right", "Action" }
                            }
                            tbody {
                                for key in displays.iter() {
                                    McpKeyRow { team_id, item: key.clone() }
                                }
                            }
                        }
                    }
                }

                form {
                    method: "post",
                    action: routes::mcp_api_keys::Create { team_id }.to_string(),
                    Modal {
                        trigger_id: "create-mcp-key",
                        ModalBody {
                            h3 { class: "font-bold text-lg mb-4", "New MCP API Key" }
                            div {
                                class: "flex flex-col gap-4",
                                Fieldset {
                                    legend: "Key Name",
                                    help_text: "Give this key a descriptive name so you know where it is used.",
                                    Input {
                                        name: "name",
                                        value: form.name.clone(),
                                        required: true,
                                        placeholder: "My reporting service",
                                    }
                                }
                                Fieldset {
                                    legend: "Usage",
                                    help_text: "Keys apply to every MCP integration. We'll validate the key on each MCP request."
                                }
                            }
                            ModalAction {
                                Button {
                                    class: "cancel-modal",
                                    button_scheme: ButtonScheme::Warning,
                                    button_size: ButtonSize::Small,
                                    "Cancel"
                                }
                                Button {
                                    button_type: ButtonType::Submit,
                                    button_scheme: ButtonScheme::Primary,
                                    "Create API Key"
                                }
                            }
                        }
                    }
                }

                for key in displays.iter() {
                    ConfirmModal {
                        action: routes::api_keys::Delete { team_id, id: key.id }.to_string(),
                        trigger_id: delete_trigger_id(team_id, key.id),
                        submit_label: "Delete".to_string(),
                        heading: "Delete API Key?".to_string(),
                        warning: "This will revoke access for MCP requests using this key.".to_string(),
                        hidden_fields: vec![
                            ("team_id".into(), team_id.to_string()),
                            ("id".into(), key.id.to_string()),
                        ],
                    }
                }
            }
        }
    };

    crate::render(page)
}

#[component]
fn McpKeyRow(team_id: i32, item: McpKeyDisplay) -> Element {
    let trigger_id = delete_trigger_id(team_id, item.id);

    rsx! {
        tr {
            td { "{item.name}" }
            td { span { class: "font-mono text-sm", "{item.hash_suffix}" } }
            td { class: "text-sm", "{item.created_at}" }
            td {
                class: "text-right",
                Button {
                    button_scheme: ButtonScheme::Error,
                    button_size: ButtonSize::Small,
                    prefix_image_src: menu_delete_svg.name,
                    popover_target: trigger_id,
                    "Delete"
                }
            }
        }
    }
}

fn delete_trigger_id(team_id: i32, key_id: i32) -> String {
    format!("delete-mcp-key-{}-{}", team_id, key_id)
}
