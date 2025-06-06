#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn Form(team_id: i32) -> Element {
    rsx!(
        form {
            action: crate::routes::history::Search{ team_id }.to_string(),
            method: "post",
            Modal {
                trigger_id: "search-history",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Search Chat History"
                    }
                    div {
                        class: "flex flex-col",
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Your Search",
                            help_text: "What do you want to look for?",
                            required: true,
                            label: "Search",
                            name: "search"
                        }
                    }
                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Run Search"
                        }
                    }
                }
            }
        }
    )
}
