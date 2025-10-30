#![allow(non_snake_case)]
use crate::app_layout::Layout;
use crate::app_layout::SideBar;
use crate::components::card_item::{CardItem, CountLabel};
use crate::i18n;
use crate::integrations::mcp_url_modal::McpUrlModal;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{datasets::Dataset, models::Model};
use db::types::public::ChunkingStrategy;
use db::Licence;
use dioxus::prelude::*;
use std::convert::TryFrom;

pub fn page(
    rbac: Rbac,
    team_id: i32,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
    can_set_visibility_to_company: bool,
) -> String {
    let licence = Licence::global();
    let show_mcp_modal = licence.features.mcp;

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Datasets,
            team_id: team_id,
            rbac: rbac.clone(),
            title: i18n::datasets().to_string(),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: i18n::dataset().into(),
                            href: None
                        }
                    ]
                }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "new-dataset-form",
                    button_scheme: ButtonScheme::Primary,
                    {format!("Add {}", i18n::dataset())}
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",

                SectionIntroduction {
                    header: i18n::datasets().to_string(),
                    subtitle: format!(
                        "Organize your documents into {} for better management and retrieval.",
                        i18n::datasets()
                    ),
                    is_empty: datasets.is_empty(),
                    empty_text: format!(
                        "No {} created yet. {} allow you to organize your documents like folders.",
                        i18n::datasets(),
                        i18n::datasets()
                    ),
                }

                if !datasets.is_empty() {
                    for dataset in &datasets {
                        DatasetCard {
                            dataset: dataset.clone(),
                            team_id,
                            show_mcp_modal,
                        }
                    }
                }

                super::upsert::Upsert {
                    trigger_id: "new-dataset-form",
                    name: "".to_string(),
                    models: models.clone(),
                    team_id: team_id,
                    combine_under_n_chars: 500,
                    new_after_n_chars: 1000,
                    _multipage_sections: true,
                    visibility: db::Visibility::Private,
                    can_set_visibility_to_company
                }
            }
        }
    };

    crate::render(page)
}

#[component]
fn DatasetCard(dataset: Dataset, team_id: i32, show_mcp_modal: bool) -> Element {
    let documents_link = crate::routes::documents::Index {
        team_id,
        dataset_id: dataset.id,
    }
    .to_string();
    let document_count = usize::try_from(dataset.count).unwrap_or(0);
    let chunking_label = match dataset.chunking_strategy {
        ChunkingStrategy::ByTitle => "By Title",
    };
    let avatar_initial = dataset.name.chars().next().unwrap_or('D').to_string();
    let external_id = dataset.external_id.to_string();
    let action = if show_mcp_modal {
        Some(rsx!(McpUrlModal {
            id_prefix: "dataset-mcp-".to_string(),
            connection_id: dataset.id,
            external_id,
            mcp_slug: Some("datasets".to_string()),
            connection_label: "dataset".to_string(),
        }))
    } else {
        None
    };

    rsx!(CardItem {
        class: Some("cursor-pointer hover:bg-base-200 w-full".into()),
        clickable_link: documents_link.clone(),
        avatar_name: Some(avatar_initial),
        title: dataset.name.clone(),
        description: Some(rsx!(
            div {
                class: "flex flex-wrap items-center gap-2 text-sm text-base-content/70",
                crate::assistants::visibility::VisLabel {
                    visibility: dataset.visibility
                }
                Badge {
                    badge_color: BadgeColor::Accent,
                    badge_style: BadgeStyle::Outline,
                    badge_size: BadgeSize::Sm,
                    "{chunking_label}"
                }
            }
        )),
        footer: Some(rsx!(
            div {
                class: "text-xs text-base-content/60 truncate",
                "Embedding model: {dataset.embeddings_model_name}"
            }
        )),
        count_labels: vec![CountLabel {
            count: document_count,
            label: "Document".to_string()
        }],
        action,
    })
}
