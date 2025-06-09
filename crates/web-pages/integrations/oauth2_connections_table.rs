#![allow(non_snake_case)]
use crate::assistants::visibility::VisLabel;
use crate::routes;
use crate::ConfirmModal;
use assets::files::menu_delete_svg;
use daisy_rsx::*;
use db::Oauth2Connection;
use dioxus::prelude::*;

pub fn Oauth2ConnectionsTable(
    team_id: i32,
    integration_id: i32,
    connections: Vec<Oauth2Connection>,
) -> Element {
    rsx! {
        div {
            class: "overflow-x-auto",
            table {
                class: "table table-zebra w-full",
                thead {
                    tr {
                        th { "Visibility" }
                        th { "Scopes" }
                        th { "Actions" }
                    }
                }
                tbody {
                    for connection in connections {
                        tr {
                            key: "{connection.id}",
                            td {
                                VisLabel {
                                    visibility: connection.visibility
                                }
                            }
                            td {
                                span {
                                    class: "text-sm text-gray-600",
                                    {
                                        if let Ok(scopes_array) = serde_json::from_value::<Vec<String>>(connection.scopes.clone()) {
                                            scopes_array.join(", ")
                                        } else {
                                            "Unknown".to_string()
                                        }
                                    }
                                }
                            }
                            td {
                                {
                                    let popover_target = format!("delete-oauth2-{}", connection.id);
                                    rsx! {
                                        Button {
                                            prefix_image_src: "{menu_delete_svg.name}",
                                            popover_target: popover_target.clone(),
                                            button_scheme: ButtonScheme::Error,
                                            button_size: ButtonSize::Small,
                                            "Delete"
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
            }
        }
    }
}
