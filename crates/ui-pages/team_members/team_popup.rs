#![allow(non_snake_case)]
use assets::files::{button_select_svg, profile_svg};
use daisy_rsx::*;
use db::queries::teams::Team;
use dioxus::prelude::*;

#[inline_props]
pub fn Page(cx: Scope, teams: Vec<(String, String)>, team: Team) -> Element {
    if let Some(name) = &team.name.clone() {
        cx.render(rsx! {
            turbo-frame {
                id: "teams-popup",
                class: "min-w-full",
                DropDown {
                    direction: Direction::Bottom,
                    button_text: "{name}",
                    prefix_image_src: profile_svg.name,
                    suffix_image_src: button_select_svg.name,
                    class: "min-w-full",
                    strong {
                        "Switch Teams"
                    },
                    teams.iter().map(|team| rsx!(
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

pub fn team_popup(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
