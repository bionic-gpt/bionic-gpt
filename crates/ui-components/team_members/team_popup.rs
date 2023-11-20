use assets::files::{button_select_svg, profile_svg};
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
                    class: "w-full",
                    DropDown {
                        direction: Direction::Bottom,
                        button_text: "{name}",
                        prefix_image_src: profile_svg.name,
                        suffix_image_src: button_select_svg.name,
                        class: "w-full",
                        cx.props.teams.iter().map(|team| rsx!(
                            DropDownLink {
                                href: "{team.1}",
                                target: "_top",
                                "{team.0}"
                            }
                        ))
                    }
                }
            })
        } else {
            cx.render(rsx! {
                turbo-frame {
                    id: "teams-popup",
                    class: "w-full",
                    div {
                        class: "flex justify-center height-full w-full items-center",
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
