#![allow(non_snake_case)]
use daisy_rsx::{select::SelectOption, *};
use db::queries::datasets::Dataset;
use dioxus::prelude::*;

#[component]
pub fn KeyDrawer(datasets: Vec<Dataset>, team_id: String) -> Element {
    rsx! {
        form {
            method: "post",
            action: crate::routes::document_pipelines::New {team_id: team_id.clone()}.to_string(),
            Modal {
                trigger_id: "create-api-key",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "New Document Pipeline"
                    }
                    div {
                        class: "flex flex-col",
                        Fieldset {
                            legend: "Name",
                            help_text: "Give your new piepline a name",
                            Input {
                                input_type: InputType::Text,
                                placeholder: "My Document Pipeline",
                                required: true,
                                name: "name"
                            }
                        }
                        Fieldset {
                            legend: "Please select a dataset",
                            legend_class: "mt-4",
                            help_text: "All access via this API key will use the above dataset",
                            Select {
                                name: "dataset_id",
                                required: true,
                                for dataset in datasets {
                                    SelectOption {
                                        value: "{dataset.id}",
                                        "{dataset.name}"
                                    }
                                }
                            }
                        }
                    }
                    ModalAction {
                        Button {
                            class: "cancel-modal",
                            button_scheme: ButtonScheme::Warning,
                            button_size: ButtonSize::Small,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Create Pipeline"
                        }
                    }
                }
            }
        }
    }
}
