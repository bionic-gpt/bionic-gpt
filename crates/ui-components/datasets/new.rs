#![allow(non_snake_case)]
use db::queries::models;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct Props {
    pub models: Vec<models::Model>,
    pub organisation_id: i32,
    pub combine_under_n_chars: i32,
    pub new_after_n_chars: i32,
    pub multipage_sections: bool,
}

pub fn New(cx: Scope<Props>) -> Element {
    cx.render(rsx!(
        form {
            action: "{crate::routes::datasets::new_route(cx.props.organisation_id)}",
            method: "post",
            Drawer {
                label: "Create a new Dataset",
                trigger_id: "new-dataset-form",
                DrawerBody {
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
                                name: "name"
                            }

                            Select {
                                name: "visibility",
                                label: "Who should be able to see this dataset?",
                                help_text: "Set to private if you don't want to share this dataset",
                                value: "Private",
                                option {
                                    value: "Private",
                                    "Just Me"
                                },
                                option {
                                    value: "Team",
                                    "Team"
                                }
                            }
                        }

                        div {
                            class: "border flex flex-col p-2",
                            strong {
                                class: "mb-2",
                                "Advanced Configuration"
                            }

                            Select {
                                name: "embeddings_model_id",
                                label: "Select the Embedding Model to use",
                                help_text: "Embeddings are vector stored in the database",
                                for model in &cx.props.models {
                                    cx.render(rsx!(
                                        option {
                                            value: "{model.id}",
                                            "{model.name}"
                                        }
                                    ))
                                }
                            }

                            Select {
                                name: "chunking_strategy",
                                label: "Select the Chunking Strategy",
                                help_text: "These are the chunking strategies supported by unstructured.",
                                value: "By Title",
                                option {
                                    value: "By Title",
                                    "By Title"
                                }
                            }

                            Input {
                                input_type: InputType::Text,
                                help_text: "combine_under_n_chars",
                                value: "{cx.props.combine_under_n_chars}",
                                required: true,
                                label: "combine_under_n_chars",
                                name: "combine_under_n_chars"
                            }
                            Input {
                                input_type: InputType::Text,
                                help_text: "new_after_n_chars",
                                value: "{cx.props.new_after_n_chars}",
                                required: true,
                                label: "new_after_n_chars",
                                name: "new_after_n_chars"
                            }

                            Select {
                                name: "multipage_sections",
                                label: "multipage_sections",
                                help_text: "multipage_sections",
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

                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Create Dataset"
                    }
                }
            }
        }
    ))
}
