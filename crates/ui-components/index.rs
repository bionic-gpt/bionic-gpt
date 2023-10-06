#![allow(non_snake_case)]
use assets::files::*;
use dioxus::prelude::*;

pub fn index() -> String {
    fn app(cx: Scope) -> Element {
        cx.render(rsx! {
            head {
                title {
                    "Bionic GPT"
                }
                meta {
                    charset: "utf-8"
                }
                meta {
                    "http-equiv": "X-UA-Compatible",
                    content: "IE=edge"
                }
                meta {
                    name: "viewport",
                    content: "width=device-width, initial-scale=1"
                }
                link {
                    rel: "stylesheet",
                    href: "{primer_view_components_css.name}",
                    "type": "text/css"
                }
                link {
                    rel: "stylesheet",
                    href: "{index_css.name}",
                    "type": "text/css"
                }
            }
            body {
                class: "height-full width-full d-flex flex-justify-center flex-content-center",
                div {
                    h3 {
                        class: "mt-5 mb-5",
                        "We have already setup an account for you just click Logon"
                    }
                    form {
                        class: "height-full d-flex flex-column",
                        action: "/auth/sign_in",
                        method: "post",
                        input {
                            class: "m-3",
                            name: "email",
                            value: "ian@bionic-gpt.com"
                        }
                        input {
                            class: "m-3",
                            name: "password",
                            value: "password",
                            "type": "password"
                        }
                        button {
                            class: "btn m-3",
                            "type": "submit",
                            "Logon"
                        }
                    }
                }
            }
        })
    }

    crate::render(VirtualDom::new(app))
}
