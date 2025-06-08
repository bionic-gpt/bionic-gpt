#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct IntegrationForm {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "OpenAPI specification is required"))]
    pub openapi_spec: String,
    #[serde(skip)]
    pub error: Option<String>,
}

pub fn page(team_id: i32, rbac: Rbac, integration: IntegrationForm) -> String {
    let placeholder = "{\n  &quot;openapi&quot;: &quot;3.0.3&quot;,\n  &quot;info&quot;: {\n    &quot;title&quot;: &quot;Blockchain Ticker API&quot;,\n    &quot;version&quot;: &quot;1.0.0&quot;,\n    &quot;description&quot;: &quot;Returns current Bitcoin price in various currencies.&quot;,\n    &quot;x-logo&quot;: {\n      &quot;url&quot;: &quot;data:image/svg+xml;base64,ICAgPHN2ZyB3aWR0aD0iMTAwIiBoZWlnaHQ9IjUwIj4KICAgICA8dGV4dCB4PSIyMCIgeT0iMzAiIGZvbnQtc2l6ZT0iMjAiPkI8L3RleHQ+CiAgIDwvc3ZnPg==&quot;\n    }\n  },\n  &quot;servers&quot;: [\n    {\n      &quot;url&quot;: &quot;https://blockchain.info&quot;,\n      &quot;description&quot;: &quot;Main Blockchain API server&quot;\n    }\n  ],\n  &quot;paths&quot;: {\n    &quot;/ticker&quot;: {\n      &quot;get&quot;: {\n        &quot;summary&quot;: &quot;Get Bitcoin prices by currency&quot;,\n        &quot;operationId&quot;: &quot;getTicker&quot;,\n        &quot;responses&quot;: {\n          &quot;200&quot;: {\n            &quot;description&quot;: &quot;A map of currency codes to price information&quot;,\n            &quot;content&quot;: {\n              &quot;application/json&quot;: {\n                &quot;schema&quot;: {\n                  &quot;type&quot;: &quot;object&quot;,\n                  &quot;additionalProperties&quot;: {\n                    &quot;$ref&quot;: &quot;#/components/schemas/CurrencyInfo&quot;\n                  }\n                }\n              }\n            }\n          }\n        }\n      }\n    }\n  },\n  &quot;components&quot;: {\n    &quot;schemas&quot;: {\n      &quot;CurrencyInfo&quot;: {\n        &quot;type&quot;: &quot;object&quot;,\n        &quot;properties&quot;: {\n          &quot;15m&quot;: {\n            &quot;type&quot;: &quot;number&quot;,\n            &quot;format&quot;: &quot;float&quot;\n          },\n          &quot;last&quot;: {\n            &quot;type&quot;: &quot;number&quot;,\n            &quot;format&quot;: &quot;float&quot;\n          },\n          &quot;buy&quot;: {\n            &quot;type&quot;: &quot;number&quot;,\n            &quot;format&quot;: &quot;float&quot;\n          },\n          &quot;sell&quot;: {\n            &quot;type&quot;: &quot;number&quot;,\n            &quot;format&quot;: &quot;float&quot;\n          },\n          &quot;symbol&quot;: {\n            &quot;type&quot;: &quot;string&quot;\n          }\n        },\n        &quot;required&quot;: [\n          &quot;15m&quot;,\n          &quot;last&quot;,\n          &quot;buy&quot;,\n          &quot;sell&quot;,\n          &quot;symbol&quot;\n        ]\n      }\n    }\n  }\n}";

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Integrations,
            team_id: team_id,
            rbac: rbac,
            title: "Integrations",
            header: rsx!(
                h3 { "Integrations" }
            ),

            Card {
                CardHeader {
                    title: "Integrations"
                }
                CardBody {
                    form {
                        method: "post",
                        class: "flex flex-col",
                        if let Some(id) = integration.id {
                            input {
                                "type": "hidden",
                                value: "{id}",
                                name: "id"
                            }
                        }

                        // Display error if present
                        if let Some(error) = &integration.error {
                            div {
                                class: "alert alert-error",
                                "{error}"
                            }
                        }

                        div {
                            class: "mt-4",
                            label {
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "OpenAPI Specification (JSON)"
                            }
                            TextArea {
                                class: "mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm font-mono leading-tight overflow-y-auto",
                                name: "openapi_spec",
                                rows: "20",
                                placeholder,
                                "{integration.openapi_spec}"
                            }
                            p {
                                class: "mt-1 text-sm text-gray-500",
                                "Paste your complete OpenAPI 3.0+ specification in JSON format"
                            }
                        }

                        div {
                            class: "mt-5 flex justify-between",
                            Button {
                                button_type: ButtonType::Link,
                                href: crate::routes::integrations::Index { team_id }.to_string(),
                                button_scheme: ButtonScheme::Error,
                                "Cancel"
                            }
                            Button {
                                button_type: ButtonType::Submit,
                                button_scheme: ButtonScheme::Primary,
                                "Submit"
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
