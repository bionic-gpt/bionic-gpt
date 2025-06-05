#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Workflow {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: WorkflowStatus,
    pub trigger_type: String,
    pub icon_url: String,
    pub category: String,
}

#[derive(Clone, PartialEq, Debug)]
pub enum WorkflowStatus {
    Active,
    Paused,
    Draft,
}

impl WorkflowStatus {
    pub fn badge_class(&self) -> &'static str {
        match self {
            WorkflowStatus::Active => "badge badge-success",
            WorkflowStatus::Paused => "badge badge-warning",
            WorkflowStatus::Draft => "badge badge-neutral",
        }
    }

    pub fn display_text(&self) -> &'static str {
        match self {
            WorkflowStatus::Active => "Active",
            WorkflowStatus::Paused => "Paused",
            WorkflowStatus::Draft => "Draft",
        }
    }
}

// Default placeholder SVG icons for different workflow types
const SOCIAL_MEDIA_ICON: &str = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDgiIGhlaWdodD0iNDgiIHZpZXdCb3g9IjAgMCA0OCA0OCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHJlY3Qgd2lkdGg9IjQ4IiBoZWlnaHQ9IjQ4IiByeD0iOCIgZmlsbD0iIzMzNzNkYyIvPgo8cGF0aCBkPSJNMjQgMTJjNi42MjcgMCAxMiA1LjM3MyAxMiAxMnMtNS4zNzMgMTItMTIgMTItMTItNS4zNzMtMTItMTIgNS4zNzMtMTIgMTItMTJ6bTAgMmMtNS41MjMgMC0xMCA0LjQ3Ny0xMCAxMHM0LjQ3NyAxMCAxMCAxMCAxMC00LjQ3NyAxMC0xMC00LjQ3Ny0xMC0xMC0xMHoiIGZpbGw9IndoaXRlIi8+CjxjaXJjbGUgY3g9IjI0IiBjeT0iMjQiIHI9IjMiIGZpbGw9IndoaXRlIi8+CjxjaXJjbGUgY3g9IjMwIiBjeT0iMTgiIHI9IjIiIGZpbGw9IndoaXRlIi8+Cjwvc3ZnPgo=";

const WEBINAR_ICON: &str = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDgiIGhlaWdodD0iNDgiIHZpZXdCb3g9IjAgMCA0OCA0OCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHJlY3Qgd2lkdGg9IjQ4IiBoZWlnaHQ9IjQ4IiByeD0iOCIgZmlsbD0iIzEwYjk4MSIvPgo8cmVjdCB4PSIxMCIgeT0iMTQiIHdpZHRoPSIyOCIgaGVpZ2h0PSIyMCIgcng9IjIiIGZpbGw9IndoaXRlIi8+CjxyZWN0IHg9IjE0IiB5PSIxOCIgd2lkdGg9IjIwIiBoZWlnaHQ9IjEyIiByeD0iMSIgZmlsbD0iIzEwYjk4MSIvPgo8Y2lyY2xlIGN4PSIyNCIgY3k9IjI0IiByPSIzIiBmaWxsPSJ3aGl0ZSIvPgo8L3N2Zz4K";

const CALENDAR_ICON: &str = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDgiIGhlaWdodD0iNDgiIHZpZXdCb3g9IjAgMCA0OCA0OCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHJlY3Qgd2lkdGg9IjQ4IiBoZWlnaHQ9IjQ4IiByeD0iOCIgZmlsbD0iI2Y1OTUxNSIvPgo8cmVjdCB4PSIxMCIgeT0iMTIiIHdpZHRoPSIyOCIgaGVpZ2h0PSIyNCIgcng9IjIiIGZpbGw9IndoaXRlIi8+CjxyZWN0IHg9IjEwIiB5PSIxMiIgd2lkdGg9IjI4IiBoZWlnaHQ9IjYiIHJ4PSIyIiBmaWxsPSIjZjU5NTE1Ii8+CjxyZWN0IHg9IjE0IiB5PSIyMiIgd2lkdGg9IjQiIGhlaWdodD0iNCIgZmlsbD0iI2Y1OTUxNSIvPgo8cmVjdCB4PSIyMiIgeT0iMjIiIHdpZHRoPSI0IiBoZWlnaHQ9IjQiIGZpbGw9IiNmNTk1MTUiLz4KPHJlY3QgeD0iMzAiIHk9IjIyIiB3aWR0aD0iNCIgaGVpZ2h0PSI0IiBmaWxsPSIjZjU5NTE1Ii8+CjxyZWN0IHg9IjE0IiB5PSIyOCIgd2lkdGg9IjQiIGhlaWdodD0iNCIgZmlsbD0iI2Y1OTUxNSIvPgo8cmVjdCB4PSIyMiIgeT0iMjgiIHdpZHRoPSI0IiBoZWlnaHQ9IjQiIGZpbGw9IiNmNTk1MTUiLz4KPC9zdmc+Cg==";

const DOCUMENT_ICON: &str = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDgiIGhlaWdodD0iNDgiIHZpZXdCb3g9IjAgMCA0OCA0OCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHJlY3Qgd2lkdGg9IjQ4IiBoZWlnaHQ9IjQ4IiByeD0iOCIgZmlsbD0iIzZiNzI4MCIvPgo8cmVjdCB4PSIxMiIgeT0iMTAiIHdpZHRoPSIyNCIgaGVpZ2h0PSIyOCIgcng9IjIiIGZpbGw9IndoaXRlIi8+CjxyZWN0IHg9IjE2IiB5PSIxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjIiIGZpbGw9IiM2YjcyODAiLz4KPHJlY3QgeD0iMTYiIHk9IjIwIiB3aWR0aD0iMTYiIGhlaWdodD0iMiIgZmlsbD0iIzZiNzI4MCIvPgo8cmVjdCB4PSIxNiIgeT0iMjQiIHdpZHRoPSIxMiIgaGVpZ2h0PSIyIiBmaWxsPSIjNmI3MjgwIi8+Cjwvc3ZnPgo=";

pub fn get_workflow_icon(workflow_name: &str) -> &'static str {
    match workflow_name {
        name if name.contains("Social Media") => SOCIAL_MEDIA_ICON,
        name if name.contains("Webinar") => WEBINAR_ICON,
        name if name.contains("Scheduling") || name.contains("Call") => CALENDAR_ICON,
        name if name.contains("Proposal") || name.contains("Document") => DOCUMENT_ICON,
        _ => DOCUMENT_ICON, // Default fallback
    }
}

#[component]
pub fn WorkflowCards(workflows: Vec<Workflow>, team_id: i32) -> Element {
    rsx!(
        h1 {
            class: "text-xl font-semibold",
            "Workflows"
        }
        p {
            "Automate your anything with powerful workflows."
        }
        div {
            class: "",
            for workflow in workflows.iter() {
                a {
                    href: crate::routes::workflows::View {team_id, id: workflow.id }.to_string(),
                    class: "no-underline",
                    Card {
                        class: "mt-5 p-4 flex flex-col clickable hover:shadow-lg transition-shadow relative",
                        div {
                            class: "absolute top-3 right-3",
                            span {
                                class: workflow.status.badge_class(),
                                "{workflow.status.display_text()}"
                            }
                        }
                        div {
                            class: "flex items-start gap-4",
                            img {
                                class: "border rounded p-2 shrink-0",
                                src: get_workflow_icon("Social Media"),
                                width: "48",
                                height: "48",
                                alt: "Workflow icon"
                            }
                            div {
                                class: "flex-1 min-w-0",
                                h2 {
                                    class: "font-semibold text-lg mb-2 pr-16",
                                    "{workflow.name}"
                                }
                                p {
                                    class: "text-sm text-gray-600",
                                    "{workflow.description}"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

// Helper function to create sample sales-focused workflows
pub fn get_sample_workflows() -> Vec<Workflow> {
    vec![
        Workflow {
            id: 1,
            name: "Generate new leads from LinkedIn".to_string(),
            description: "Automatically capture and qualify leads from social media interactions and mentions".to_string(),
            status: WorkflowStatus::Active,
            trigger_type: "Social Media Engagement".to_string(),
            icon_url: SOCIAL_MEDIA_ICON.to_string(),
            category: "Lead Generation".to_string(),
        },
        Workflow {
            id: 2,
            name: "Triage incoming support emails".to_string(),
            description: "Send personalized follow-up sequences to webinar attendees based on their engagement level".to_string(),
            status: WorkflowStatus::Active,
            trigger_type: "Webinar Completion".to_string(),
            icon_url: WEBINAR_ICON.to_string(),
            category: "Lead Nurturing".to_string(),
        },
        Workflow {
            id: 3,
            name: "Transfer incoming leads to the CRM".to_string(),
            description: "Automate calendar booking and send preparation materials to qualified prospects".to_string(),
            status: WorkflowStatus::Active,
            trigger_type: "Lead Qualification Score".to_string(),
            icon_url: CALENDAR_ICON.to_string(),
            category: "Sales Process".to_string(),
        },
        Workflow {
            id: 4,
            name: "Auto respond to RFP's".to_string(),
            description: "Generate customized proposals based on prospect data and specific requirements".to_string(),
            status: WorkflowStatus::Draft,
            trigger_type: "Opportunity Stage Change".to_string(),
            icon_url: DOCUMENT_ICON.to_string(),
            category: "Sales Automation".to_string(),
        },
    ]
}
