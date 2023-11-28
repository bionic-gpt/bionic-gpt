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
                        class: "d-flex flex-column",

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
                            option {
                                value: "0",
                                "Any"
                            }
                            option {
                                value: "1",
                                "User Interface"
                            }
                            option {
                                value: "2",
                                "CLI"
                            }
                        }

                        Select {
                            label: "Action",
                            help_text: "What action did the user perform",
                            name: "action",
                            option {
                                value: "0",
                                "Any"
                            }
                            option {
                                value: "1",
                                "Add Member"
                            }
                            option {
                                value: "2",
                                "Delete Member"
                            }
                            option {
                                value: "3",
                                "Add Secret"
                            }
                            option {
                                value: "4",
                                "Delete Secret"
                            }
                            option {
                                value: "5",
                                "Access Secret"
                            }
                            option {
                                value: "6",
                                "New Service Account"
                            }
                            option {
                                value: "7",
                                "Delete Service Account"
                            }
                            option {
                                value: "8",
                                "Connect Service Account"
                            }
                            option {
                                value: "9",
                                "Create Invite"
                            }
                            option {
                                value: "10",
                                "Remove Team Member"
                            }
                            option {
                                value: "11",
                                "Create Vault"
                            }
                            option {
                                value: "12",
                                "Delete Vault"
                            }
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
