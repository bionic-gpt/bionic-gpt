#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn Form(team_id: i32) -> Element {
    rsx!(
        Drawer {
            submit_action: crate::routes::history::Search{ team_id }.to_string(),
            label: "Search Chat History",
            trigger_id: "search-history",
            DrawerBody {
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
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Primary,
                    "Run Search"
                }
            }
        }
    )
}
