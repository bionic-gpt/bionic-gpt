#![allow(non_snake_case)]
use crate::assistants::visibility::VisLabel;
use crate::routes;
use crate::ConfirmModal;
use assets::files::menu_delete_svg;
use daisy_rsx::*;
use db::ApiKeyConnection;
use dioxus::prelude::*;

pub fn ApiKeyConnectionsTable(
    team_id: i32,
    integration_id: i32,
    connections: Vec<ApiKeyConnection>,
) -> Element {
    rsx! {
        div {
            class: "overflow-x-auto",
            table {
                class: "table table-zebra w-full",
                thead {
                    tr {
                        th { "Visibility" }
                        th { "Created" }
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
                                    "{connection.created_at}"
                                }
                            }
                            td {
                                {
                                    let popover_target = format!("delete-api-key-{}", connection.id);
                                    rsx! {
                                        Button {
                                            prefix_image_src: "{menu_delete_svg.name}",
                                            popover_target: popover_target.clone(),
                                            button_scheme: ButtonScheme::Error,
                                            button_size: ButtonSize::Small,
                                            "Delete"
                                        }
                                        ConfirmModal {
                                            action: routes::integrations::DeleteApiKeyConnection{
                                                team_id,
                                                integration_id,
                                                connection_id: connection.id
                                            }.to_string(),
                                            trigger_id: popover_target,
                                            submit_label: "Delete".to_string(),
                                            heading: "Delete API Key Connection?".to_string(),
                                            warning: "Are you sure you want to delete this API key connection? This action cannot be undone.".to_string(),
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
