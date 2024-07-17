#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn Form(team_id: i32) -> Element {
    rsx!(
        Drawer {
            submit_action: crate::routes::api_keys::New{ team_id }.to_string(),
            label: "Search Chat History",
            trigger_id: "create-api-key",
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Input {
                        input_type: InputType::Text,
                        placeholder: "Production API Key",
                        help_text: "Give your new key a name",
                        required: true,
                        label: "Name",
                        name: "name"
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Primary,
                    "Create API Key"
                }
            }
        }
    )
}
