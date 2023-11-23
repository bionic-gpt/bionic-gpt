#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
pub fn TeamNameForm(cx: Scope, submit_action: String) -> Element {
    cx.render(rsx! {
        form {
            method: "post",
            action: "{submit_action}",
            Drawer {
                label: "Set Team Name",
                trigger_id: "set-name-drawer",
                DrawerBody {
                    div {
                        class: "flex flex-col",
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Team Name",
                            help_text: "Give your new team a name",
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Set Team Name"
                    }
                }
            }
        }
    })
}
