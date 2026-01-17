#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use crate::components::card_item::{CardItem, CountLabel};
use crate::components::confirm_modal::ConfirmModal;
use crate::integrations::mcp_url_modal::McpUrlModal;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{datasets::Dataset, documents::Document, models::Model};
use db::Licence;
use dioxus::prelude::*;
use std::convert::TryFrom;

pub fn page(
    rbac: Rbac,
    team_id: i32,
    dataset: Dataset,
    documents: Vec<Document>,
    models: Vec<Model>,
    can_set_visibility_to_company: bool,
    locale: &str,
) -> String {
    let can_edit_dataset = rbac.can_edit_dataset(&dataset);
    let edit_trigger_id = format!("edit-dataset-trigger-{}-{}", dataset.id, team_id);
    let delete_trigger_id = format!("delete-dataset-trigger-{}-{}", dataset.id, team_id);
    let dataset_name = dataset.name.clone();
    let dataset_external_id = dataset.external_id.to_string();
    let licence = Licence::global();
    let show_mcp_modal = licence.features.mcp;

    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::Datasets,
            team_id: team_id,
            rbac: rbac,
            title: format!("{dataset_name} / Documents"),
            locale: Some(locale.to_string()),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: dataset_name.clone(),
                            href: None
                        },
                        BreadcrumbItem {
                            text: "Documents".into(),
                            href: None
                        }
                    ]
                }
                div {
                    class: "flex items-center gap-2",

                    if show_mcp_modal {
                        McpUrlModal {
                            id_prefix: "dataset-mcp-".to_string(),
                            connection_id: dataset.id,
                            external_id: dataset_external_id.clone(),
                            mcp_slug: Some("datasets".to_string()),
                            connection_label: format!("{dataset_name} dataset"),
                        }
                    }

                    if can_edit_dataset {
                        Button {
                            popover_target: edit_trigger_id.clone(),
                            button_scheme: ButtonScheme::Secondary,
                            "Edit Dataset"
                        }
                    }
                    Button {
                        popover_target: delete_trigger_id.clone(),
                        button_scheme: ButtonScheme::Warning,
                        "Delete Dataset"
                    }
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        popover_target: "upload-form",
                        button_scheme: ButtonScheme::Primary,
                        "Add Document"
                    }
                }
            ),
            div {
                class: "p-4 max-w-3xl w-full mx-auto",

                SectionIntroduction {
                    header: "Documents".to_string(),
                    subtitle: "Upload and manage documents in various formats for this dataset.".to_string(),
                    is_empty: documents.is_empty(),
                    empty_text: "This dataset doesn't have any documents yet. Upload your first document to get started.".to_string(),
                }

                if !documents.is_empty() {
                    for doc in &documents {
                        Row {
                            document: doc.clone(),
                            team_id: team_id,
                            first_time: true
                        }
                    }

                    for doc in documents {
                        ConfirmModal {
                            action: crate::routes::documents::Delete{team_id, document_id: doc.id}.to_string(),
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

                ConfirmModal {
                    action: crate::routes::datasets::Delete{team_id, id: dataset.id}.to_string(),
                    trigger_id: delete_trigger_id.clone(),
                    submit_label: "Delete".to_string(),
                    heading: format!("Delete this {}?", crate::i18n::dataset(locale)),
                    warning: format!(
                        "Are you sure you want to delete this {}?",
                        crate::i18n::dataset(locale)
                    ),
                    hidden_fields: vec![
                        ("team_id".into(), team_id.to_string()),
                        ("id".into(), dataset.id.to_string()),
                    ],
                }

                if can_edit_dataset {
                    crate::datasets::upsert::Upsert {
                        id: dataset.id,
                        trigger_id: edit_trigger_id.clone(),
                        name: dataset_name.clone(),
                        models: models.clone(),
                        team_id: team_id,
                        combine_under_n_chars: dataset.combine_under_n_chars,
                        new_after_n_chars: dataset.new_after_n_chars,
                        _multipage_sections: true,
                        visibility: dataset.visibility,
                        can_set_visibility_to_company,
                        locale: locale.to_string()
                    }
                }

                // The form to create an invitation - always available
                super::upload::Upload {
                    upload_action: crate::routes::documents::Upload{team_id, dataset_id: dataset.id}.to_string()
                }
            }
        }
    };

    crate::render(page)
}

#[component]
pub fn Row(document: Document, team_id: i32, first_time: bool) -> Element {
    let text = if let Some(failure_reason) = document.failure_reason.clone() {
        failure_reason.replace(['{', '"', ':', '}'], " ")
    } else {
        "None".to_string()
    };

    let class = if document.waiting > 0 || document.batches == 0 {
        "processing"
    } else {
        "processing-finished"
    };

    let id = format!("processing-label-{}", document.id);

    let src = if first_time {
        Some(
            crate::routes::documents::Processing {
                team_id,
                document_id: document.id,
            }
            .to_string(),
        )
    } else {
        None
    };

    let chunk_count = usize::try_from(document.batches).unwrap_or(0);
    let avatar_initial = document.file_name.chars().next().unwrap_or('D').to_string();
    let content_size = document.content_size;

    rsx!(CardItem {
        class: Some("w-full".into()),
        avatar_name: Some(avatar_initial),
        title: document.file_name.clone(),
        description: Some(rsx!(
            div {
                class: "flex flex-wrap items-center gap-2 text-sm text-base-content/70",
                span { "Status:" }
                turbo-frame {
                    id,
                    src,

                    if document.waiting > 0 || document.batches == 0 {
                        Badge {
                            class: class,
                            badge_style: BadgeStyle::Outline,
                            badge_size: BadgeSize::Sm,
                            "Processing ({document.waiting} remaining)"
                        }
                    } else if document.failure_reason.is_some() {
                        ToolTip {
                            text: "{text}",
                            Badge {
                                class: class,
                                badge_color: BadgeColor::Error,
                                badge_style: BadgeStyle::Outline,
                                badge_size: BadgeSize::Sm,
                                "Failed"
                            }
                        }
                    } else if document.fail_count > 0 {
                        Badge {
                            class: class,
                            badge_color: BadgeColor::Error,
                            badge_style: BadgeStyle::Outline,
                            badge_size: BadgeSize::Sm,
                            "Processed ({document.fail_count} failed)"
                        }
                    } else {
                        Badge {
                            class: class,
                            badge_color: BadgeColor::Success,
                            badge_style: BadgeStyle::Outline,
                            badge_size: BadgeSize::Sm,
                            "Processed"
                        }
                    }
                }
            }
        )),
        footer: Some(rsx!(
            div {
                class: "text-xs text-base-content/60",
                "Size: {content_size} bytes"
            }
        )),
        count_labels: vec![CountLabel {
            count: chunk_count,
            label: "Chunk".to_string()
        }],
        action: Some(rsx!(
            DropDown {
                direction: Direction::Left,
                button_text: "...",
                DropDownLink {
                    popover_target: format!("delete-doc-trigger-{}-{}",
                        document.id, team_id),
                    href: "#",
                    target: "_top",
                    "Delete Document"
                }
            }
        )),
    })
}
