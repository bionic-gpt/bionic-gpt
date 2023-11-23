#![allow(non_snake_case)]
use db::queries::conversations::History;
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
pub fn HistoryDrawer(
    cx: Scope,
    trigger_id: String,
    organisation_id: i32,
    history: Vec<History>,
) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "Your History",
            trigger_id: &trigger_id,
            DrawerBody {
                history.iter().map(|history| {
                    cx.render(rsx!(
                        li {
                            a {
                                href: "{crate::routes::console::conversation_route(*organisation_id, history.id)}",
                                history.summary.clone()
                            }
                        }
                    ))
                })
            }
            DrawerFooter {
            }
        }
    })
}
