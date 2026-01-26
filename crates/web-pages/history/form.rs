#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn Form(team_id: String) -> Element {
    rsx!(
        form {
            action: crate::routes::history::Search{ team_id: team_id.clone() }.to_string(),
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
                        Fieldset {
                            legend: "Search",
                            help_text: "What do you want to look for?",
                            Input {
                                input_type: InputType::Text,
                                placeholder: "Your Search",
                                required: true,
                                name: "search"
                            }
                        }
                    }
                    ModalAction {
                        Button {
                            class: "cancel-modal",
                            button_scheme: ButtonScheme::Warning,
                            button_size: ButtonSize::Small,
                            "Cancel"
                        }
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
