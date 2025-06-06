#![allow(non_snake_case)]
use daisy_rsx::*;
use db::History;
use dioxus::prelude::*;

#[component]
pub fn HistoryDrawer(trigger_id: String, team_id: i32, history: Vec<History>) -> Element {
    rsx! {
        Modal {
            trigger_id: &trigger_id,
            ModalBody {
                h3 {
                    class: "font-bold text-lg mb-4",
                    "Recent Chats"
                }
                ul {
                    class: "space-y-2",
                    for history in history {
                        li {
                            class: "w-full overflow-hidden truncate",
                            a {
                                class: "block p-2 hover:bg-gray-100 rounded",
                                href: crate::routes::console::Conversation{team_id, conversation_id: history.id}.to_string(),
                                {history.summary.clone()}
                            }
                        }
                    }
                }
            }
        }
    }
}
