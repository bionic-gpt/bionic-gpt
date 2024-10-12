use db::Prompt;
use dioxus::prelude::*;

#[component]
pub fn ModelPopup(id: i32, value: String, prompts: Vec<Prompt>) -> Element {
    rsx! {
        div {
            id: "model-selector",
            class: "select-menu relative inline-block",
            div {
                class: "selected-option cursor-pointer",
                "data-value": "{id}",
                "{value}"
            }
            div {
                class: "options hidden absolute left-0 bg-base-200 w-64 border rounded mt-1 z-10",
                for prompt in prompts {
                    div {
                        class: "option p-2 hover:bg-gray-200 cursor-pointer",
                        "data-value": "{prompt.id}",
                        span {
                            class: "",
                            "{prompt.name}"
                        }
                        p {
                            class: "text-sm",
                            "{prompt.description}"
                        }
                    }
                }
            }
        }
    }
}
