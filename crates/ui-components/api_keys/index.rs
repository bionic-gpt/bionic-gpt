#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use db::{ApiKey, Prompt};
use dioxus::prelude::*;
use primer_rsx::{select::SelectOption, *};

#[inline_props]
pub fn Page(
    cx: Scope,
    organisation_id: i32,
    api_keys: Vec<ApiKey>,
    prompts: Vec<Prompt>,
) -> Element {
    cx.render(rsx! {
        if api_keys.is_empty() {
            cx.render(rsx! {
                Layout {
                    section_class: "normal",
                    selected_item: SideBar::ApiKeys,
                    team_id: *organisation_id,
                    title: "API Keys",
                    header: cx.render(rsx!(
                        h3 { "API Keys" }
                    )),
                    BlankSlate {
                        heading: "Looks like you don't have any API keys",
                        visual: empty_api_keys_svg.name,
                        description: "API Keys allow you to access our programming interface",
                        primary_action_drawer: ("New API Key", "create-api-key")
                    }
                }
            })
        } else {
            cx.render(rsx! {
                Layout {
                    section_class: "normal",
                    selected_item: SideBar::ApiKeys,
                    team_id: *organisation_id,
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
                    }
                }
            })
        }
        form {
            method: "post",
            action: "{crate::routes::api_keys::new_route(*organisation_id)}",
            Drawer {
                label: "New API Key",
                trigger_id: "create-api-key",
                DrawerBody {
                    div {
                        class: "flex flex-col",
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Production API Key",
                            help_text: "Give your new key a name",
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                        Select {
                            name: "prompt_id",
                            label: "Please select a prompt",
                            label_class: "mt-4",
                            help_text: "All access via this API key will use the above prompt",
                            prompts.iter().map(|prompt| rsx!(
                                SelectOption {
                                    value: "{prompt.id}",
                                    "{prompt.name}"
                                }
                            ))
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Create API Key"
                    }
                }
            }
        }
    })
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
