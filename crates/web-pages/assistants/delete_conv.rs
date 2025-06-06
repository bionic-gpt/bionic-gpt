#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use crate::ConfirmModal;

#[component]
pub fn DeleteDrawer(
    team_id: i32,
    conversation_id: i64,
    prompt_id: i32,
    trigger_id: String,
) -> Element {
    rsx! {
        ConfirmModal {
            action: crate::routes::prompts::DeleteConv{team_id, prompt_id, conversation_id}.to_string(),
            trigger_id,
            submit_label: "Delete".to_string(),
            heading: "Delete this Conversation?".to_string(),
            warning: "Are you sure you want to delete this Conversation?".to_string(),
            hidden_fields: vec![
                ("team_id".into(), team_id.to_string()),
                ("id".into(), conversation_id.to_string()),
                ("prompt_id".into(), prompt_id.to_string()),
            ],
        }
    }
}
