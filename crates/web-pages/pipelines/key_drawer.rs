#![allow(non_snake_case)]
use daisy_rsx::{select::SelectOption, *};
use db::queries::datasets::Dataset;
use dioxus::prelude::*;

#[component]
pub fn KeyDrawer(datasets: Vec<Dataset>, team_id: i32) -> Element {
    rsx! {
        form {
            method: "post",
            action: crate::routes::document_pipelines::New {team_id}.to_string(),
            Modal {
                trigger_id: "create-api-key",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "New Document Pipeline"
                    }
                    div {
                        class: "flex flex-col",
                        Input {
                            input_type: InputType::Text,
                            placeholder: "My Document Pipeline",
                            help_text: "Give your new piepline a name",
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                        Select {
                            name: "dataset_id",
                            label: "Please select a dataset",
                            label_class: "mt-4",
                            required: true,
                            help_text: "All access via this API key will use the above dataset",
                            for dataset in datasets {
                                SelectOption {
                                    value: "{dataset.id}",
                                    "{dataset.name}"
                                }
                            }
                        }
                    }
                    ModalAction {
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
