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
    if let Some(submit_action) = cx.props.submit_action.clone() {
        cx.render(rsx!(
            form {
                action: "{submit_action}",
                method: "post",
                side-drawer {
                    class: "side_drawer flex flex-col",
                    label: cx.props.label,
                    id: cx.props.trigger_id,
                    &cx.props.children
                }
            }
        ))
    } else {
        cx.render(rsx!(
            side-drawer {
                class: "side_drawer flex flex-col",
                label: cx.props.label,
                id: cx.props.trigger_id,
                &cx.props.children
            }
        ))
    }
}

#[derive(Props)]
pub struct DrawerFooterProps<'a> {
    children: Element<'a>,
}

pub fn DrawerFooter<'a>(cx: Scope<'a, DrawerFooterProps<'a>>) -> Element {
    cx.render(rsx!(
        template {
            "slot": "footer",
            &cx.props.children
        }
    ))
}

#[derive(Props)]
pub struct DrawerBodyProps<'a> {
    children: Element<'a>,
}

pub fn DrawerBody<'a>(cx: Scope<'a, DrawerBodyProps<'a>>) -> Element {
    cx.render(rsx!(cx.render(rsx!(
        template {
            "slot": "body",
            &cx.props.children
        }
    ))))
}
