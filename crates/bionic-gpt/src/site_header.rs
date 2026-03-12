use dioxus::prelude::*;
use ssg_whiz::SiteHeader;

pub fn site_header() -> SiteHeader {
    rsx!(
        div {
            class: "bg-primary text-primary-content text-xs sm:text-sm px-3 sm:px-4 py-2 flex flex-wrap items-center justify-center gap-2 text-center",
            span {
                class: "font-semibold",
                "🎉 Zero to Agentic AI Architect Hero"
            }
            a {
                class: "inline-flex items-center gap-1 underline font-semibold hover:text-base-200",
                href: crate::routes::architect_course::Index {}.to_string(),
                "Take the course"
                span { "→" }
            }
        }
    )
}
