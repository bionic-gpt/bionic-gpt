#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
    href: String,
    icon: String,
    title: String,
    selected_item_id: Option<String>,
    id: Option<String>,
    class: Option<String>,
    #[props(default)]
    disabled: bool,
}

#[component]
pub fn NavItem(props: MenuProps) -> Element {
    let mut class = String::new();
    if let (Some(id), Some(selected_item_id)) = (&props.id, &props.selected_item_id) {
        if id == selected_item_id {
            class.push_str("menu-active");
        }
    }
    if props.disabled {
        if !class.is_empty() {
            class.push(' ');
        }
        class.push_str("opacity-50 pointer-events-none");
    }
    let href = if props.disabled {
        String::new()
    } else {
        props.href.clone()
    };
    rsx!(
        li {
            role: "listitem",
            a {
                class: "{class}",
                href: "{href}",
                "aria-disabled": "{props.disabled}",
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
    #[props(default)]
    disabled: bool,
}

#[component]
pub fn NavSubItem(props: NavSubItemProps) -> Element {
    let mut class = String::new();
    if let (Some(id), Some(selected_item_id)) = (&props.id, &props.selected_item_id) {
        if id == selected_item_id {
            class.push_str("active");
        }
    }
    if props.disabled {
        if !class.is_empty() {
            class.push(' ');
        }
        class.push_str("opacity-50 pointer-events-none");
    }
    let href = if props.disabled {
        String::new()
    } else {
        props.href.clone()
    };
    rsx!(
        li {
            class: "{class}",
            a {
                href: "{href}",
                "aria-disabled": "{props.disabled}",
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

#[component]
pub fn NavGroup(props: NavGroupProps) -> Element {
    rsx!(
        ul {
            role: "list",
            class: "menu w-full",
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

#[component]
pub fn NavSubGroup(props: NavSubGroupProps) -> Element {
    rsx!(
        ul {
            role: "list",
            class: "",
            {props.children}
        }
    )
}
