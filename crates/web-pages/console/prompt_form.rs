#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn Form(
    team_id: i32,
    prompt_id: i32,
    conversation_id: i64,
    lock_console: bool,
    disclaimer: String,
) -> Element {
    rsx! {
        div {
            class: "mx-auto md:max-w-3xl lg:max-w-[40rem] xl:max-w-[48rem]",
            form {
                class: "remember w-full flex max-h-[79px] bg-base-200 p-2 rounded-lg",
                method: "post",
                "data-remember-name": "console-prompt",
                "data-remember-reset": "false",
                action: routes::console::SendMessage{team_id}.to_string(),

                TextArea {
                    class: "submit-on-enter flex-1 mr-2 resize-none",
                    rows: "4",
                    name: "message",
                    disabled: lock_console
                }
                div {
                    class: "flex items-center justify-center",
                    div {
                        input {
                            "type": "hidden",
                            name: "conversation_id",
                            value: "{conversation_id}"
                        }
                        input {
                            "type": "hidden",
                            id: "prompt-form-prompt-id",
                            name: "prompt_id",
                            value: "{prompt_id}"
                        }
                    }
                    button {
                        class: "flex h-8 w-8 p-2 items-center bg-primary justify-center rounded-full",
                        disabled: lock_console,
                        "type": "submit",
                        img {
                            class: "svg-icon",
                            width: "48",
                            height: "48",
                            src: submit_button_svg.name
                        }
                    }
                }
            }
            p {
                class: "text-xs text-center",
                "{disclaimer}"
            }
        }
    }
}
