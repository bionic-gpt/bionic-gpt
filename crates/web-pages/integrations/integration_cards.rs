#![allow(non_snake_case)]
use super::IntegrationOas3;
use daisy_rsx::*;
use dioxus::prelude::*;

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
            a {
                href: crate::routes::integrations::View {team_id, id: integration.integration.id }.to_string(),
                class: "no-underline",
                Card {
                    class: "p-3 mt-5 flex flex-row clickable",
                    img {
                        class: "border rounded p-2",
                        src: super::get_logo_url(&integration.spec.info.extensions),
                        width: "48",
                        height: "48"
                    }
                    div {
                        class: "ml-4",
                        h2 {
                            class: "font-semibold",
                            "{integration.spec.info.title.clone()}"
                        }
                        p {
                            if let Some(description) = integration.spec.info.description.clone() {
                                "{description}"
                            }
                        }
                    }
                }
            }
        }
    )
}
