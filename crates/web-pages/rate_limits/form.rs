#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Model;
use dioxus::prelude::*;

#[component]
pub fn Form(team_id: i32, models: Vec<Model>) -> Element {
    rsx!(
        Drawer {
            submit_action: crate::routes::rate_limits::Upsert{ team_id }.to_string(),
            label: "New Limit",
            trigger_id: "create-limit",
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Input {
                        input_type: InputType::Text,
                        placeholder: "RBAC Role Name",
                        help_text: "Which RBAC role should this limit apply to. Leave blank for any role.",
                        label: "Limit Role",
                        name: "limits_role"
                    }
                    Input {
                        input_type: InputType::Text,
                        placeholder: "Users Email",
                        help_text: "If the limit is only applied to 1 user, set the email name here.",
                        label: "Users Email",
                        name: "users_email"
                    }

                    Select {
                        name: "model_id",
                        label: "Select the model to apply this limit to",
                        label_class: "mt-4",
                        help_text: "The prompt will be passed to the model",
                        required: true,
                        SelectOption {
                            value: "all",
                            "All"
                        }
                        for model in models {
                            SelectOption {
                                value: "{model.id}",
                                "{model.name}"
                            }
                        }
                    }

                    Input {
                        input_type: InputType::Number,
                        placeholder: "Tokens per Hour e.g. 1000",
                        help_text: "Set the limit of the number of tokens per hour permitted",
                        label: "Tokens per Hour",
                        required: true,
                        name: "tokens_per_hour"
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Primary,
                    "Create Limit"
                }
            }
        }
    )
}
