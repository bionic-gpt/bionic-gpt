#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use integrations::BionicOpenAPI;

#[component]
pub fn IntegrationCards(integrations: Vec<(BionicOpenAPI, i32)>, team_id: i32) -> Element {
    rsx!(
        h1 {
            class: "text-xl font-semibold",
            "Integrations"
        }
        p {
            "Connect external tools to retrieve data, take actions, and more."
        }
        for integration in integrations.iter() {
            {
                rsx! {
                    Card {
                        class: "p-3 mt-5 flex flex-row",
                        a {
                            href: crate::routes::integrations::View {team_id, id: integration.1 }.to_string(),
                            class: "no-underline flex-1 min-w-0",
                            div {
                                class: "flex flex-row",
                                img {
                                    class: "border border-neutral-content rounded p-2",
                                    src: integration.0.get_logo_url(),
                                    width: "48",
                                    height: "48"
                                }
                                div {
                                    class: "ml-4 text-sm flex flex-col justify-center flex-1 min-w-0",
                                    h2 {
                                        class: "font-semibold",
                                        "{integration.0.get_title()}"
                                    }
                                    p {
                                        class: "truncate overflow-hidden whitespace-nowrap",
                                        if let Some(description) = integration.0.get_description() {
                                            "{description}"
                                        }
                                    }
                                }
                            }
                        }
                        if integration.0.get_oauth2_config().is_some() {
                            div {
                                class: "flex flex-col justify-center ml-4",
                                a {
                                    class: "btn  btn-primary btn-sm ",
                                    "data-turbo": "false",
                                    href: crate::routes::integrations::Connect { team_id, integration_id: integration.1 }.to_string(),
                                    "Connect"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
