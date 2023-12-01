#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::{ApiKey, Prompt};
use dioxus::prelude::*;

#[inline_props]
pub fn Page(cx: Scope, team_id: i32, api_keys: Vec<ApiKey>, prompts: Vec<Prompt>) -> Element {
    cx.render(rsx! {
        if api_keys.is_empty() {
            cx.render(rsx! {
                Layout {
                    section_class: "normal",
                    selected_item: SideBar::ApiKeys,
                    team_id: *team_id,
                    title: "API Keys",
                    header: cx.render(rsx!(
                        h3 { "API Keys" }
                    )),
                    BlankSlate {
                        heading: "Looks like you don't have any API keys",
                        visual: empty_api_keys_svg.name,
                        description: "API Keys allow you to access our programming interface",
                        primary_action_drawer: ("New API Key", "create-api-key")
                    },
                    super::form::Form {
                        team_id: *team_id,
                        prompts: prompts.clone()
                    }
                }
            })
        } else {
            cx.render(rsx! {
                Layout {
                    section_class: "normal",
                    selected_item: SideBar::ApiKeys,
                    team_id: *team_id,
                    title: "API Keys",
                    header: cx.render(rsx!(
                        h3 { "API Keys" }
                        Button {
                            drawer_trigger: "create-api-key",
                            button_scheme: ButtonScheme::Primary,
                            "Add Key"
                        }
                    )),
                    Box {
                        class: "has-data-table",
                        BoxHeader {
                            title: "API Keys"
                        }
                        BoxBody {
                            table {
                                class: "table table-sm",
                                thead {
                                    th { "Name" }
                                    th { "API Key" }
                                    th { "Prompt" }
                                    th {
                                        class: "text-right",
                                        "Action"
                                    }
                                }
                                tbody {
                                    api_keys.iter().map(|key| rsx!(
                                        tr {
                                            td {
                                                "{key.name}"
                                            }
                                            td {
                                                Input {
                                                    value: &key.api_key,
                                                    name: "api_key"
                                                }
                                            }
                                            td {
                                                "{key.prompt_name}"
                                            }
                                            td {
                                                class: "text-right",
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "...",
                                                    DropDownLink {
                                                        href: "#",
                                                        target: "_top",
                                                        "Not Implemented"
                                                    }
                                                }
                                            }
                                        }
                                    ))
                                }
                            }
                        }
                    },
                    // Drawers have to be fairly high up in the hierarchy or they
                    // get missed off in turbo::load
                    super::form::Form {
                        team_id: *team_id,
                        prompts: prompts.clone()
                    }
                }
            })
        }
    })
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
