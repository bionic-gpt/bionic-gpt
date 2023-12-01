#![allow(non_snake_case)]
use daisy_rsx::{select::SelectOption, *};
use db::queries::datasets::Dataset;
use dioxus::prelude::*;

#[inline_props]
pub fn KeyDrawer(cx: Scope, datasets: Vec<Dataset>, team_id: i32) -> Element {
    cx.render(rsx! {
        form {
            method: "post",
            action: "{crate::routes::document_pipelines::new_route(*team_id)}",
            Drawer {
                label: "New Document Pipeline",
                trigger_id: "create-api-key",
                DrawerBody {
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
                            datasets.iter().map(|dataset| rsx!(
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
