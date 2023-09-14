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
    let mut class = "ActionListItem";
    if let (Some(id), Some(selected_item_id)) = (&cx.props.id, &cx.props.selected_item_id) {
        if id == selected_item_id {
            class = "ActionListItem ActionListItem--navActive";
        }
    }
    cx.render(rsx!(
        li {
            role: "listitem",
            class: "{class}",
            a {
                href: "{cx.props.href}",
                class: "ActionListContent ActionListContent--visual16",
                span {
                    class: "ActionListItem-visual ActionListItem-visual--leading",
                    img {
                        width: "16",
                        height: "16",
                        src: "{cx.props.icon}"
                    }
                }
                span {
                    class: "ActionListItem-label",
                    "{cx.props.title}"
                }
            }
        }
    ))
}

#[derive(Props)]
pub struct NavItemWithSubItemProps<'a> {
    href: String,
    icon: &'a str,
    title: &'a str,
    selected_item_id: Option<String>,
    id: Option<String>,
    children: Element<'a>,
}

pub fn NavItemWithSubItem<'a>(cx: Scope<'a, NavItemWithSubItemProps<'a>>) -> Element {
    let mut class = "ActionListItem ActionListItem--hasSubItem";
    if let (Some(id), Some(selected_item_id)) = (&cx.props.id, &cx.props.selected_item_id) {
        if id == selected_item_id {
            class = "ActionListItem ActionListItem--navActive ActionListItem--hasSubItem";
        }
    }
    cx.render(rsx!(
        li {
            role: "listitem",
            class: "{class}",
            a {
                href: "{cx.props.href}",
                class: "ActionListContent ActionListContent--visual16",
                span {
                    class: "ActionListItem-visual ActionListItem-visual--leading",
                    img {
                        width: "16",
                        height: "16",
                        src: "{cx.props.icon}"
                    }
                }
                span {
                    class: "ActionListItem-label",
                    "{cx.props.title}"
                }
            }
            &cx.props.children
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
    let mut class = "ActionListItem--subItem ActionListItem";
    if let (Some(id), Some(selected_item_id)) = (&cx.props.id, &cx.props.selected_item_id) {
        if id == selected_item_id {
            class = "ActionListItem--subItem ActionListItem ActionListItem--navActive";
        }
    }
    cx.render(rsx!(
        li {
            role: "listitem",
            class: "{class}",
            a {
                href: "{cx.props.href}",
                class: "ActionListContent",
                span {
                    class: "ml-4 ActionListItem-label",
                    "{cx.props.title}"
                }
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
            class: "ActionListWrap",
            li {
                class: "ActionList-sectionDivider",
                h3 {
                    class: "ActionList-sectionDivider-title",
                    "{cx.props.heading}"
                }
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
