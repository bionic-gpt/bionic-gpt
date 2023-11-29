#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Member;
use dioxus::prelude::*;

pub static DRAW_TRIGGER: &str = "filter-audit-drawer";

#[inline_props]
pub fn FilterDrawer(
    cx: Scope,
    team_users: Vec<Member>,
    reset_search: bool,
    submit_action: String,
) -> Element {
    cx.render(rsx! {
        form {
            class: "remember",
            method: "post",
            "data-turbo": "false",
            "data-remember-reset": "{reset_search}",
            "data-remember-name": "audit",
            id: "filter-form",
            action: "{submit_action}",

            Drawer {
                label: "Filter",
                trigger_id: DRAW_TRIGGER,
                DrawerBody {
                    div {
                        class: "flex flex-col ",

                        Select {
                            label: "User",
                            help_text: "For which user do you want to search",
                            name: "user",
                            option {
                                value: "0",
                                "Any"
                            }
                            team_users.iter().map(|user| {
                                cx.render(rsx! {
                                    option {
                                        value: "{user.id}",
                                        "{user.email}"
                                    }
                                })
                            })
                        }

                        Select {
                            label: "Access Type",
                            help_text: "Split between user interface and CLI usage.",
                            name: "access_type",
                            super::AUDIT_ACCESS.iter().enumerate().map(|(index, access_type)| {
                                cx.render(rsx! {
                                    option {
                                        value: "{index + 1}",
                                        super::access_type_to_string(*access_type)
                                    }
                                })
                            })
                        }

                        Select {
                            label: "Action",
                            help_text: "What action did the user perform",
                            name: "action",
                            super::AUDIT_ACTION.iter().enumerate().map(|(index, action_type)| {
                                cx.render(rsx! {
                                    option {
                                        value: "{index + 1}",
                                        super::audit_action_to_string(*action_type)
                                    }
                                })
                            })
                        }

                        input {
                            "type": "hidden",
                            name: "id",
                            id: "last-row-id",
                            value: "0"
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Apply Filter"
                    }
                }
            }
        }
    })
}
