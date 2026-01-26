#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
use crate::documents::page::Row;
use crate::documents::Upload;
use crate::history;
use crate::SectionIntroduction;
use assets::files::*;
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
    let history_buckets = history::bucket_history(histories);
    let upload_trigger = "upload-form";
    let edit_trigger = format!("edit-project-{}-{}", project.id, team_id);

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Projects,
            team_id: team_id.clone(),
            rbac: rbac,
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
                div {
                    class: "flex items-center gap-2",
                    form {
                        method: "post",
                        action: crate::routes::projects::StartChat { team_id: team_id.clone(), project_id: project.id }.to_string(),
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Start Chat"
                        }
                    }
                    Button {
                        button_scheme: ButtonScheme::Neutral,
                        popover_target: edit_trigger.clone(),
                        "Edit Project"
                    }
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        popover_target: "{upload_trigger}",
                        button_scheme: ButtonScheme::Primary,
                        "Add Attachment"
                    }
                }
            ),
            div {
                class: "p-4 max-w-3xl w-full mx-auto space-y-6",
                Card {
                    CardHeader { title: "Chat prompt" }
                    CardBody {
                        if project.instructions.trim().is_empty() {
                            div { class: "text-sm text-base-content/60", "No chat prompt yet." }
                        } else {
                            p { class: "text-sm whitespace-pre-wrap", "{project.instructions}" }
                        }
                    }
                }
                SectionIntroduction {
                    header: "Chat History".to_string(),
                    subtitle: "Continue conversations tied to this project.".to_string(),
                    is_empty: history_buckets.1 == 0,
                    empty_text: "No chats yet. Start a conversation to build history here.".to_string(),
                }
                if history_buckets.1 > 0 {
                    history::history_table::HistoryTable {
                        team_id: team_id.clone(),
                        buckets: history_buckets.0
                    }
                }

                SectionIntroduction {
                    header: "Attachments".to_string(),
                    subtitle: "Project files used to ground your chats.".to_string(),
                    is_empty: documents.is_empty(),
                    empty_text: "No attachments yet. Upload files to add context.".to_string(),
                }
                if !documents.is_empty() {
                    for doc in &documents {
                        Row {
                            document: doc.clone(),
                            team_id: team_id.clone(),
                            first_time: true
                        }
                    }
                    for doc in &documents {
                        ConfirmModal {
                            action: crate::routes::documents::Delete{team_id: team_id.clone(), document_id: doc.id}.to_string(),
                            trigger_id: format!("delete-doc-trigger-{}-{}", doc.id, team_id),
                            submit_label: "Delete Document".to_string(),
                            heading: "Delete this document?".to_string(),
                            warning: "Are you sure you want to delete this document?".to_string(),
                            hidden_fields: vec![
                                ("team_id".into(), team_id.to_string()),
                                ("document_id".into(), doc.id.to_string()),
                                ("dataset_id".into(), doc.dataset_id.to_string()),
                            ],
                        }
                    }
                }

                Upload {
                    upload_action: crate::routes::documents::Upload { team_id: team_id.clone(), dataset_id: project.dataset_id }.to_string()
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
            }
        }
    };

    crate::render(page)
}
