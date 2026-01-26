#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Model;
use dioxus::prelude::*;

#[component]
pub fn Form(team_id: String, models: Vec<Model>) -> Element {
    rsx!(
        form {
            action: crate::routes::rate_limits::Upsert{ team_id: team_id.clone() }.to_string(),
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

                        Fieldset {
                            legend: "API Key ID",
                            legend_class: "mt-4",
                            help_text: "We need the ID of the Api Key from the ID field",
                            Input {
                                input_type: InputType::Number,
                                placeholder: "Api Key Id i.e. 1234",
                                required: true,
                                name: "api_key_id"
                            }
                        }

                        Fieldset {
                            legend: "Tokens per Minute",
                            legend_class: "mt-4",
                            help_text: "Tokens Per minute",
                            Input {
                                input_type: InputType::Number,
                                placeholder: "Tokens per Minute e.g. 1000",
                                required: true,
                                name: "tpm_limit"
                            }
                        }

                        Fieldset {
                            legend: "Requests per Minute",
                            legend_class: "mt-4",
                            help_text: "Requests Per minute",
                            Input {
                                input_type: InputType::Number,
                                placeholder: "Requests per Minute e.g. 1000",
                                required: true,
                                name: "rpm_limit"
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
                            "Create Limit"
                        }
                    }
                }
            }
        }
    )
}
