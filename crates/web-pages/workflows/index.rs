#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

pub fn page(rbac: Rbac, team_id: i32) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4 max-w-3xl w-full mx-auto",
            selected_item: SideBar::Workflows,
            team_id: team_id,
            rbac: rbac,
            title: "Workflows",
            header: rsx! {
                h3 { "Workflows" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "create-workflow-form",
                    button_scheme: ButtonScheme::Primary,
                    "New Workflow"
                }
            },

            super::workflow_cards::WorkflowCards {
                workflows: super::workflow_cards::get_sample_workflows(),
                team_id: team_id
            }

            // Create workflow drawer
            form {
                method: "post",
                action: crate::routes::workflows::Upsert { team_id }.to_string(),
                Modal {
                    trigger_id: "create-workflow-form",
                    ModalBody {
                        h3 {
                            class: "font-bold text-lg mb-4",
                            "Create New Workflow"
                        }
                        div {
                            class: "flex flex-col gap-4",
                            div {
                                class: "form-group",
                                label {
                                    class: "label",
                                    "for": "name",
                                    "Workflow Name"
                                }
                                input {
                                    "type": "text",
                                    class: "input",
                                    id: "name",
                                    name: "name",
                                    placeholder: "Enter workflow name",
                                    required: true
                                }
                            }
                            div {
                                class: "form-group",
                                label {
                                    class: "label",
                                    "for": "description",
                                    "Description"
                                }
                                textarea {
                                    class: "textarea",
                                    id: "description",
                                    name: "description",
                                    placeholder: "Enter workflow description",
                                    rows: "3"
                                }
                            }
                            div {
                                class: "form-group",
                                label {
                                    class: "label",
                                    "for": "trigger_type",
                                    "Trigger Type"
                                }
                                select {
                                    class: "select",
                                    id: "trigger_type",
                                    name: "trigger_type",
                                    option { value: "file_upload", "File Upload" }
                                    option { value: "api_call", "API Call" }
                                    option { value: "schedule", "Schedule" }
                                    option { value: "webhook", "Webhook" }
                                }
                            }
                        }
                        ModalAction {
                            class: "flex gap-2 mt-4",
                            Button {
                                button_type: ButtonType::Submit,
                                button_scheme: ButtonScheme::Primary,
                                "Create Workflow"
                            }
                            button {
                                "type": "button",
                                class: "btn btn-outline cancel-modal",
                                "Cancel"
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
