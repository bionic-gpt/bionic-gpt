#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn SectionIntroduction(
    header: String,
    subtitle: String,
    is_empty: bool,
    empty_text: String,
) -> Element {
    rsx! {
        div {
            h1 {
                class: "text-xl font-semibold",
                "{header}"
            }
            p {
                class: "text-sm text-base-content/70 mt-4",
                "{subtitle}"
            }

            if is_empty {
                Card {
                    class: "mt-4",
                    CardBody {
                        div {
                            class: "text-center",
                            p {
                                class: "text-base-content/70",
                                "{empty_text}"
                            }
                        }
                    }
                }
            }
        }
    }
}
