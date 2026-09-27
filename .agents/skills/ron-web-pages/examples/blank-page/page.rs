use super::support::{render, Layout};
use dioxus::prelude::*;

pub fn page(team_id: i64) -> String {
    render(rsx! { Layout { title: "Overview", team_id,
        p { "Page content" }
    } })
}
