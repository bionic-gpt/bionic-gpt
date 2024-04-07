#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavItemProps {
    href: String,
    icon: String,
    title: String,
    selected_item_id: Option<String>,
    id: Option<String>,
}

pub fn NavItem(props: NavItemProps) -> Element {
    let mut class = "";
    if let (Some(id), Some(selected_item_id)) = (&props.id, &props.selected_item_id) {
        if id == selected_item_id {
            class = "active";
        }
    }
    rsx!(
        li {
            role: "listitem",
            a {
                class: "{class}",
                href: "{props.href}",
                "data-turbo-frame": "main-content",
                img {
                    width: "16",
                    height: "16",
                    src: "{props.icon}"
                }
                "{props.title}"
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct NavSubItemProps {
    href: String,
    title: String,
    selected_item_id: Option<String>,
    id: Option<String>,
}

pub fn NavSubItem(props: NavSubItemProps) -> Element {
    let mut class = "";
    if let (Some(id), Some(selected_item_id)) = (&props.id, &props.selected_item_id) {
        if id == selected_item_id {
            class = "active";
        }
    }
    rsx!(
        li {
            class: class,
            a {
                href: "{props.href}",
                "{props.title}"
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct NavGroupProps {
    heading: String,
    content: Element,
}

pub fn NavGroup(props: NavGroupProps) -> Element {
    rsx!(
        ul {
            role: "list",
            class: "menu",
            li {
                class: "menu-title",
                "{props.heading}"
            }
            {props.content}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct NavSubGroupProps {
    children: Element,
}

pub fn NavSubGroup(props: NavSubGroupProps) -> Element {
    rsx!(
        ul {
            role: "list",
            class: "ActionList ActionList--subGroup",
            {props.children}
        }
    )
}
