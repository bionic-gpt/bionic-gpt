use dioxus::prelude::*;

#[component]
pub fn Faq() -> Element {
    rsx! {
        section {
            class: "lg:max-w-5xl mt-24",
            h1 {
                class: "text-3xl font-medium text-primary title-font mb-12 text-center",
                "Frequently asked questions"
            }
            div {
                class: "collapse collapse-arrow bg-base-200",
                input {
                    r#type: "radio",
                    name: "my-accordion-2",
                    checked: "checked"
                }
                div {
                    class: "collapse-title text-xl font-medium",
                    "Click to open this one and close others"
                }
                div {
                    class: "collapse-content",
                    p { "hello" }
                }
            }
            div {
                class: "collapse collapse-arrow bg-base-200",
                input {
                    r#type: "radio",
                    name: "my-accordion-2"
                }
                div {
                    class: "collapse-title text-xl font-medium",
                    "Click to open this one and close others"
                }
                div {
                    class: "collapse-content",
                    p { "hello" }
                }
            }
            div {
                class: "collapse collapse-arrow bg-base-200",
                input {
                    r#type: "radio",
                    name: "my-accordion-2"
                }
                div {
                    class: "collapse-title text-xl font-medium",
                    "Click to open this one and close others"
                }
                div {
                    class: "collapse-content",
                    p { "hello" }
                }
            }
        }
    }
}
