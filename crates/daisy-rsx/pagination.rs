#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    next_page_url: Option<String>,
    prev_page_url: Option<String>,
}

pub fn Pagination(props: PaginationProps) -> Element {
    rsx!(
        nav {
            class: "paginate-container",
            "aria-label": "Pagination",
            div {
                class: "pagination",
                if let Some(url) = props.prev_page_url {
                        a {
                            class: "previous_page",
                            rel: "previous",
                            href: "{url}",
                            "Previous"
                        }
                } else {
                    span {
                        class: "previous_page",
                        "aria-disabled": "true",
                        "Previous"
                    }
                }
                if let Some(url) = props.next_page_url {
                        a {
                            class: "next_page",
                            rel: "next",
                            href: "{url}",
                            "Next"
                        }
                } else {
                        span {
                            class: "next_page",
                            "aria-disabled": "true",
                            "Next"
                        }
                }
            }
        }
    )
}
