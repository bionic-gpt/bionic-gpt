use crate::app_layout::{Layout, SideBar};
use assets::files::avatar_svg;
use db::queries::users::User;
use dioxus::prelude::*;
use primer_rsx::*;

struct ProfileProps {
    organisation_id: i32,
    first_name: String,
    last_name: String,
    users_name_or_email: String,
    form_action: String,
}

pub fn profile(user: User, organisation_id: i32) -> String {
    fn app(cx: Scope<ProfileProps>) -> Element {
        cx.render(rsx! {
            Layout {
                section_class: "normal",
                selected_item: SideBar::None,
                title: "Your Profile",
                team_id: cx.props.organisation_id,
                header: cx.render(rsx!(
                    h3 { "Your Profile" }
                )),
                BlankSlate {
                    heading: "Welcome, {cx.props.users_name_or_email}",
                    visual: avatar_svg.name,
                    description: "Here you can manage your account to personalize the experience",
                }

                Box {
                    BoxHeader {
                        title: "Update Your Details"
                    }
                    BoxBody {
                        form {
                            method: "post",
                            action: "{cx.props.form_action}",
                            div {
                                class: "flex flex-col",

                                Input {
                                    input_type: InputType::Text,
                                    label: "First Name",
                                    name: "first_name",
                                    value: &cx.props.first_name
                                }

                                Input {
                                    input_type: InputType::Text,
                                    label_class: "mt-3",
                                    label: "Last Name",
                                    name: "last_name",
                                    value: &cx.props.last_name
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
        })
    }

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

    let form_action = crate::routes::profile::set_details_route(organisation_id);

    crate::render(VirtualDom::new_with_props(
        app,
        ProfileProps {
            organisation_id,
            first_name,
            last_name,
            users_name_or_email,
            form_action,
        },
    ))
}
