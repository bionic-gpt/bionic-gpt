#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Prompt;
use dioxus::prelude::*;

#[component]
pub fn Form(team_id: i32, prompts: Vec<Prompt>) -> Element {
    rsx!(
        Drawer {
            submit_action: crate::routes::api_keys::New{ team_id }.to_string(),
            label: "Enter Licence",
            trigger_id: "create-licence",
            DrawerBody {
                div {
                }
            }
            DrawerFooter {
            }
        }
    )
}
