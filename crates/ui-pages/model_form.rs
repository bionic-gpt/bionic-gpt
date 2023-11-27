#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn Form(cx: Scope) -> Element {
    cx.render(rsx! {
        // The form to create an invitation
        form {
            method: "post",
            action: "",
            Drawer {
                label: "Add a Model",
                trigger_id: "create-model-form",
                DrawerBody {
                    div {
                        class: "flex flex-col",
                        Input {
                            input_type: InputType::Text,
                            help_text: "Give the model a name",
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                        Input {
                            input_type: InputType::Text,
                            help_text: "How many parameters does this model have",
                            required: true,
                            label: "Parameters",
                            name: "paramters"
                        }
                        Input {
                            input_type: InputType::Text,
                            help_text: "How big is the input context of this model",
                            required: true,
                            label: "Context Size",
                            name: "context_size"
                        }
                        Input {
                            input_type: InputType::Text,
                            help_text: "Where is the model located (URL)",
                            required: true,
                            label: "API Endpoint URL",
                            name: "context_size"
                        }
                        Input {
                            input_type: InputType::Text,
                            help_text: "Provide an API key if needed",
                            required: true,
                            label: "API Key",
                            name: "context_size"
                        }
                        Alert {
                            alert_color: AlertColor::Success,
                            class: "mb-3",
                            label {
                                input {
                                    "type": "checkbox",
                                    name: "admin"
                                }
                                strong {
                                    class: "ml-2",
                                    "Can this model work with confidential data"
                                }
                            }
                            p {
                                class: "note",
                                "If the model is offsite than usually you shouldn't send confidential data to it."
                            }
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Add Model"
                    }
                }
            }
        }
    })
}
