#![allow(non_snake_case)]
use assets::files::{button_select_svg, profile_svg};
use daisy_rsx::*;
use dioxus::prelude::*;

#[inline_props]
pub fn ProfilePopup(
    cx: Scope,
    email: String,
    first_name: String,
    last_name: String,
    team_id: i32,
) -> Element {
    let user_name_or_email = if !first_name.is_empty() || !last_name.is_empty() {
        format!("{} {}", cx.props.first_name, cx.props.last_name)
    } else {
        email.to_string()
    };

    cx.render(rsx! {
        DropDown {
            direction: Direction::Top,
            button_text: &user_name_or_email,
            prefix_image_src: profile_svg.name,
            suffix_image_src: button_select_svg.name,
            class: "w-full",
            strong {
                "Theme"
            }
            DropDownLink {
                href: "#light",
                class: "theme",
                "Light Theme"
            }
            DropDownLink {
                href: "#dark",
                class: "theme",
                "Dark Theme"
            }
            DropDownLink {
                href: "#mixed",
                class: "theme",
                "Mixed Theme"
            }
            strong {
                "Profile"
            }
            DropDownLink {
                href: "{crate::routes::profile::index_route(*team_id)}",
                target: "_top",
                "Profile"
            }
            DropDownLink {
                href: "#",
                target: "_top",
                drawer_trigger: "logout-drawer".to_string(),
                "Log Out"
            }
        }
    })
}
