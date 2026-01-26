use db::queries::documents::Document;
use dioxus::prelude::*;

pub fn status(document: Document, team_id: String, first_time: bool) -> String {
    let row = rsx! {
        super::page::Row {
            team_id,
            document,
            first_time
        }
    };
    dioxus_ssr::render_element(row)
}
