#![allow(non_snake_case)]
use crate::app_layout::AdminLayout;
use crate::app_layout::SideBar;
use crate::components::card_item::{CardItem, CountLabel};
use crate::i18n;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{datasets::Dataset, models::Model};
use db::types::public::ChunkingStrategy;
use dioxus::prelude::*;
use std::convert::TryFrom;

pub fn page(
    rbac: Rbac,
    team_id: i32,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
    can_set_visibility_to_company: bool,
    locale: &str,
) -> String {
    let datasets_label = i18n::datasets(locale);
    let dataset_label = i18n::dataset(locale);
    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::Datasets,
            team_id: team_id,
            rbac: rbac.clone(),
            title: datasets_label.clone(),
            locale: Some(locale.to_string()),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: dataset_label.clone(),
                            href: None
                        }
                    ]
                }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "new-dataset-form",
                    button_scheme: ButtonScheme::Primary,
                    {format!("Add {}", dataset_label.clone())}
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",

                SectionIntroduction {
                    header: datasets_label.clone(),
                    subtitle: format!(
                        "Organize your documents into {} for better management and retrieval.",
                        datasets_label.clone()
                    ),
                    is_empty: datasets.is_empty(),
                    empty_text: format!(
                        "No {} created yet. {} allow you to organize your documents like folders.",
                        datasets_label.clone(),
                        datasets_label.clone()
                    ),
                }

                if !datasets.is_empty() {
                    for dataset in &datasets {
                        DatasetCard {
                            dataset: dataset.clone(),
                            team_id,
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
                    can_set_visibility_to_company,
                    locale: locale.to_string()
                }
            }
        }
    };

    crate::render(page)
}

#[component]
fn DatasetCard(dataset: Dataset, team_id: i32) -> Element {
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
    })
}
