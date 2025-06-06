#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use crate::ConfirmModal;

#[component]
pub fn RemoveMemberDrawer(
    team_id: i32,
    email: String,
    user_id: i32,
    trigger_id: String,
) -> Element {
    rsx! {
        ConfirmModal {
            action: crate::routes::team::Delete{team_id}.to_string(),
            trigger_id,
            submit_label: "Remove User".to_string(),
            heading: "Remove this user?".to_string(),
            warning: format!("Are you sure you want to remove '{email}' from the team?"),
            hidden_fields: vec![
                ("team_id".into(), team_id.to_string()),
                ("user_id".into(), user_id.to_string()),
            ],
        }
    }
}
