#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct NavItemProps<'a> {
    href: String,
    icon: &'a str,
    title: &'a str,
    selected_item_id: Option<String>,
    id: Option<String>,
}

pub fn NavItem<'a>(cx: Scope<'a, NavItemProps<'a>>) -> Element {
    let mut class = "";
    if let (Some(id), Some(selected_item_id)) = (&cx.props.id, &cx.props.selected_item_id) {
        if id == selected_item_id {
            class = "active";
        }
    }
    cx.render(rsx!(
        li {
            role: "listitem",
            a {
                class: "{class}",
                href: "{cx.props.href}",
                img {
                    width: "16",
                    height: "16",
                    src: "{cx.props.icon}"
                }
                "{cx.props.title}"
            }
        }
    ))
}

#[derive(Props)]
pub struct NavSubItemProps<'a> {
    href: String,
    title: &'a str,
    selected_item_id: Option<String>,
    id: Option<String>,
}

pub fn NavSubItem<'a>(cx: Scope<'a, NavSubItemProps<'a>>) -> Element {
    let mut class = "";
    if let (Some(id), Some(selected_item_id)) = (&cx.props.id, &cx.props.selected_item_id) {
        if id == selected_item_id {
            class = "active";
        }
    }
    cx.render(rsx!(
        li {
            class: class,
            a {
                href: "{cx.props.href}",
                "{cx.props.title}"
            }
        }
    ))
}

#[derive(Props)]
pub struct NavGroupProps<'a> {
    heading: &'a str,
    content: Element<'a>,
}

pub fn NavGroup<'a>(cx: Scope<'a, NavGroupProps<'a>>) -> Element {
    cx.render(rsx!(
        ul {
            role: "list",
            class: "menu",
            li {
                class: "menu-title",
                "{cx.props.heading}"
            }
            &cx.props.content
        }
    ))
}

#[derive(Props)]
pub struct NavSubGroupProps<'a> {
    children: Element<'a>,
}

pub fn NavSubGroup<'a>(cx: Scope<'a, NavSubGroupProps<'a>>) -> Element {
    cx.render(rsx!(
        ul {
            role: "list",
            class: "ActionList ActionList--subGroup",
            &cx.props.children
        }
    ))
}
