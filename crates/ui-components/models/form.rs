use crate::app_layout::{Layout, SideBar};
use db::queries::models::Model;
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
    name: String,
    base_url: String,
    template: String,
    billion_parameters: i32,
    context_size_bytes: i32,
    id: Option<i32>,
}

pub fn form(organisation_id: i32, model: Option<Model>) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                selected_item: SideBar::Models,
                team_id: cx.props.organisation_id,
                title: "Prompts",
                header: cx.render(rsx!(
                    h3 { "Prompt" }
                )),
                form {
                    class: "d-flex flex-column",
                    method: "post",
                    action: "{crate::routes::models::new_route(cx.props.organisation_id)}",

                    if let Some(id) = cx.props.id {
                        cx.render(rsx!(
                            input {
                                "type": "hidden",
                                value: "{id}",
                                name: "id"
                            }
                        ))
                    }

                    Input {
                        input_type: InputType::Text,
                        name: "name",
                        label: "Prompt Name",
                        help_text: "Make the name memorable and imply it's usage.",
                        value: &cx.props.name,
                        required: true
                    }

                    Input {
                        input_type: InputType::Text,
                        name: "base_url",
                        label: "The Base URL of the model",
                        help_text: "The URL location of the OpenAI compatible API",
                        value: &cx.props.base_url,
                        required: true
                    }

                    TextArea {
                        name: "template",
                        label: "Prompt Template",
                        required: true,
                        "{cx.props.template}",
                    }

                    Input {
                        input_type: InputType::Number,
                        name: "billion_parameters",
                        label: "How many billion parameters is the model",
                        help_text: "This is used only for information purposes.",
                        value: "{cx.props.billion_parameters}",
                        required: true
                    }

                    Input {
                        input_type: InputType::Number,
                        name: "context_size_bytes",
                        label: "Context Size (bytes)",
                        help_text: "How much data can be passed to the prompt",
                        value: "{cx.props.context_size_bytes}",
                        required: true
                    }

                    Button {
                        button_type: ButtonType::Submit,
                        "Submit"
                    }
                }
            }
        })
    }

    if let Some(model) = model {
        crate::render(VirtualDom::new_with_props(
            app,
            Props {
                organisation_id,
                name: model.name,
                base_url: model.base_url,
                template: model.template,
                billion_parameters: model.billion_parameters,
                context_size_bytes: model.context_size_bytes,
                id: Some(model.id),
            },
        ))
    } else {
        crate::render(VirtualDom::new_with_props(
            app,
            Props {
                organisation_id,
                name: "".to_string(),
                base_url: "".to_string(),
                template: "".to_string(),
                billion_parameters: 7,
                context_size_bytes: 2048,
                id: None,
            },
        ))
    }
}
