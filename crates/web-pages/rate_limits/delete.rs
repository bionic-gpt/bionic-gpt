#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use crate::ConfirmModal;

#[component]
pub fn DeleteDrawer(team_id: i32, id: i32, trigger_id: String) -> Element {
    ConfirmModal {
        action: crate::routes::rate_limits::Delete {team_id, id}.to_string(),
        trigger_id,
        submit_label: "Delete".to_string(),
        heading: "Delete this Rate Limit?".to_string(),
        warning: "Are you sure you want to delete this rate limit?".to_string(),
        hidden_fields: vec![
            ("team_id".into(), team_id.to_string()),
            ("id".into(), id.to_string()),
        ],
    }
}
