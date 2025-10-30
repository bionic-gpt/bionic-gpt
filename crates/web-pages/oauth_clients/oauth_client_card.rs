#![allow(non_snake_case)]
use crate::components::card_item::CardItem;
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
    let delete_id = format!("delete_oauth_client_{}", props.oauth_client.id);
    rsx!(
        CardItem {
            class: Some("mt-5".into()),
            popover_target: None,
            image_src: None,
            avatar_name: Some(props.oauth_client.provider.clone()),
            title: props.oauth_client.provider.clone(),
            description: Some(rsx!(span { "Client ID: {props.oauth_client.client_id}" })),
            footer: Some(rsx!(span { "Created: " RelativeTime { format: RelativeTimeFormat::Relative, datetime: props.oauth_client.created_at.clone() } })),
            count_labels: vec![],
            action: if props.rbac.is_sys_admin {
                Some(rsx!(Button {
                    button_scheme: ButtonScheme::Error,
                    button_size: ButtonSize::Small,
                    popover_target: delete_id.clone(),
                    "Delete"
                }))
            } else {
                None
            }
        }

        if props.rbac.is_sys_admin {
            ConfirmModal {
                action: routes::oauth_clients::Delete {
                    team_id: props.team_id,
                    id: props.oauth_client.id
                }.to_string(),
                trigger_id: delete_id,
                submit_label: "Delete OAuth Client".to_string(),
                heading: "Delete OAuth Client".to_string(),
                warning: format!("Are you sure you want to delete the OAuth client for {}? This action cannot be undone.", props.oauth_client.provider),
                hidden_fields: vec![]
            }
        }
    )
}
