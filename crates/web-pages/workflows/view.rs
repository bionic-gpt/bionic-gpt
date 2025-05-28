#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use dioxus::prelude::*;

use super::workflow_cards::{get_workflow_icon, Workflow};

/// Generate workflow-specific example prompts based on workflow name/category
fn get_workflow_example_prompts(workflow: &Workflow) -> Vec<String> {
    match workflow.name.as_str() {
        name if name.contains("LinkedIn") => vec![
            "Generate leads from LinkedIn using this profile data: [profile_url]".to_string(),
            "Analyze LinkedIn engagement and identify potential leads: [post_data]".to_string(),
            "Extract contact information from LinkedIn profiles: [search_criteria]".to_string(),
        ],
        name if name.contains("support emails") || name.contains("Triage") => vec![
            "Triage this support email and categorize by priority: [email_content]".to_string(),
            "Route this customer inquiry to the appropriate team: [inquiry_details]".to_string(),
            "Generate automated response for common support issues: [issue_type]".to_string(),
        ],
        name if name.contains("CRM") || name.contains("leads") => vec![
            "Transfer this lead to CRM with qualification score: [lead_data]".to_string(),
            "Update lead status and assign to sales rep: [lead_info]".to_string(),
            "Generate lead scoring based on engagement: [interaction_data]".to_string(),
        ],
        name if name.contains("RFP") || name.contains("Proposal") => vec![
            "Generate a proposal response for this RFP: [rfp_requirements]".to_string(),
            "Create customized proposal sections: [client_needs]".to_string(),
            "Auto-populate proposal templates with client data: [client_profile]".to_string(),
        ],
        _ => vec![
            "Execute this workflow with the provided data: [input_data]".to_string(),
            "Run workflow step by step with these parameters: [parameters]".to_string(),
            "Trigger workflow automation for: [trigger_event]".to_string(),
        ],
    }
}

pub fn view(team_id: i32, rbac: Rbac, workflow: Option<Workflow>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4 max-w-3xl w-full mx-auto",
            selected_item: SideBar::Workflows,
            team_id: team_id,
            rbac: rbac,
            title: "Workflow Details",
            header: rsx!(
                h3 { "Workflow Details" }
            ),

            if let Some(workflow) = workflow {
                div {
                    // Header section with icon, name, and status
                    div {
                        class: "flex items-start gap-4 mb-6",
                        img {
                            class: "border rounded p-2 flex-shrink-0",
                            src: get_workflow_icon(&workflow.name),
                            width: "48",
                            height: "48",
                            alt: "Workflow icon"
                        }
                        div {
                            class: "flex-1 min-w-0",
                            div {
                                class: "flex items-center gap-3 mb-2",
                                h2 {
                                    class: "text-2xl font-semibold",
                                    "{workflow.name}"
                                }
                                span {
                                    class: workflow.status.badge_class(),
                                    "{workflow.status.display_text()}"
                                }
                            }
                            p {
                                class: "text-gray-600 mb-4",
                                "{workflow.description}"
                            }
                        }
                    }

                    hr {
                        class: "mb-6"
                    }

                    // Workflow Details Section
                    div {
                        class: "mb-8",
                        h3 {
                            class: "text-lg font-semibold mb-4",
                            "Workflow Details"
                        }
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div {
                                class: "bg-gray-50 p-4 rounded-lg",
                                h4 {
                                    class: "font-medium text-gray-700 mb-2",
                                    "Trigger Type"
                                }
                                p {
                                    class: "text-gray-900",
                                    "{workflow.trigger_type}"
                                }
                            }
                            div {
                                class: "bg-gray-50 p-4 rounded-lg",
                                h4 {
                                    class: "font-medium text-gray-700 mb-2",
                                    "Category"
                                }
                                p {
                                    class: "text-gray-900",
                                    "{workflow.category}"
                                }
                            }
                        }
                    }

                    // Instructions Section with Example Prompts
                    div {
                        h3 {
                            class: "text-lg font-semibold mb-4",
                            "Example Prompts"
                        }
                        p {
                            class: "text-gray-600 mb-4",
                            "Use these example prompts to interact with this workflow:"
                        }
                        div {
                            class: "space-y-3",
                            for (index, prompt) in get_workflow_example_prompts(&workflow).iter().enumerate() {
                                div {
                                    class: "bg-gray-50 border border-gray-200 rounded-lg p-4",
                                    div {
                                        class: "flex items-start justify-between",
                                        div {
                                            class: "flex-1",
                                            h4 {
                                                class: "font-medium text-gray-700 mb-2",
                                                "Example {index + 1}"
                                            }
                                            code {
                                                class: "text-sm text-gray-800 bg-white px-2 py-1 rounded border",
                                                "{prompt}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Back to workflows link
                    div {
                        class: "mt-8 pt-6 border-t",
                        a {
                            href: crate::routes::workflows::Index { team_id }.to_string(),
                            class: "inline-flex items-center text-blue-600 hover:text-blue-800 font-medium",
                            "← Back to Workflows"
                        }
                    }
                }
            } else {
                // Workflow not found
                div {
                    class: "text-center py-12",
                    h2 {
                        class: "text-xl font-semibold text-gray-900 mb-2",
                        "Workflow Not Found"
                    }
                    p {
                        class: "text-gray-600 mb-6",
                        "The requested workflow could not be found."
                    }
                    a {
                        href: crate::routes::workflows::Index { team_id }.to_string(),
                        class: "inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-medium",
                        "← Back to Workflows"
                    }
                }
            }
        }
    };

    crate::render(page)
}
