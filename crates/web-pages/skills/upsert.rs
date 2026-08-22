#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Visibility;
use dioxus::prelude::*;

#[component]
pub fn Upsert(
    id: Option<i32>,
    trigger_id: String,
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
                            class: "min-w-0",
                            legend: "Files",
                            legend_class: "mt-4",
                            FileInput {
                                class: "min-w-0 max-w-full",
                                name: "payload",
                                required: requires_file,
                                multiple: false
                            }
                            p {
                                class: "label block w-full min-w-0 whitespace-normal break-words",
                                "Upload a SKILL.md file or a .zip folder containing SKILL.md."
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
