use crate::app_layout::{Layout, SideBar};
use db::{Dataset, Model, Prompt};
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
    name: String,
    template: String,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
    id: Option<i32>,
}

pub fn form(
    organisation_id: i32,
    prompt: Option<Prompt>,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                selected_item: SideBar::Prompts,
                team_id: cx.props.organisation_id,
                title: "Prompts",
                header: cx.render(rsx!(
                    h3 { "Prompt" }
                )),
                form {
                    class: "d-flex flex-column",
                    method: "post",
                    action: "{crate::routes::prompts::new_route(cx.props.organisation_id)}",

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

                    Select {
                        name: "model_id",
                        label: "Select the model this prompt will use for inference",
                        help_text: "The prompt will be passed to the model",
                        value: &cx.props.name,
                        required: true,
                        cx.props.models.iter().map(|model| {
                            cx.render(rsx!(
                                option {
                                    value: "{model.id}",
                                    "{model.name}"
                                }
                            ))
                        })

                    }

                    Select {
                        name: "datasets",
                        label: "Select 1 or more datasets to connect to this prompt",
                        help_text: "Connect this persona with a dataset",
                        value: &cx.props.name,
                        required: true,
                        multiple: true,
                        cx.props.datasets.iter().map(|dataset| {
                            cx.render(rsx!(
                                option {
                                    value: "{dataset.id}",
                                    "{dataset.name}"
                                }
                            ))
                        })

                    }

                    TextArea {
                        name: "template",
                        rows: "15",
                        label: "Prompt Template",
                        required: true,
                        "{cx.props.template}",
                    }
                    Button {
                        button_type: ButtonType::Submit,
                        "Submit"
                    }
                }
            }
        })
    }

    if let Some(prompt) = prompt {
        crate::render(VirtualDom::new_with_props(
            app,
            Props {
                organisation_id,
                name: prompt.name,
                template: prompt.template,
                id: Some(prompt.id),
                datasets,
                models,
            },
        ))
    } else {
        crate::render(VirtualDom::new_with_props(
            app,
            Props {
                organisation_id,
                name: "".to_string(),
                template: "The prompt below is a question to answer, a task to complete, or a conversation to respond to; decide which and write an appropriate response.
### Prompt:
{{.Input}}
### Data:
{{.Data}}
### Response:".to_string(),
                datasets,
                models,
                id: None,
            },
        ))
    }
}
