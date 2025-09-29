#![allow(non_snake_case)]
use crate::assistants::visibility::VisLabel;
use crate::components::confirm_modal::ConfirmModal;
use crate::routes;
use assets::files::menu_delete_svg;
use daisy_rsx::*;
use db::Oauth2Connection;
use dioxus::prelude::*;

pub fn Oauth2Cards(
    team_id: i32,
    integration_id: i32,
    mcp_slug: Option<String>,
    connections: Vec<Oauth2Connection>,
) -> Element {
    rsx! {
        div {
            class: "space-y-3",
            for connection in connections {
                {
                    let popover_target = format!("delete-oauth2-{}", connection.id);
                    let slug_modal = mcp_slug.clone().map(|slug| {
                        let modal_id = format!("mcp-url-oauth2-{}", connection.id);
                        let mcp_url = format!("/v1/mcp/{}/{}", slug, connection.external_id);
                        (modal_id, mcp_url)
                    });
                    rsx! {
                        Card {
                            key: "{connection.id}",
                            class: "p-4",
                            div {
                                class: "flex justify-between items-start",
                                div {
                                    class: "flex flex-col space-y-2",
                                    div {
                                        VisLabel {
                                            visibility: connection.visibility
                                        }
                                    }
                                    div {
                                        span {
                                            class: "text-sm text-gray-600",
                                            "Scopes: "
                                            {
                                                if let Ok(scopes_array) = serde_json::from_value::<Vec<String>>(connection.scopes.clone()) {
                                                    scopes_array.join(", ")
                                                } else {
                                                    "Unknown".to_string()
                                                }
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col justify-center items-end gap-2",
                                    if let Some((modal_id, mcp_url)) = slug_modal {
                                        Button {
                                            popover_target: modal_id.clone(),
                                            button_style: ButtonStyle::Outline,
                                            button_scheme: ButtonScheme::Primary,
                                            button_size: ButtonSize::Small,
                                            "View MCP URL"
                                        }
                                        Modal {
                                            trigger_id: &modal_id,
                                            ModalBody {
                                                h3 {
                                                    class: "font-bold text-lg mb-2",
                                                    "Machine Connection Protocol URL"
                                                }
                                                p {
                                                    class: "text-sm text-base-content/70 mb-3",
                                                    "Use this URL to connect your MCP client to this OAuth2 connection."
                                                }
                                                pre {
                                                    class: "bg-base-200 rounded p-3 text-sm break-all",
                                                    "{mcp_url}"
                                                }
                                                ModalAction {
                                                    Button {
                                                        class: "cancel-modal",
                                                        button_scheme: ButtonScheme::Primary,
                                                        button_size: ButtonSize::Small,
                                                        "Close"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Button {
                                        prefix_image_src: "{menu_delete_svg.name}",
                                        popover_target: popover_target.clone(),
                                        button_scheme: ButtonScheme::Error,
                                        button_size: ButtonSize::Small,
                                        "Delete"
                                    }
                                }
                            }
                        }
                        ConfirmModal {
                            action: routes::integrations::DeleteOauth2Connection{
                                team_id,
                                integration_id,
                                connection_id: connection.id
                            }.to_string(),
                            trigger_id: popover_target,
                            submit_label: "Delete".to_string(),
                            heading: "Delete OAuth2 Connection?".to_string(),
                            warning: "Are you sure you want to delete this OAuth2 connection? This action cannot be undone.".to_string(),
                            hidden_fields: vec![
                                ("team_id".into(), team_id.to_string()),
                                ("integration_id".into(), integration_id.to_string()),
                                ("connection_id".into(), connection.id.to_string()),
                            ],
                        }
                    }
                }
            }
        }
    }
}
