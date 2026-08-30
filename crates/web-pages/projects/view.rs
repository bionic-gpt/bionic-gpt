#![allow(non_snake_case)]

use crate::app_layout::{Layout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
use crate::documents::Upload;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::documents::Document;
use db::queries::projects::Project;
use db::History;
use dioxus::prelude::*;

pub fn page(
    team_id: String,
    rbac: Rbac,
    project: Project,
    histories: Vec<History>,
    documents: Vec<Document>,
    can_set_visibility_to_company: bool,
) -> String {
    let upload_trigger = "upload-form";
    let edit_trigger = format!("edit-project-{}-{}", project.id, team_id);
    let delete_trigger = format!("delete-project-{}-{}", project.id, team_id);
    let upload_action = format!(
        "{}?project_id={}",
        crate::routes::documents::Upload {
            team_id: team_id.clone(),
            dataset_id: project.dataset_id,
        },
        project.id
    );

    let page = rsx! {
        Layout {
            section_class: "p-0",
            selected_item: SideBar::Projects,
            selected_project_id: Some(project.id),
            team_id: team_id.clone(),
            rbac,
            title: project.name.clone(),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Projects".into(),
                            href: Some(crate::routes::projects::Index { team_id: team_id.clone() }.to_string())
                        },
                        BreadcrumbItem {
                            text: project.name.clone(),
                            href: None
                        }
                    ]
                }
            ),
            div {
                class: "mx-auto w-full max-w-6xl space-y-10 px-4 py-8 sm:px-6 lg:px-8",
                div {
                    class: "flex flex-wrap items-center justify-between gap-4",
                    h1 {
                        class: "min-w-0 truncate text-2xl font-semibold",
                        "{project.name}"
                    }
                    div {
                        class: "flex items-center gap-2",
                        form {
                            method: "post",
                            action: crate::routes::projects::StartChat {
                                team_id: team_id.clone(),
                                project_id: project.id,
                            }.to_string(),
                            Button {
                                button_type: ButtonType::Submit,
                                button_scheme: ButtonScheme::Primary,
                                "New chat"
                            }
                        }
                        DropDown {
                            direction: Direction::Left,
                            button_text: "...",
                            DropDownLink {
                                popover_target: edit_trigger.clone(),
                                href: "#",
                                target: "_top",
                                "Edit project"
                            }
                            DropDownLink {
                                popover_target: delete_trigger.clone(),
                                href: "#",
                                target: "_top",
                                "Delete project"
                            }
                        }
                    }
                }

                div {
                    class: "grid gap-10 lg:grid-cols-[minmax(0,2fr)_minmax(18rem,1fr)] lg:gap-12",
                    section {
                        class: "lg:col-start-2 lg:row-start-1",
                        aria_labelledby: "project-instructions-heading",
                        h2 {
                            id: "project-instructions-heading",
                            class: "text-lg font-semibold",
                            "Instructions"
                        }
                        if project.instructions.trim().is_empty() {
                            p {
                                class: "mt-3 text-sm text-base-content/55",
                                "No project instructions yet. Use the project menu to add them."
                            }
                        } else {
                            p {
                                class: "mt-3 whitespace-pre-wrap text-sm leading-relaxed text-base-content/80",
                                "{project.instructions}"
                            }
                        }
                    }

                    section {
                        class: "min-w-0 lg:col-start-1 lg:row-span-2 lg:row-start-1",
                        aria_labelledby: "project-chats-heading",
                        div {
                            class: "flex items-center justify-between gap-4",
                            h2 {
                                id: "project-chats-heading",
                                class: "text-lg font-semibold",
                                "Chats"
                            }
                        }
                        if histories.is_empty() {
                            p {
                                class: "mt-3 text-sm text-base-content/55",
                                "No chats yet. Start a conversation for this project."
                            }
                        } else {
                            div {
                                class: "mt-3 divide-y divide-base-300",
                                for history in histories.iter() {
                                    a {
                                        class: "flex items-center justify-between gap-4 rounded-lg px-2 py-3 hover:bg-base-200",
                                        href: crate::routes::console::Conversation {
                                            team_id: team_id.clone(),
                                            conversation_id: history.id,
                                        }.to_string(),
                                        span {
                                            class: "min-w-0 truncate text-sm font-medium",
                                            "{history.summary}"
                                        }
                                        span {
                                            class: "shrink-0 text-xs text-base-content/50",
                                            RelativeTime {
                                                format: RelativeTimeFormat::Relative,
                                                datetime: &history.created_at_iso,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    section {
                        class: "min-w-0 lg:col-start-2 lg:row-start-2",
                        aria_labelledby: "project-files-heading",
                        div {
                            class: "flex items-center justify-between gap-4",
                            h2 {
                                id: "project-files-heading",
                                class: "text-lg font-semibold",
                                "Files"
                            }
                            Button {
                                prefix_image_src: assets::files::button_plus_svg.name,
                                popover_target: upload_trigger,
                                button_style: ButtonStyle::Ghost,
                                button_size: ButtonSize::Small,
                                "Add files"
                            }
                        }
                        if documents.is_empty() {
                            p {
                                class: "mt-3 text-sm text-base-content/55",
                                "No files have been added to this project."
                            }
                        } else {
                            div {
                                class: "mt-3 divide-y divide-base-300",
                                for document in documents.iter() {
                                    ProjectFileRow {
                                        document: document.clone(),
                                        team_id: team_id.clone(),
                                    }
                                }
                            }
                        }
                    }
                }

                for document in documents.iter() {
                    ConfirmModal {
                        action: crate::routes::documents::Delete {
                            team_id: team_id.clone(),
                            document_id: document.id,
                        }.to_string(),
                        trigger_id: format!("delete-doc-trigger-{}-{}", document.id, team_id),
                        submit_label: "Delete file".to_string(),
                        heading: "Delete this file?".to_string(),
                        warning: "Are you sure you want to remove this file from the project?".to_string(),
                        hidden_fields: vec![
                            ("team_id".into(), team_id.to_string()),
                            ("document_id".into(), document.id.to_string()),
                            ("dataset_id".into(), document.dataset_id.to_string()),
                            ("project_id".into(), project.id.to_string()),
                        ],
                    }
                }

                Upload {
                    upload_action,
                    heading: Some("Add files to this project".to_string()),
                }
                super::upsert::Upsert {
                    id: Some(project.id),
                    trigger_id: edit_trigger,
                    name: project.name.clone(),
                    instructions: project.instructions.clone(),
                    visibility: project.visibility,
                    can_set_visibility_to_company,
                    team_id: team_id.clone(),
                }
                ConfirmModal {
                    action: crate::routes::projects::Delete {
                        team_id: team_id.clone(),
                        id: project.id,
                    }.to_string(),
                    trigger_id: delete_trigger,
                    submit_label: "Delete".to_string(),
                    heading: "Delete this project?".to_string(),
                    warning: "Are you sure you want to delete this project?".to_string(),
                    hidden_fields: vec![
                        ("team_id".into(), team_id.to_string()),
                        ("id".into(), project.id.to_string()),
                    ],
                }
            }
        }
    };

    crate::render(page)
}

#[component]
fn ProjectFileRow(document: Document, team_id: String) -> Element {
    let processing = document.waiting > 0 || document.batches == 0;
    let status_id = format!("processing-label-{}", document.id);
    let status_src = crate::routes::documents::Processing {
        team_id: team_id.clone(),
        document_id: document.id,
    }
    .to_string();
    let failure_text = document
        .failure_reason
        .clone()
        .unwrap_or_default()
        .replace(['{', '"', ':', '}'], " ");

    rsx! {
        div {
            class: "flex items-center gap-3 px-2 py-3",
            div {
                class: "flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-base-200 text-xs font-semibold uppercase",
                "{document.file_name.chars().next().unwrap_or('F')}"
            }
            div {
                class: "min-w-0 flex-1",
                div {
                    class: "truncate text-sm font-medium",
                    "{document.file_name}"
                }
                div {
                    class: "mt-1 flex flex-wrap items-center gap-2 text-xs text-base-content/50",
                    span { "{document.content_size} bytes" }
                    span { "·" }
                    turbo-frame {
                        id: status_id,
                        src: status_src,
                        if processing {
                            span { "Processing ({document.waiting} remaining)" }
                        } else if document.failure_reason.is_some() {
                            ToolTip {
                                text: failure_text,
                                span { class: "text-error", "Failed" }
                            }
                        } else if document.fail_count > 0 {
                            span { class: "text-error", "Processed with errors" }
                        } else {
                            span { "Ready" }
                        }
                    }
                }
            }
            DropDown {
                direction: Direction::Left,
                button_text: "...",
                DropDownLink {
                    popover_target: format!("delete-doc-trigger-{}-{}", document.id, team_id),
                    href: "#",
                    target: "_top",
                    "Delete file"
                }
            }
        }
    }
}
