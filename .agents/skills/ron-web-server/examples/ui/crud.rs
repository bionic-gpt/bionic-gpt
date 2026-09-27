use super::support::{render, DeleteConfirmation, ItemFields, ItemForm, Layout};
use dioxus::prelude::*;

// The loader must authorize ownership before calling this page.
pub fn page(team_id: i64, id: i64, name: String, error: Option<String>) -> String {
    render(rsx! { Layout { title: "Edit item", team_id,
        ItemFields { team_id, form: ItemForm { id: Some(id), name, error } }
        DeleteConfirmation { team_id, id }
    } })
}
