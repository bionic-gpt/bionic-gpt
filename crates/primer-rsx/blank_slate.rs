#![allow(non_snake_case)]
use super::button::{Button, ButtonScheme};
use dioxus::prelude::*;

#[derive(Props)]
pub struct BlankSlateProps<'a> {
    heading: &'a str,
    visual: &'a str,
    description: &'a str,
    primary_action: Option<(&'a str, String)>,
    primary_action_drawer: Option<(&'a str, &'a str)>,
    secondary_action: Option<(&'a str, &'a str)>,
}

pub fn BlankSlate<'a>(cx: Scope<'a, BlankSlateProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "blankslate",
            img {
                src: "{cx.props.visual}",
                width: "15%"
            }
            h2 {
                class: "blankslate-heading",
                "{cx.props.heading}"
            }
            p {
                "{cx.props.description}"
            }
            match &cx.props.primary_action {
                Some(pa) => cx.render(rsx!(
                    div {
                        class: "blankslate-action",
                        a {
                            href: "{pa.1}",
                            span {
                                class: "Button-label",
                                "{pa.0}"
                            }
                        }
                    }
                 )),
                None => None
            }
            match cx.props.primary_action_drawer {
                Some(pa) => cx.render(rsx!(
                    div {
                        class: "blankslate-action",
                        Button {
                            button_scheme: ButtonScheme::Primary,
                            drawer_trigger: "{pa.1}",
                            "{pa.0}"
                        }
                    }
                 )),
                None => None
            }
            match cx.props.secondary_action {
                Some(pa) => cx.render(rsx!(
                    div {
                        class: "blankslate-action",
                        a {
                            href: "{pa.1}",
                            "{pa.0}"
                        }
                    }
                 )),
                None => None
            }
        }
    ))
}
