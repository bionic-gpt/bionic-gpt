#![allow(non_snake_case)]
use daisy_rsx::*;
use db::queries::models;
use db::Visibility;
use dioxus::prelude::*;

#[component]
pub fn Upsert(
    id: Option<i32>,
    trigger_id: String,
    models: Vec<models::Model>,
    name: String,
    team_id: i32,
    combine_under_n_chars: i32,
    new_after_n_chars: i32,
    _multipage_sections: bool,
    visibility: Visibility,
    can_set_visibility_to_company: bool,
) -> Element {
    rsx!(
        Modal {
            submit_action: crate::routes::datasets::Upsert{team_id}.to_string(),
            trigger_id,
            ModalBody {
                class: "flex flex-col justify-between",
                h3 {
                    class: "font-bold text-lg mb-4",
                    "Dataset"
                }
                TabContainer {
                    TabPanel {
                        checked: true,
                        name: "prompt-tabs",
                        tab_name: "Dataset",
                        div {
                            class: "flex flex-col justify-between height-full",
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
                                    help_text: "Give your new dataset a name",
                                    Input {
                                        input_type: InputType::Text,
                                        placeholder: "Dataset Name",
                                        required: true,
                                        value: name,
                                        name: "name"
                                    }
                                }

                                Fieldset {
                                    legend: "Who should be able to see this dataset?",
                                    legend_class: "mt-4",
                                    help_text: "Set to private if you don't want to share this dataset",
                                    Select {
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
                            }
                        }
                    }
                    TabPanel {
                        name: "prompt-tabs",
                        tab_name: "Advanced Configuration",

                        div {
                            class: "flex flex-col",

                            Fieldset {
                                legend: "Select the Embedding Model to use",
                                legend_class: "mt-4",
                                help_text: "Embeddings are vector stored in the database",
                                Select {
                                    name: "embeddings_model_id",
                                    for model in &models {
                                        option {
                                            value: "{model.id}",
                                            "{model.name}"
                                        }
                                    }
                                }
                            }

                            Fieldset {
                                legend: "Select the Chunking Strategy",
                                legend_class: "mt-4",
                                help_text: "These are the chunking strategies supported by unstructured.",
                                Select {
                                    name: "chunking_strategy",
                                    value: "By Title",
                                    option {
                                        value: "By Title",
                                        "By Title"
                                    }
                                }
                            }

                            Fieldset {
                                legend: "Combine Under N Chars",
                                legend_class: "mt-4",
                                help_text: "Sections will be combined if they do not exceed the specified threshold",
                                Input {
                                    input_type: InputType::Text,
                                    value: "{combine_under_n_chars}",
                                    required: true,
                                    name: "combine_under_n_chars"
                                }
                            }
                            Fieldset {
                                legend: "New After N Chars",
                                legend_class: "mt-4",
                                help_text: "Start a new section if the length of a section exceeds this value",
                                Input {
                                    input_type: InputType::Text,
                                    value: "{new_after_n_chars}",
                                    required: true,
                                    name: "new_after_n_chars"
                                }
                            }

                            Fieldset {
                                legend: "Multipage Sections",
                                legend_class: "mt-4",
                                help_text: "Allow for sections that span between pages?",
                                Select {
                                    name: "multipage_sections",
                                    option {
                                        value: "true",
                                        "Yes"
                                    }
                                    option {
                                        value: "false",
                                        "No"
                                    }
                                }
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
