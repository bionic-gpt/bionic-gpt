#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use daisy_rsx::*;
use db::rls::Rbac;
use db::TeamOwner;
use dioxus::prelude::*;

#[inline_props]
pub fn Page(
    cx: Scope,
    rbac: Rbac,
    team_id: i32,
    teams: Vec<TeamOwner>,
    submit_action: String,
) -> Element {
    cx.render(rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Switch,
            team_id: *team_id,
            rbac: rbac,
            title: "Your Teams",
            header: cx.render(rsx!(
                h3 { "Your Teams" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "create-new-team",
                    button_scheme: ButtonScheme::Primary,
                    "Create a New Team"
                }
            )),
            Box {
                class: "has-data-table",
                BoxHeader {
                    title: "Teams"
                }
                BoxBody {
                    table {
                        class: "table table-sm",
                        thead {
                            th { "Team" }
                            th {
                                class: "text-right",
                                "Team Creator" 
                            }
                        }
                        tbody {
                            teams.iter().map(|team| rsx!(
                                if let Some(name) = &team.team_name {
                                    cx.render(rsx! {
                                        tr {
                                            td {
                                                Avatar {
                                                    name: "{name}",
                                                    avatar_type: avatar::AvatarType::Team
                                                }
                                                span {
                                                    class: "ml-2 mr-2",
                                                    "{name}"
                                                }
                                                if team.id != *team_id {
                                                    cx.render(rsx! {
                                                        a {
                                                            "data-turbo-frame": "_top",
                                                            href: "{crate::routes::team::index_route(team.id)}",
                                                            "(Switch to this Team)"
                                                        }
                                                    })
                                                } else {
                                                    None
                                                }
                                            }
                                            td {
                                                class: "text-right",
                                                strong {
                                                    "{team.team_owner}"
                                                }
                                            }
                                        }
                                    })
                                } else {
                                    cx.render(rsx! {
                                        tr {
                                            td {
                                                Avatar {
                                                    avatar_type: avatar::AvatarType::Team
                                                }
                                                span {
                                                    class: "ml-2 mr-2",
                                                    "Name Not Set"
                                                }
                                                if team.id != *team_id {
                                                    cx.render(rsx! {
                                                        a {
                                                            "data-turbo-frame": "_top",
                                                            href: "{crate::routes::team::index_route(team.id)}",
                                                            "(Switch to this Team)"
                                                        }
                                                    })
                                                } else {
                                                    None
                                                }
                                            }
                                            td {
                                                class: "text-right",
                                                strong {
                                                    "{team.team_owner}"
                                                }
                                            }
                                        }
                                    })
                                }
                            ))
                        }
                    }
                }
            }

            // The for to create new teams
            form {
                method: "post",
                "data-turbo-frame": "_top",
                action: "{submit_action}",
                Drawer {
                    label: "Create a new team?",
                    trigger_id: "create-new-team",
                    DrawerBody {
                        div {
                            class: "flex flex-col",
                            Input {
                                input_type: InputType::Text,
                                placeholder: "Team Name",
                                help_text: "Give your new team a name",
                                required: true,
                                label: "Name",
                                name: "name"
                            }
                        }
                    }
                    DrawerFooter {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Create Team"
                        }
                    }
                }
            }
        }
    })
}

pub fn teams(teams: Vec<TeamOwner>, team_id: i32, rbac: Rbac) -> String {
    let submit_action = crate::routes::team::new_team_route(team_id);

    crate::render(VirtualDom::new_with_props(
        Page,
        PageProps {
            team_id,
            rbac,
            teams,
            submit_action,
        },
    ))
}
