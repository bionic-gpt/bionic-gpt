#![allow(non_snake_case)]
use db::queries::conversations::History;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct DrawerProps {
    trigger_id: String,
    organisation_id: i32,
    history: Vec<History>,
}

pub fn HistoryDrawer(cx: Scope<DrawerProps>) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "Your History",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
                cx.props.history.iter().map(|history| {
                    cx.render(rsx!(
                        li {
                            a {
                                href: "{crate::routes::console::conversation_route(cx.props.organisation_id, history.id)}",
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
