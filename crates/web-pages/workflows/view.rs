#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use dioxus::prelude::*;

use super::workflow_cards::{get_workflow_icon, Workflow};

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
                            class: "border rounded p-2 shrink-0",
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

                    // Instructions Section
                    div {
                        class: "mb-8",
                        h3 {
                            class: "text-lg font-semibold mb-4",
                            "Instructions"
                        }
                        textarea {
                            class: "w-full h-32 p-4 border border-gray-300 rounded-lg resize-none bg-gray-50",
                            readonly: true,
                            "Connect to linked in and get anyone who has set their job to an AI related field in the last few days"
                        }
                    }

                    // Actions Section
                    div {
                        class: "mb-8",
                        h3 {
                            class: "text-lg font-semibold mb-4",
                            "Actions"
                        }

                        // LinkedIn Action
                        details {
                            class: "card mt-5",
                            summary {
                                class: "cursor-pointer px-4 py-3 flex items-center justify-between",
                                div {
                                    class: "flex",
                                    div {
                                        class: "",
                                        img {
                                            class: "border rounded p-1",
                                            src: "data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjIwMHB4IiB3aWR0aD0iMjAwcHgiIHZlcnNpb249IjEuMSIgaWQ9IkxheWVyXzEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiIHZpZXdCb3g9IjAgMCAzODIgMzgyIiB4bWw6c3BhY2U9InByZXNlcnZlIiBmaWxsPSIjMDAwMDAwIj48ZyBpZD0iU1ZHUmVwb19iZ0NhcnJpZXIiIHN0cm9rZS13aWR0aD0iMCI+PC9nPjxnIGlkPSJTVkdSZXBvX3RyYWNlckNhcnJpZXIiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCI+PC9nPjxnIGlkPSJTVkdSZXBvX2ljb25DYXJyaWVyIj4gPHBhdGggc3R5bGU9ImZpbGw6IzAwNzdCNzsiIGQ9Ik0zNDcuNDQ1LDBIMzQuNTU1QzE1LjQ3MSwwLDAsMTUuNDcxLDAsMzQuNTU1djMxMi44ODlDMCwzNjYuNTI5LDE1LjQ3MSwzODIsMzQuNTU1LDM4MmgzMTIuODg5IEMzNjYuNTI5LDM4MiwzODIsMzY2LjUyOSwzODIsMzQ3LjQ0NFYzNC41NTVDMzgyLDE1LjQ3MSwzNjYuNTI5LDAsMzQ3LjQ0NSwweiBNMTE4LjIwNywzMjkuODQ0YzAsNS41NTQtNC41MDIsMTAuMDU2LTEwLjA1NiwxMC4wNTYgSDY1LjM0NWMtNS41NTQsMC0xMC4wNTYtNC41MDItMTAuMDU2LTEwLjA1NlYxNTAuNDAzYzAtNS41NTQsNC41MDItMTAuMDU2LDEwLjA1Ni0xMC4wNTZoNDIuODA2IGM1LjU1NCwwLDEwLjA1Niw0LjUwMiwxMC4wNTYsMTAuMDU2VjMyOS44NDR6IE04Ni43NDgsMTIzLjQzMmMtMjIuNDU5LDAtNDAuNjY2LTE4LjIwNy00MC42NjYtNDAuNjY2UzY0LjI4OSw0Mi4xLDg2Ljc0OCw0Mi4xIHM0MC42NjYsMTguMjA3LDQwLjY2Niw0MC42NjZTMTA5LjIwOCwxMjMuNDMyLDg2Ljc0OCwxMjMuNDMyeiBNMzQxLjkxLDMzMC42NTRjMCw1LjEwNi00LjE0LDkuMjQ2LTkuMjQ2LDkuMjQ2SDI4Ni43MyBjLTUuMTA2LDAtOS4yNDYtNC4xNC05LjI0Ni05LjI0NnYtODQuMTY4YzAtMTIuNTU2LDMuNjgzLTU1LjAyMS0zMi44MTMtNTUuMDIxYy0yOC4zMDksMC0zNC4wNTEsMjkuMDY2LTM1LjIwNCw0Mi4xMXY5Ny4wNzkgYzAsNS4xMDYtNC4xMzksOS4yNDYtOS4yNDYsOS4yNDZoLTQ0LjQyNmMtNS4xMDYsMC05LjI0Ni00LjE0LTkuMjQ2LTkuMjQ2VjE0OS41OTNjMC01LjEwNiw0LjE0LTkuMjQ2LDkuMjQ2LTkuMjQ2aDQ0LjQyNiBjNS4xMDYsMCw5LjI0Niw0LjE0LDkuMjQ2LDkuMjQ2djE1LjY1NWMxMC40OTctMTUuNzUzLDI2LjA5Ny0yNy45MTIsNTkuMzEyLTI3LjkxMmM3My41NTIsMCw3My4xMzEsNjguNzE2LDczLjEzMSwxMDYuNDcyIEwzNDEuOTEsMzMwLjY1NEwzNDEuOTEsMzMwLjY1NHoiPjwvcGF0aD4gPC9nPjwvc3ZnPg==",
                                            width: "24",
                                            height: "24"
                                        }
                                    }
                                    div {
                                        class: "ml-4",
                                        h2 {
                                            class: "font-semibold",
                                            "LinkedIn Search"
                                        }
                                        p {
                                            "Search LinkedIn based on the query provided"
                                        }
                                    }
                                }
                            }
                            "Searches LinkedIn for professionals who have recently updated their job titles to AI-related positions"
                        }

                        // Gmail Action
                        details {
                            class: "card mt-5",
                            summary {
                                class: "cursor-pointer px-4 py-3 flex items-center justify-between",
                                div {
                                    class: "flex",
                                    div {
                                        class: "",
                                        img {
                                            class: "border rounded p-1",
                                            src: "data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMzIgMzIiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGcgaWQ9IlNWR1JlcG9fYmdDYXJyaWVyIiBzdHJva2Utd2lkdGg9IjAiPjwvZz48ZyBpZD0iU1ZHUmVwb190cmFjZXJDYXJyaWVyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiPjwvZz48ZyBpZD0iU1ZHUmVwb19pY29uQ2FycmllciI+IDxwYXRoIGQ9Ik0yIDExLjk1NTZDMiA4LjQ3MDc4IDIgNi43Mjg0IDIuNjc4MTggNS4zOTczOUMzLjI3NDczIDQuMjI2NjEgNC4yMjY2MSAzLjI3NDczIDUuMzk3MzkgMi42NzgxOEM2LjcyODQgMiA4LjQ3MDc4IDIgMTEuOTU1NiAySDIwLjA0NDRDMjMuNTI5MiAyIDI1LjI3MTYgMiAyNi42MDI2IDIuNjc4MThDMjcuNzczNCAzLjI3NDczIDI4LjcyNTMgNC4yMjY2MSAyOS4zMjE4IDUuMzk3MzlDMzAgNi43Mjg0IDMwIDguNDcwNzggMzAgMTEuOTU1NlYyMC4wNDQ0QzMwIDIzLjUyOTIgMzAgMjUuMjcxNiAyOS4zMjE4IDI2LjYwMjZDMjguNzI1MyAyNy43NzM0IDI3Ljc3MzQgMjguNzI1MyAyNi42MDI2IDI5LjMyMThDMjUuMjcxNiAzMCAyMy41MjkyIDMwIDIwLjA0NDQgMzBIMTEuOTU1NkM4LjQ3MDc4IDMwIDYuNzI4NCAzMCA1LjM5NzM5IDI5LjMyMThDNC4yMjY2MSAyOC43MjUzIDMuMjc0NzMgMjcuNzczNCAyLjY3ODE4IDI2LjYwMjZDMiAyNS4yNzE2IDIgMjMuNTI5MiAyIDIwLjA0NDRWMTEuOTU1NloiIGZpbGw9IndoaXRlIj48L3BhdGg+IDxwYXRoIGQ9Ik0yMi4wNTE1IDguNTIyOTVMMTYuMDY0NCAxMy4xOTU0TDkuOTQwNDMgOC41MjI5NVY4LjUyNDIxTDkuOTQ3ODMgOC41MzA1M1YxNS4wNzMyTDE1Ljk5NTQgMTkuODQ2NkwyMi4wNTE1IDE1LjI1NzVWOC41MjI5NVoiIGZpbGw9IiNFQTQzMzUiPjwvcGF0aD4gPHBhdGggZD0iTTIzLjYyMzEgNy4zODYzOUwyMi4wNTA4IDguNTIyOTJWMTUuMjU3NUwyNi45OTgzIDExLjQ1OVY5LjE3MDc0QzI2Ljk5ODMgOS4xNzA3NCAyNi4zOTc4IDUuOTAyNTggMjMuNjIzMSA3LjM4NjM5WiIgZmlsbD0iI0ZCQkMwNSI+PC9wYXRoPiA8cGF0aCBkPSJNMjIuMDUwOCAxNS4yNTc1VjIzLjk5MjRIMjUuODQyOEMyNS44NDI4IDIzLjk5MjQgMjYuOTIxOSAyMy44ODEzIDI2Ljk5OTUgMjIuNjUxM1YxMS40NTlMMjIuMDUwOCAxNS4yNTc1WiIgZmlsbD0iIzM0QTg1MyI+PC9wYXRoPiA8cGF0aCBkPSJNOS45NDgxMSAyNC4wMDAxVjE1LjA3MzJMOS45NDA0MyAxNS4wNjY5TDkuOTQ4MTEgMjQuMDAwMVoiIGZpbGw9IiNDNTIyMUYiPjwvcGF0aD4gPHBhdGggZD0iTTkuOTQwMTQgOC41MjQwNEw4LjM3NjQ2IDcuMzkzODJDNS42MDE3OSA1LjkxMDAxIDUgOS4xNzY5MiA1IDkuMTc2OTJWMTEuNDY1MUw5Ljk0MDE0IDE1LjA2NjdWOC41MjQwNFoiIGZpbGw9IiNDNTIyMUYiPjwvcGF0aD4gPHBhdGggZD0iTTkuOTQwNDMgOC41MjQ0MVYxNS4wNjcxTDkuOTQ4MTEgMTUuMDczNFY4LjUzMDczTDkuOTQwNDMgOC41MjQ0MVoiIGZpbGw9IiNDNTIyMUYiPjwvcGF0aD4gPHBhdGggZD0iTTUgMTEuNDY2OFYyMi42NTkxQzUuMDc2NDYgMjMuODkwNCA2LjE1NjczIDI0LjAwMDMgNi4xNTY3MyAyNC4wMDAzSDkuOTQ4NzdMOS45NDAxNCAxNS4wNjcxTDUgMTEuNDY2OFoiIGZpbGw9IiM0Mjg1RjQiPjwvcGF0aD4gPC9nPjwvc3ZnPg==",
                                            width: "24",
                                            height: "24"
                                        }
                                    }
                                    div {
                                        class: "ml-4",
                                        h2 {
                                            class: "font-semibold",
                                            "Gmail Send Email"
                                        }
                                        p {
                                            "Create and send an email with the gmail API"
                                        }
                                    }
                                }
                            }
                            "Composes and sends personalized emails to the LinkedIn prospects found in the previous step"
                        }
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
