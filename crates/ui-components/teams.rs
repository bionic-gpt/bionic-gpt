use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use db::Team;
use dioxus::prelude::*;
use primer_rsx::*;

struct TeamsProps {
    organisation_id: i32,
    teams: Vec<Team>,
    submit_action: String,
}

pub fn teams(teams: Vec<Team>, organisation_id: i32) -> String {
    fn app(cx: Scope<TeamsProps>) -> Element {
        cx.render(rsx! {
            Layout {
                section_class: "normal",
                selected_item: SideBar::Switch,
                team_id: cx.props.organisation_id,
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
                                cx.props.teams.iter().map(|team| rsx!(
                                    if let Some(name) = &team.organisation_name {
                                        cx.render(rsx! {
                                            tr {
                                                td {
                                                    Avatar {
                                                        name: "{name}",
                                                        avatar_type: avatar::AvatarType::Organisation
                                                    }
                                                    span {
                                                        class: "ml-2 mr-2",
                                                        "{name}"
                                                    }
                                                    if team.id != cx.props.organisation_id {
                                                        cx.render(rsx! {
                                                            a {
                                                                href: "../{team.id}",
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
                                                        avatar_type: avatar::AvatarType::Organisation
                                                    }
                                                    span {
                                                        class: "ml-2 mr-2",
                                                        "Name Not Set"
                                                    }
                                                    if team.id != cx.props.organisation_id {
                                                        cx.render(rsx! {
                                                            a {
                                                                href: "../{team.id}/team",
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
            }

            // The for to create new teams
            form {
                method: "post",
                action: "{cx.props.submit_action}",
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
        })
    }

    let submit_action = crate::routes::team::new_team_route(organisation_id);

    crate::render(VirtualDom::new_with_props(
        app,
        TeamsProps {
            organisation_id,
            teams,
            submit_action,
        },
    ))
}
