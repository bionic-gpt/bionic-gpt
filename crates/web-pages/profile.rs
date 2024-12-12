#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::avatar_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::users::User;
use dioxus::prelude::*;

#[component]
fn Page(
    team_id: i32,
    rbac: Rbac,
    first_name: String,
    last_name: String,
    users_name_or_email: String,
    form_action: String,
) -> Element {
    rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::None,
            title: "Your Profile",
            team_id: team_id,
            rbac: rbac,
            header: rsx!(
                h3 { "Your Profile" }
            ),
            BlankSlate {
                heading: "Welcome, {users_name_or_email}",
                visual: avatar_svg.name,
                description: "Here you can manage your account to personalize the experience",
            }

            Card {
                CardHeader {
                    title: "Update Your Details"
                }
                CardBody {
                    form {
                        method: "post",
                        "data-turbo-frame": "_top",
                        action: "{form_action}",
                        div {
                            class: "flex flex-col",

                            Input {
                                input_type: InputType::Text,
                                label: "First Name",
                                name: "first_name",
                                value: first_name
                            }

                            Input {
                                input_type: InputType::Text,
                                label_class: "mt-3",
                                label: "Last Name",
                                name: "last_name",
                                value: last_name
                            }

                            Button {
                                class: "mt-3",
                                button_type: ButtonType::Submit,
                                button_scheme: ButtonScheme::Primary,
                                "Update Profile"
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn profile(user: User, team_id: i32, rbac: Rbac) -> String {
    let (mut first_name, mut last_name) = ("".to_string(), "".to_string());
    if let (Some(first), Some(last)) = (user.first_name, user.last_name) {
        first_name = first;
        last_name = last;
    }

    let users_name_or_email = if !first_name.is_empty() {
        format!("{} {}", first_name, last_name)
    } else {
        user.email
    };

    let form_action = crate::routes::profile::SetDetails { team_id }.to_string();

    let page = rsx! {
        Page {
            team_id,
            rbac,
            first_name,
            last_name,
            users_name_or_email,
            form_action
        }
    };
    crate::render(page)
}
