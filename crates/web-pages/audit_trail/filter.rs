#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Member;
use dioxus::prelude::*;

pub static DRAW_TRIGGER: &str = "filter-audit-drawer";

#[component]
pub fn FilterDrawer(team_users: Vec<Member>, reset_search: bool, submit_action: String) -> Element {
    rsx! {
        form {
            class: "remember",
            method: "post",
            "data-remember-reset": "{reset_search}",
            "data-remember-name": "audit",
            id: "filter-form",
            action: "{submit_action}",

            Modal {
                trigger_id: DRAW_TRIGGER,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Filter"
                    }
                    div {
                        class: "flex flex-col ",

                        Fieldset {
                            legend: "User",
                            help_text: "For which user do you want to search",
                            Select {
                                name: "user",
                                option {
                                    value: "0",
                                    "Any"
                                }
                                for user in team_users {
                                    option {
                                        value: "{user.id}",
                                        "{user.email}"
                                    }
                                }
                            }
                        }

                        Fieldset {
                            legend: "Access Type",
                            help_text: "Split between user interface and CLI usage.",
                            Select {
                                name: "access_type",
                                {super::AUDIT_ACCESS.iter().enumerate().map(|(index, access_type)| {
                                    rsx! {
                                        option {
                                            value: "{index + 1}",
                                            {super::access_type_to_string(*access_type)}
                                        }
                                    }
                                })}
                            }
                        }

                        Fieldset {
                            legend: "Action",
                            help_text: "What action did the user perform",
                            Select {
                                name: "action",
                                {super::AUDIT_ACTION.iter().enumerate().map(|(index, action_type)| {
                                    rsx! {
                                        option {
                                            value: "{index + 1}",
                                            {super::audit_action_to_string(*action_type)}
                                        }
                                    }
                                })}
                            }
                        }

                        input {
                            "type": "hidden",
                            name: "id",
                            id: "last-row-id",
                            value: "0"
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
                            "Apply Filter"
                        }
                    }
                }
            }
        }
    }
}
