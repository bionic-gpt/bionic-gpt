#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use crate::ConfirmModal;

#[component]
pub fn RemoveInviteDrawer(team_id: i32, invite_id: i32, trigger_id: String) -> Element {
    ConfirmModal {
        action: crate::routes::team::DeleteInvite{team_id}.to_string(),
        trigger_id,
        submit_label: "Remove Invite".to_string(),
        heading: "Remove this invite?".to_string(),
        warning: "Are you sure you want to remove this invite?".to_string(),
        hidden_fields: vec![
            ("team_id".into(), team_id.to_string()),
            ("invite_id".into(), invite_id.to_string()),
        ],
    }
}
