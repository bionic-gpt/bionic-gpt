use super::document_card::DocumentCard;
use db::queries::documents::Document;
use dioxus::prelude::*;

pub fn status(document: Document, team_id: i32, first_time: bool) -> String {
    let row = rsx! {
        DocumentCard { document, team_id, first_time }
    };
    dioxus_ssr::render_element(row)
}
