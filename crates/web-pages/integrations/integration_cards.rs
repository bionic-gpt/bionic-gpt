#![allow(non_snake_case)]
use super::IntegrationOas3;
use daisy_rsx::*;
use dioxus::prelude::*;
use integrations::has_oauth2_support;

#[component]
pub fn IntegrationCards(integrations: Vec<IntegrationOas3>, team_id: i32) -> Element {
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
                let has_oauth2 = has_oauth2_support(&integration.integration);
                rsx! {
                    Card {
                        class: "p-3 mt-5 flex flex-row",
                        a {
                            href: crate::routes::integrations::View {team_id, id: integration.integration.id }.to_string(),
                            class: "no-underline flex-1 min-w-0",
                            div {
                                class: "flex flex-row",
                                img {
                                    class: "border border-neutral-content rounded p-2",
                                    src: super::get_logo_url(&integration.spec.info.extensions),
                                    width: "48",
                                    height: "48"
                                }
                                div {
                                    class: "ml-4 text-sm flex flex-col justify-center flex-1 min-w-0",
                                    h2 {
                                        class: "font-semibold",
                                        "{integration.spec.info.title.clone()}"
                                    }
                                    p {
                                        class: "truncate overflow-hidden whitespace-nowrap",
                                        if let Some(description) = integration.spec.info.description.clone() {
                                            "{description}"
                                        }
                                    }
                                }
                            }
                        }
                        if has_oauth2 {
                            div {
                                class: "flex flex-col justify-center ml-4",
                                crate::button::Button {
                                    button_type: crate::button::ButtonType::Link,
                                    href: crate::routes::integrations::Connect { team_id, integration_id: integration.integration.id }.to_string(),
                                    button_scheme: crate::button::ButtonScheme::Primary,
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
