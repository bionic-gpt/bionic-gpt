#![allow(non_snake_case)]
use crate::routes;

use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn Form(team_id: i32, prompt_id: i32, conversation_id: i64, lock_console: bool) -> Element {
    rsx! {
        div {
            class: "position-relative w-full bottom-0 p-2 border-t color-bg-subtle",
            form {
                class: "remember w-full flex max-h-[79px]",
                method: "post",
                "data-remember-name": "console-prompt",
                "data-remember-reset": "false",
                action: routes::console::SendMessage{team_id}.to_string(),

                TextArea {
                    class: "submit-on-enter flex-1 mr-2",
                    rows: "4",
                    name: "message",
                    disabled: lock_console
                }
                div {
                    class: "flex flex-col justify-between",
                    div {
                        class: "flex flex-row",
                        label {
                            class: "my-auto mr-2",
                            "Model"
                        }
                        div {
                            input {
                                "type": "hidden",
                                name: "conversation_id",
                                value: "{conversation_id}"
                            }
                            input {
                                "type": "hidden",
                                name: "prompt_id",
                                value: "{prompt_id}"
                            }
                        }
                    }
                    Button {
                        disabled: lock_console,
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Send Message"
                    }
                }
            }
        }
    }
}
