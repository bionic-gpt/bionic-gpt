use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use db::{ApiKey, Prompt};
use dioxus::prelude::*;
use primer_rsx::{select::SelectOption, *};

struct ApiKeysProps {
    organisation_id: i32,
    api_keys: Vec<ApiKey>,
    prompts: Vec<Prompt>,
    submit_action: String,
}

pub fn index(api_keys: Vec<ApiKey>, prompts: Vec<Prompt>, organisation_id: i32) -> String {
    fn app(cx: Scope<ApiKeysProps>) -> Element {
        cx.render(rsx! {
            if cx.props.api_keys.is_empty() {
                cx.render(rsx! {
                    Layout {
                        section_class: "normal",
                        selected_item: SideBar::ApiKeys,
                        team_id: cx.props.organisation_id,
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
                        team_id: cx.props.organisation_id,
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
                                        cx.props.api_keys.iter().map(|key| rsx!(
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
                action: "{cx.props.submit_action}",
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
                                cx.props.prompts.iter().map(|prompt| rsx!(
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

    let submit_action = crate::routes::api_keys::new_route(organisation_id);

    crate::render(VirtualDom::new_with_props(
        app,
        ApiKeysProps {
            organisation_id,
            api_keys,
            prompts,
            submit_action,
        },
    ))
}
