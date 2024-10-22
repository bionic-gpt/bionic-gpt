#![allow(non_snake_case)]
use daisy_rsx::*;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

#[component]
pub fn ViewDrawer(team_id: i32, prompt: Prompt, trigger_id: String) -> Element {
    rsx! {
        Drawer {
            label: "{prompt.name}",
            trigger_id,
            DrawerBody {
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Danger,
                    "Chat"
                }
            }
        }
    }
}
