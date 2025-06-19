#![allow(non_snake_case)]
use crate::components::confirm_modal::ConfirmModal;
use crate::routes;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OauthClientCardProps {
    oauth_client: db::OauthClient,
    team_id: i32,
    rbac: Rbac,
}

#[component]
pub fn OauthClientCard(props: OauthClientCardProps) -> Element {
    rsx!(
        Card {
            class: "p-3 mt-5 flex flex-row",
            div {
                class: "flex flex-row flex-1 min-w-0",

                Avatar {
                    avatar_size: AvatarSize::Medium,
                    name: "{props.oauth_client.provider}"
                }
                div {
                    class: "ml-4 text-sm flex flex-col justify-center flex-1 min-w-0",
                    h2 {
                        class: "font-semibold text-base-content",
                        "{props.oauth_client.provider}"
                    }
                    p {
                        class: "text-base-content/70 truncate overflow-hidden whitespace-nowrap",
                        "Client ID: {props.oauth_client.client_id}"
                    }
                    p {
                        class: "text-xs text-base-content/50",
                        "Created: {props.oauth_client.created_at}"
                    }
                }
            }
            if props.rbac.is_sys_admin {
                div {
                    class: "flex flex-col justify-center ml-4",
                    Button {
                        button_scheme: ButtonScheme::Error,
                        button_size: ButtonSize::Small,
                        popover_target: format!("delete_oauth_client_{}", props.oauth_client.id),
                        "Delete"
                    }
                    ConfirmModal {
                        action: routes::oauth_clients::Delete {
                            team_id: props.team_id,
                            id: props.oauth_client.id
                        }.to_string(),
                        trigger_id: format!("delete_oauth_client_{}", props.oauth_client.id),
                        submit_label: "Delete OAuth Client".to_string(),
                        heading: "Delete OAuth Client".to_string(),
                        warning: format!("Are you sure you want to delete the OAuth client for {}? This action cannot be undone.", props.oauth_client.provider),
                        hidden_fields: vec![]
                    }
                }
            }
        }
    )
}
