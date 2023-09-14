#![allow(non_snake_case)]
use db::Role;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct RoleProps<'a> {
    pub role: &'a Role,
}

pub fn Role<'a>(cx: Scope<'a, RoleProps<'a>>) -> Element {
    match cx.props.role {
        Role::Administrator => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Administrator"
            }
        )),
        Role::Collaborator => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Attention,
                "Collaborator"
            }
        )),
        Role::SystemAdministrator => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Attention,
                "System Administrator"
            }
        )),
    }
}
