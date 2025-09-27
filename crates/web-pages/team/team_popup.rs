#![allow(non_snake_case)]
use assets::files::{bionic_logo_svg, button_select_svg, profile_svg};
use daisy_rsx::*;
use db::queries::teams::Team;
use db::Licence;
use dioxus::prelude::*;
use std::borrow::Cow;

pub fn popup(teams: Vec<(String, String)>, team: Team) -> String {
    let licence = Licence::global();

    let app_name: Cow<'static, str> = if licence.app_name.is_empty() {
        Cow::Borrowed("Bionic")
    } else {
        Cow::Owned(licence.app_name.clone())
    };

    let app_logo_src: Cow<'static, str> = if licence.app_logo_svg.is_empty() {
        Cow::Borrowed(bionic_logo_svg.name)
    } else {
        Cow::Owned(format!(
            "data:image/svg+xml;base64,{}",
            licence.app_logo_svg
        ))
    };

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
                    class: "flex gap-2 height-full w-full items-center",
                    img {
                        height: "16",
                        width: "16",
                        src: "{app_logo_src}"
                    }
                    h4 {
                        "{app_name}"
                    }
                }
            }
        }
    };

    dioxus_ssr::render_element(page)
}
