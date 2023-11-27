#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct PaginationProps<'a> {
    next_page_url: Option<&'a str>,
    prev_page_url: Option<&'a str>,
}

pub fn Pagination<'a>(cx: Scope<'a, PaginationProps<'a>>) -> Element {
    cx.render(rsx!(
        nav {
            class: "paginate-container",
            "aria-label": "Pagination",
            div {
                class: "pagination",
                if let Some(url) = cx.props.prev_page_url {
                    cx.render(rsx!(
                        a {
                            class: "previous_page",
                            rel: "previous",
                            href: "{url}",
                            "Previous"
                        }
                    ))
                } else {
                    cx.render(rsx!(
                        span {
                            class: "previous_page",
                            "aria-disabled": "true",
                            "Previous"
                        }
                    ))
                }
                if let Some(url) = cx.props.next_page_url {
                    cx.render(rsx!(
                        a {
                            class: "next_page",
                            rel: "next",
                            href: "{url}",
                            "Next"
                        }
                    ))
                } else {
                    cx.render(rsx!(
                        span {
                            class: "next_page",
                            "aria-disabled": "true",
                            "Next"
                        }
                    ))
                }
            }
        }
    ))
}
