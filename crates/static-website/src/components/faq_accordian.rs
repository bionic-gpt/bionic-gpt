use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FaqText {
    pub question: String,
    pub answer: String,
}

#[component]
pub fn Faq(questions: Vec<FaqText>, class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        section {
            class: format!("{class} lg:max-w-5xl"),
            h1 {
                class: "text-3xl font-medium text-primary title-font mb-12 text-center",
                "Frequently asked questions"
            }
            for question in questions {
                div {
                    class: "collapse collapse-arrow bg-base-200",
                    input {
                        r#type: "radio",
                        name: "faq-accordion"
                    }
                    div {
                        class: "collapse-title text-xl font-medium",
                        "{question.question}"
                    }
                    div {
                        class: "collapse-content",
                        p { "{question.answer}" }
                    }
                }
            }
        }
    }
}
