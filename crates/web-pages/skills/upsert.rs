#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Visibility;
use dioxus::prelude::*;

#[component]
pub fn Upsert(
    id: Option<i32>,
    trigger_id: String,
    name: String,
    description: String,
    team_id: String,
    visibility: Visibility,
    can_set_visibility_to_company: bool,
    requires_file: bool,
) -> Element {
    rsx!(
        form {
            action: crate::routes::skills::Upsert { team_id }.to_string(),
            method: "post",
            enctype: "multipart/form-data",
            Modal {
                trigger_id,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Skill"
                    }
                    div {
                        class: "flex flex-col",
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
                            help_text: "Use a short name for the skill folder.",
                            Input {
                                input_type: InputType::Text,
                                class: "w-full",
                                placeholder: "Skill name",
                                required: true,
                                value: name,
                                name: "name"
                            }
                        }
                        Fieldset {
                            legend: "Description",
                            legend_class: "mt-4",
                            help_text: "Tell the model when this skill should be used.",
                            TextArea {
                                class: "w-full",
                                name: "description",
                                rows: "4",
                                placeholder: "What this skill helps with",
                                "{description}"
                            }
                        }
                        Fieldset {
                            legend: "Visibility",
                            legend_class: "mt-4",
                            help_text: "Choose who can use this skill in chat.",
                            Select {
                                class: "w-full",
                                name: "visibility",
                                value: "Private",
                                SelectOption {
                                    value: "{crate::visibility_to_string(Visibility::Private)}",
                                    selected_value: "{crate::visibility_to_string(visibility)}",
                                    {crate::visibility_to_string(Visibility::Private)}
                                },
                                SelectOption {
                                    value: "{crate::visibility_to_string(Visibility::Team)}",
                                    selected_value: "{crate::visibility_to_string(visibility)}",
                                    {crate::visibility_to_string(Visibility::Team)}
                                },
                                if can_set_visibility_to_company {
                                    SelectOption {
                                        value: "{crate::visibility_to_string(Visibility::Company)}",
                                        selected_value: "{crate::visibility_to_string(visibility)}",
                                        {crate::visibility_to_string(Visibility::Company)}
                                    }
                                }
                            }
                        }
                        Fieldset {
                            legend: "Files",
                            legend_class: "mt-4",
                            help_text: "Upload a SKILL.md file or a .zip folder containing SKILL.md.",
                            FileInput {
                                class: "w-full",
                                name: "payload",
                                required: requires_file,
                                multiple: false
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
                            "Save Skill"
                        }
                    }
                }
            }
        }
    )
}
