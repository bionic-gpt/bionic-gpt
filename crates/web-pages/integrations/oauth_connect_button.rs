#![allow(non_snake_case)]
use crate::routes;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OauthConnectButtonProps {
    team_id: String,
    integration_id: i32,
    label: String,
    class: String,
}

#[component]
pub fn OauthConnectButton(props: OauthConnectButtonProps) -> Element {
    rsx! {
        a {
            class: "{props.class}",
            href: routes::integrations::Connect { team_id: props.team_id, integration_id: props.integration_id }.to_string(),
            "{props.label}"
        }
    }
}
