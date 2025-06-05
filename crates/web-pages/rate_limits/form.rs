#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Model;
use dioxus::prelude::*;

#[component]
pub fn Form(team_id: i32, models: Vec<Model>) -> Element {
    rsx!(
        form {
            action: crate::routes::rate_limits::Upsert{ team_id }.to_string(),
            method: "post",
            Modal {
                trigger_id: "create-limit",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "New Limit"
                    }
                    div {
                        class: "flex flex-col",

                        Input {
                            label_class: "mt-4",
                            input_type: InputType::Number,
                            placeholder: "Api Key Id i.e. 1234",
                            help_text: "We need the ID of the Api Key from the ID field",
                            label: "API Key ID",
                            required: true,
                            name: "api_key_id"
                        }

                        Input {
                            label_class: "mt-4",
                            input_type: InputType::Number,
                            placeholder: "Tokens per Minute e.g. 1000",
                            help_text: "Tokens Per minute",
                            label: "Tokens per Minute",
                            required: true,
                            name: "tpm_limit"
                        }

                        Input {
                            label_class: "mt-4",
                            input_type: InputType::Number,
                            placeholder: "Requests per Minute e.g. 1000",
                            help_text: "Requests Per minute",
                            label: "Requests per Minute",
                            required: true,
                            name: "rpm_limit"
                        }
                    }
                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Create Limit"
                        }
                    }
                }
            }
        }
    )
}
