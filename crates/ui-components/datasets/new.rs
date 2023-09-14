#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props {
    pub organisation_id: i32,
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
                        class: "d-flex flex-column",
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Dataset Name",
                            help_text: "Give your new dataset a name",
                            required: true,
                            label: "Name",
                            name: "name"
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
