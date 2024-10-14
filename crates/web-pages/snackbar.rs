#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Snackbar() -> Element {
    rsx! {
        div {
            id: "snackbar",
            class: "fixed bottom-0 left-1/2 transform -translate-x-1/2 bg-gray-800/90 text-white flex justify-between items-center rounded-md mb-8 min-w-[288px] max-w-[568px] p-4 transition-transform duration-500 ease-out translate-y-full opacity-0",
            // Starts with translate-y-full and opacity-0 to be hidden
            p {
                class: "m-0 p-0"
            }
            button {
                class: "action bg-inherit border-none text-green-500 uppercase ml-6 p-0 min-w-min cursor-pointer",
                "DISMISS"
            }
        }
    }
}
