#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn Upsert(
    id: Option<i32>,
    trigger_id: String,
    name: String,
    description: String,
    team_id: String,
) -> Element {
    rsx!(
        Modal {
            submit_action: crate::routes::categories::Upsert { team_id: team_id.clone() }.to_string(),
            trigger_id,
            ModalBody {
                class: "flex flex-col gap-4",
                h3 { class: "font-bold text-lg mb-4", "Category" }
                if let Some(id) = id {
                    input { "type": "hidden", name: "id", value: "{id}" }
                }
                Fieldset {
                    legend: "Name",
                    Input {
                        input_type: InputType::Text,
                        name: "name",
                        value: name,
                        required: true,
                    }
                }
                Fieldset {
                    legend: "Description",
                    TextArea { name: "description", rows: "4", "{description}" }
                }
                ModalAction {
                    Button { class: "cancel-modal", button_scheme: ButtonScheme::Warning, "Cancel" }
                    Button { button_type: ButtonType::Submit, button_scheme: ButtonScheme::Primary, "Save" }
                }
            }
        }
    )
}
