#![allow(non_snake_case)]
use crate::visibility_to_string;
use daisy_rsx::*;
use db::Visibility;
use dioxus::prelude::*;

#[component]
pub fn Upsert(
    id: Option<i32>,
    trigger_id: String,
    name: String,
    instructions: String,
    visibility: Visibility,
    can_set_visibility_to_company: bool,
    team_id: String,
) -> Element {
    let selected_visibility = visibility_to_string(visibility);

    rsx!(
        Modal {
            submit_action: crate::routes::projects::Upsert { team_id: team_id.clone() }.to_string(),
            trigger_id,
            ModalBody {
                class: "flex flex-col justify-between",
                h3 {
                    class: "font-bold text-lg mb-4",
                    "Project"
                }
                if let Some(id) = id {
                    input {
                        "type": "hidden",
                        value: "{id}",
                        name: "id"
                    }
                }
                Fieldset {
                    legend: "Name",
                    legend_class: "mt-4",
                    help_text: "Give your project a short, descriptive name".to_string(),
                    Input {
                        input_type: InputType::Text,
                        placeholder: "Project name".to_string(),
                        required: true,
                        value: name,
                        name: "name"
                    }
                }
                Fieldset {
                    legend: "Chat prompt",
                    legend_class: "mt-4",
                    help_text: "Add context or guidelines for chats in this project".to_string(),
                    textarea {
                        class: "textarea textarea-bordered w-full",
                        name: "instructions",
                        rows: 6,
                        "{instructions}"
                    }
                }
                Fieldset {
                    legend: "Visibility",
                    legend_class: "mt-4",
                    help_text: "Private projects are only visible to you".to_string(),
                    Select {
                        name: "visibility",
                        value: "Private",
                        SelectOption {
                            value: "{visibility_to_string(Visibility::Private)}",
                            selected_value: "{selected_visibility}",
                            {visibility_to_string(Visibility::Private)}
                        },
                        SelectOption {
                            value: "{visibility_to_string(Visibility::Team)}",
                            selected_value: "{selected_visibility}",
                            {visibility_to_string(Visibility::Team)}
                        },
                        if can_set_visibility_to_company {
                            SelectOption {
                                value: "{visibility_to_string(Visibility::Company)}",
                                selected_value: "{selected_visibility}",
                                {visibility_to_string(Visibility::Company)}
                            }
                        }
                    }
                }
                ModalAction {
                    Button {
                        class: "cancel-modal",
                        button_scheme: ButtonScheme::Warning,
                        "Cancel"
                    }
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Save"
                    }
                }
            }
        }
    )
}
