#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn Upload(team_id: String) -> Element {
    rsx!(
        form {
            action: crate::routes::openapi_specs::Import { team_id }.to_string(),
            method: "post",
            enctype: "multipart/form-data",
            Modal {
                trigger_id: "upload-openapi-specs",
                ModalBody {
                    class: "w-[calc(100vw-2rem)] max-w-lg overflow-x-hidden",
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Upload OpenAPI Specs"
                    }
                    div {
                        class: "flex w-full min-w-0 max-w-full flex-col gap-4",
                        Fieldset {
                            class: "w-full min-w-0 max-w-full",
                            legend: "Spec file",
                            FileInput {
                                class: "box-border block w-full min-w-0 max-w-full overflow-hidden text-ellipsis",
                                name: "payload",
                                accept: ".json,.yaml,.yml,.zip,application/json,application/yaml,application/zip",
                                required: true,
                                multiple: false,
                            }
                            p {
                                class: "label block w-full min-w-0 whitespace-normal break-words",
                                "Upload one JSON or YAML spec, or a ZIP containing multiple specs in any folder structure."
                            }
                        }
                        Fieldset {
                            class: "w-full min-w-0 max-w-full",
                            legend: "Category",
                            select {
                                name: "category",
                                class: "select select-bordered min-w-0 max-w-full w-full",
                                SelectOption {
                                    value: "Application",
                                    selected_value: "Application",
                                    "Application"
                                },
                                SelectOption {
                                    value: "WebSearch",
                                    selected_value: "Application",
                                    "Web Search"
                                },
                            }
                            p {
                                class: "label block w-full min-w-0 whitespace-normal break-words",
                                "Applied to every spec in the upload."
                            }
                        }
                        Fieldset {
                            class: "w-full min-w-0 max-w-full",
                            legend: "Status",
                            label {
                                class: "flex items-center gap-2 min-h-12",
                                input {
                                    r#type: "checkbox",
                                    class: "checkbox",
                                    name: "is_active",
                                    value: "true",
                                    checked: true,
                                }
                                span { "Active" }
                            }
                            p {
                                class: "label block w-full min-w-0 whitespace-normal break-words",
                                "Applied to every spec in the upload."
                            }
                        }
                        Alert {
                            class: "w-full min-w-0 max-w-full break-words",
                            alert_color: AlertColor::Default,
                            "Maximum upload: 50 MiB. Existing specs are never overwritten."
                        }
                    }
                    ModalAction {
                        class: "flex flex-wrap",
                        Button {
                            class: "cancel-modal",
                            button_scheme: ButtonScheme::Warning,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            disabled_text: "Validating and importing specs",
                            "Upload Specs"
                        }
                    }
                }
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_upload_modal_and_supported_extensions() {
        let html = dioxus_ssr::render_element(rsx!(Upload {
            team_id: "team-1".to_string()
        }));

        assert!(html.contains("/o/team-1/openapi-specs/import"));
        assert!(html.contains("multipart/form-data"));
        assert!(html.contains(".json,.yaml,.yml,.zip"));
        assert!(html.contains("Upload OpenAPI Specs"));
        assert!(html.contains("w-[calc(100vw-2rem)] max-w-lg overflow-x-hidden"));
        assert!(html
            .contains("box-border block w-full min-w-0 max-w-full overflow-hidden text-ellipsis"));
        assert!(html.contains("label block w-full min-w-0 whitespace-normal break-words"));
        assert!(html.contains("modal-action flex flex-wrap"));
    }
}
