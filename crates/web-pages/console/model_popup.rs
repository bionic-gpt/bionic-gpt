use assets::files::button_select_svg;
use db::Prompt;
use dioxus::prelude::*;

#[component]
pub fn ModelPopup(id: i32, value: String, prompts: Vec<Prompt>) -> Element {
    rsx! {
        form {
            id: "prompt-form",
            action: crate::routes::console::SetPrompt{}.to_string(),
            method: "post",
            enctype: "application/x-www-form-urlencoded",

            // Hidden input for the prompt ID
            input {
                type: "hidden",
                id: "prompt-id-input",
                name: "id",
                value: "{id}"
            }

            div {
                id: "model-selector",
                class: "select-menu relative inline-block",
                div {
                    class: "selected-option cursor-pointer flex flex-row gap-2",
                    "data-value": "{id}",
                    span {
                        "{value}"
                    }
                    img {
                        width: "16",
                        height: "16",
                        class: "svg-icon",
                        src: button_select_svg.name
                    }
                }
                div {
                    class: "options hidden absolute left-0 w-96 p-4 border bg-base-100 shadow-lg rounded-2xl mt-1 z-10",
                    for prompt in prompts {
                        div {
                            class: "option p-2 hover:bg-base-200 cursor-pointer",
                            "data-value": "{prompt.id}",
                            "data-action": "select-prompt",
                            span {
                                class: "font-medium",
                                "{prompt.name}"
                            }
                            p {
                                class: "text-sm font-light",
                                "{prompt.description}"
                            }
                        }
                    }
                }
            }
        }
    }
}
