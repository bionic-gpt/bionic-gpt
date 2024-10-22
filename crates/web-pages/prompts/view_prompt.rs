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
                h2 {
                    class: "text-center text-2xl font-semibold",
                    "{prompt.name}"
                }
                p {
                    class: "mt-6 text-center text-sm text-token-text-tertiary",
                    "Created by {prompt.created_by}"
                }
                p {
                    class: "mt-6 text-center",
                    "{prompt.description}"
                }
            }
            DrawerFooter {
                a {
                    class: "btn btn-primary btn-sm w-full",
                    href: crate::routes::prompts::NewChat{team_id, prompt_id: prompt.id}.to_string(),
                    "Start a Chat"
                }
            }
        }
    }
}
