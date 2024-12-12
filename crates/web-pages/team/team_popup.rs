#![allow(non_snake_case)]
use assets::files::button_select_svg;
use assets::files::profile_svg;
use daisy_rsx::*;
use db::queries::teams::Team;
use dioxus::prelude::*;

pub fn popup(teams: Vec<(String, String)>, team: Team) -> String {
    let page = if let Some(name) = &team.name.clone() {
        rsx! {
            turbo-frame {
                id: "teams-popup",
                class: "min-w-full",
                DropDown {
                    direction: Direction::Bottom,
                    button_text: "{name}",
                    prefix_image_src: profile_svg.name,
                    suffix_image_src: button_select_svg.name,
                    class: "w-full",
                    strong {
                        "Switch Teams"
                    },
                    for team in teams {
                        DropDownLink {
                            href: "{team.1}",
                            target: "_top",
                            "{team.0}"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
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
        }
    };

    dioxus_ssr::render_element(page)
}
