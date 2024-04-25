#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use db::TeamOwner;
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32, teams: Vec<TeamOwner>) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Switch,
            team_id: team_id,
            rbac: rbac,
            title: "Your Teams",
            header: rsx!(
                h3 { "Your Teams" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "create-new-team",
                    button_scheme: ButtonScheme::Primary,
                    "Create a New Team"
                }
            ),
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
                            for team in teams {

                                if let Some(name) = &team.team_name {
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
                                            if team.id != team_id {
                                                a {
                                                    "data-turbo-frame": "_top",
                                                    href: crate::routes::team::Index{ team_id: team.id }.to_string(),
                                                    "(Switch to this Team)"
                                                }
                                            }
                                        }
                                        td {
                                            class: "text-right",
                                            strong {
                                                "{team.team_owner}"
                                            }
                                        }
                                    }
                                } else {
                                    tr {
                                        td {
                                            Avatar {
                                                avatar_type: avatar::AvatarType::Team
                                            }
                                            span {
                                                class: "ml-2 mr-2",
                                                "Name Not Set"
                                            }
                                            if team.id != team_id {
                                                a {
                                                    "data-turbo-frame": "_top",
                                                    href: crate::routes::team::Index{ team_id: team.id }.to_string(),
                                                    "(Switch to this Team)"
                                                }
                                            }
                                        }
                                        td {
                                            class: "text-right",
                                            strong {
                                                "{team.team_owner}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // The for to create new teams
            form {
                method: "post",
                "data-turbo-frame": "_top",
                action: crate::routes::team::New{team_id}.to_string(),
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
    }
}
