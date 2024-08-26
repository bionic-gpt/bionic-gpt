#![allow(non_snake_case)]
use daisy_rsx::*;
use db::queries::models;
use db::Visibility;
use dioxus::prelude::*;

#[component]
pub fn Upsert(
    models: Vec<models::Model>,
    team_id: i32,
    combine_under_n_chars: i32,
    new_after_n_chars: i32,
    _multipage_sections: bool,
    visibility: Visibility,
    is_saas: bool,
) -> Element {
    rsx!(
        form {
            action: crate::routes::datasets::Upsert{team_id}.to_string(),
            method: "post",
            Drawer {
                label: "Create a new Dataset",
                trigger_id: "new-dataset-form",
                DrawerBody {
                    TabContainer {
                        TabPanel {
                            checked: true,
                            name: "prompt-tabs",
                            tab_name: "Dataset",
                            div {
                                class: "flex flex-col justify-between height-full",
                                div {
                                    class: "flex flex-col",
                                    Input {
                                        input_type: InputType::Text,
                                        placeholder: "Dataset Name",
                                        help_text: "Give your new dataset a name",
                                        required: true,
                                        label: "Name",
                                        label_class: "mt-4",
                                        name: "name"
                                    }

                                    Select {
                                        name: "visibility",
                                        label: "Who should be able to see this dataset?",
                                        label_class: "mt-4",
                                        help_text: "Set to private if you don't want to share this dataset",
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
                                        if ! is_saas {
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
                        TabPanel {
                            name: "prompt-tabs",
                            tab_name: "Advanced Configuration",

                            div {
                                class: "flex flex-col",

                                Select {
                                    name: "embeddings_model_id",
                                    label: "Select the Embedding Model to use",
                                    label_class: "mt-4",
                                    help_text: "Embeddings are vector stored in the database",
                                    for model in &models {
                                        option {
                                            value: "{model.id}",
                                            "{model.name}"
                                        }
                                    }
                                }

                                Select {
                                    name: "chunking_strategy",
                                    label: "Select the Chunking Strategy",
                                    label_class: "mt-4",
                                    help_text: "These are the chunking strategies supported by unstructured.",
                                    value: "By Title",
                                    option {
                                        value: "By Title",
                                        "By Title"
                                    }
                                }

                                Input {
                                    input_type: InputType::Text,
                                    help_text: "Sections will be combined if they do not exceed the specified threshold",
                                    value: "{combine_under_n_chars}",
                                    required: true,
                                    label: "Combine Under N Chars",
                                    label_class: "mt-4",
                                    name: "combine_under_n_chars"
                                }
                                Input {
                                    input_type: InputType::Text,
                                    help_text: "Start a new section if the length of a section exceeds this value",
                                    value: "{new_after_n_chars}",
                                    required: true,
                                    label: "New After N Chars",
                                    label_class: "mt-4",
                                    name: "new_after_n_chars"
                                }

                                Select {
                                    name: "multipage_sections",
                                    label: "Multipage Sections",
                                    label_class: "mt-4",
                                    help_text: "Allow for sections that span between pages?",
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

                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Create Dataset"
                    }
                }
            }
        }
    )
}
