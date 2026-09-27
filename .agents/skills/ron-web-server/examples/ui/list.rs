use super::support::{render, routes, Introduction, ItemCard, ItemView, Layout};
use dioxus::prelude::*;

pub fn page(team_id: i64, items: Vec<ItemView>, can_create: bool) -> String {
    render(rsx! { Layout { title: "Items", team_id,
        Introduction { is_empty: items.is_empty() }
        if can_create {
            a { class: "btn btn-primary mb-4", href: routes::New { team_id }.to_string(), "Add item" }
        }
        for item in items {
            ItemCard { key: "{item.id}", team_id, item }
        }
    } })
}
