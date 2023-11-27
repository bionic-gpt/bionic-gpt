#![allow(non_snake_case)]
use assets::files::{button_select_svg, profile_svg};
use daisy_rsx::*;
use db::queries::users::User;
use dioxus::prelude::*;

#[inline_props]
fn Page(cx: Scope, user_name_or_email: String, profile_url: String) -> Element {
    cx.render(rsx! {
        turbo-frame {
            class: "full-width",
            id: "profile-popup",
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
                    href: &profile_url,
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
        }
    })
}

pub fn profile_popup(user: User, organisation_id: i32) -> String {
    let name = if user.first_name.is_some() && user.last_name.is_some() {
        format!("{} {}", user.first_name.unwrap(), user.last_name.unwrap())
    } else {
        user.email
    };

    crate::render(VirtualDom::new_with_props(
        Page,
        PageProps {
            user_name_or_email: name,
            profile_url: crate::routes::profile::index_route(organisation_id),
        },
    ))
}
