#![allow(non_snake_case)]
use db::queries::datasets::Dataset;
use dioxus::prelude::*;
use primer_rsx::{select::SelectOption, *};

#[derive(Props, PartialEq)]
pub struct DrawerProps {
    datasets: Vec<Dataset>,
    organisation_id: i32,
}

pub fn KeyDrawer(cx: Scope<DrawerProps>) -> Element {
    cx.render(rsx! {
        form {
            method: "post",
            action: "{crate::routes::document_pipelines::new_route(cx.props.organisation_id)}",
            Drawer {
                label: "New Document Pipeline",
                trigger_id: "create-api-key",
                DrawerBody {
                    div {
                        class: "d-flex flex-column",
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
                            required: true,
                            help_text: "All access via this API key will use the above dataset",
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
                        "Create Pipeline"
                    }
                }
            }
        }
    })
}
