use super::support::{render, ItemFields, ItemForm, Layout};
use dioxus::prelude::*;

pub fn page(team_id: i64, form: ItemForm) -> String {
    let title = if form.id.is_some() {
        "Edit item"
    } else {
        "New item"
    };
    render(rsx! { Layout { title, team_id, ItemFields { team_id, form } } })
}
