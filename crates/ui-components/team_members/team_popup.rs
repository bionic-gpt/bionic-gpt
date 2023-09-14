use db::queries::organisations::GetTeams;
use db::queries::organisations::Organisation;
use dioxus::prelude::*;
use primer_rsx::*;

struct TeamPopupProps {
    teams: Vec<(String, String)>,
    organisation: Organisation,
}

pub fn team_popup(teams: Vec<GetTeams>, organisation: Organisation) -> String {
    fn app(cx: Scope<TeamPopupProps>) -> Element {
        if let Some(name) = &cx.props.organisation.name.clone() {
            cx.render(rsx! {
                turbo-frame {
                    id: "teams-popup",
                    class: "width-full",
                    SelectMenu {
                        summary: cx.render(rsx!(
                            summary {
                                class: "btn d-flex flex-justify-between width-full flex-items-center",
                                "aria-haspopup": "true",
                                span {
                                    class: "mr-2 d-flex flex-items-center",
                                    Avatar {
                                        avatar_size: AvatarSize::Small,
                                        name: "{name}",
                                        avatar_type: AvatarType::Organisation
                                    }
                                }
                                span {
                                    class: "Truncate",
                                    span {
                                        class: "Truncate-text",
                                        "{name}"
                                    }
                                }
                                span {
                                    class: "ml-2 dropdown-caret"
                                }
                            }
                        )),
                        SelectMenuModal {
                            header {
                                class: "SelectMenu-header",
                                h3 {
                                    class: "SelectMenu-title",
                                    "Your Teams"
                                }
                            }
                            SelectMenuList {
                                cx.props.teams.iter().map(|team| rsx!(
                                    a {
                                        class: "SelectMenu-item",
                                        href: "{team.1}",
                                        target: "_top",
                                        role: "menuitemcheckbox",
                                        Avatar {
                                            avatar_size: AvatarSize::Small,
                                            name: "{team.0}",
                                            avatar_type: AvatarType::Organisation
                                        }
                                        span {
                                            class: "ml-2",
                                            "{team.0}"
                                        }
                                    }
                                ))
                            }
                        }
                    }
                }
            })
        } else {
            cx.render(rsx! {
                turbo-frame {
                    id: "teams-popup",
                    class: "width-full",
                    div {
                        class: "d-flex flex-justify-center height-full width-full flex-items-center",
                        h4 {
                            "BionicGPT"
                        }
                    }
                }
            })
        }
    }

    let teams: Vec<(String, String)> = teams
        .iter()
        .filter_map(|team| {
            team.organisation_name
                .clone()
                .map(|name| (name, crate::routes::team::index_route(team.id)))
        })
        .collect();

    crate::render(VirtualDom::new_with_props(
        app,
        TeamPopupProps {
            teams,
            organisation,
        },
    ))
}
