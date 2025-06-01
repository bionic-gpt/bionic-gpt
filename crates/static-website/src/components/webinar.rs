#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn WebinarHeader() -> Element {
    rsx! {
        div {
            class: "bg-linear-to-r from-blue-500 to-purple-600 text-white py-2 px-2 text-center whitespace-nowrap",
            h1 {
                class: "text-md font-bold inline text-white",
                "Join our No Code Enterprise RAG webinar"
            }
            a {
                class: "inline-block bg-white text-blue-500 font-semibold py-1 px-3 rounded-full shadow-md hover:bg-gray-100 transition duration-300 ml-4",
                href: "https://www.linkedin.com/events/7249357198881886208/comments/",
                "Reserve Your Spot"
            }
        }
    }
}
