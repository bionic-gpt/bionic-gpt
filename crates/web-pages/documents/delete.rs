#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use crate::ConfirmModal;

#[component]
pub fn DeleteDrawer(
    team_id: i32,
    document_id: i32,
    dataset_id: i32,
    trigger_id: String,
) -> Element {
    rsx! {
        ConfirmModal {
            action: crate::routes::documents::Delete{team_id, document_id}.to_string(),
            trigger_id,
            submit_label: "Delete Document".to_string(),
            heading: "Delete this document?".to_string(),
            warning: "Are you sure you want to delete this document?".to_string(),
            hidden_fields: vec![
                ("team_id".into(), team_id.to_string()),
                ("document_id".into(), document_id.to_string()),
                ("dataset_id".into(), dataset_id.to_string()),
            ],
        }
    }
}
