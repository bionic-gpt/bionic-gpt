#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct DrawerProps<'a> {
    trigger_id: &'a str,
    label: &'a str,
    children: Element<'a>,
    submit_action: Option<String>,
}

pub fn Drawer<'a>(cx: Scope<'a, DrawerProps<'a>>) -> Element {
    let action = if let Some(action) = &cx.props.submit_action {
        action.to_string()
    } else {
        "".to_string()
    };
    cx.render(rsx!(
        form {
            action: "{action}",
            method: "post",
            div {
                div {
                    class: "side-drawer flex flex-col",
                    id: cx.props.trigger_id,
                    div {
                        class: "drawer__overlay",
                        tabindex: "-1"
                    }
                    div {
                        class: "drawer__panel",
                        header {
                            class: "drawer__header",
                            h4 {
                                class: "drawer__title",
                                "{cx.props.label}"
                            }
                            a {
                                href: "#",
                                class: "drawer__close",
                                "X"
                            }
                        }
                        &cx.props.children
                    }
                }
            }
        }
    ))
}

#[derive(Props)]
pub struct DrawerFooterProps<'a> {
    children: Element<'a>,
}

pub fn DrawerFooter<'a>(cx: Scope<'a, DrawerFooterProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "drawer__footer",
            &cx.props.children
        }
    ))
}

#[derive(Props)]
pub struct DrawerBodyProps<'a> {
    children: Element<'a>,
    class: Option<&'a str>,
}

pub fn DrawerBody<'a>(cx: Scope<'a, DrawerBodyProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let class = format!("drawer__body {}", class);
    cx.render(rsx!(cx.render(rsx!(
        div {
            class: "{class}",
            &cx.props.children
        }
    ))))
}
