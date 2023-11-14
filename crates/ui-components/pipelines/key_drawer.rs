#![allow(non_snake_case)]
use db::queries::datasets::Dataset;
use dioxus::prelude::*;
use primer_rsx::{select::SelectOption, *};

pub struct DrawerProps {
    submit_action: String,
    datasets: Vec<Dataset>,
}

pub fn KeyDrawer(cx: Scope<DrawerProps>) -> Element {
    cx.render(rsx! {
        form {
            method: "post",
            action: "{cx.props.submit_action}",
            Drawer {
                label: "New API Key",
                trigger_id: "create-api-key",
                DrawerBody {
                    div {
                        class: "d-flex flex-column",
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Production API Key",
                            help_text: "Give your new key a name",
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                        Select {
                            name: "dataset_id",
                            label: "Please select a dataset",
                            help_text: "All access via this API key will use the above dqataset",
                            cx.props.datasets.iter().map(|dataset| rsx!(
                                SelectOption {
                                    value: "{dataset.id}",
                                    "{dataset.name}"
                                }
                            ))
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
        }
    })
}
