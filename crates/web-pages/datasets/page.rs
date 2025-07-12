#![allow(non_snake_case)]
use super::dataset_card::DatasetCard;
use crate::app_layout::Layout;
use crate::app_layout::SideBar;
use crate::components::confirm_modal::ConfirmModal;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{datasets::Dataset, models::Model};
use dioxus::prelude::*;

pub fn page(
    rbac: Rbac,
    team_id: i32,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
    can_set_visibility_to_company: bool,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Datasets,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Datasets",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Dataset".into(),
                            href: None
                        }
                    ]
                }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "new-dataset-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Dataset"
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",

                SectionIntroduction {
                    header: "Datasets".to_string(),
                    subtitle: "Organize your documents into datasets for better management and retrieval.".to_string(),
                    is_empty: datasets.is_empty(),
                    empty_text: "No datasets created yet. Datasets allow you to organize your documents like folders.".to_string(),
                }

                if !datasets.is_empty() {
                    div {
                        class: "space-y-2",
                        for dataset in &datasets {
                            DatasetCard { team_id, rbac: rbac.clone(), dataset: dataset.clone() }
                        }
                    }

                        for dataset in datasets {
                            ConfirmModal {
                                action: crate::routes::datasets::Delete{team_id, id: dataset.id}.to_string(),
                                trigger_id: format!("delete-trigger-{}-{}", dataset.id, team_id),
                                submit_label: "Delete".to_string(),
                                heading: "Delete this Dataset?".to_string(),
                                warning: "Are you sure you want to delete this Dataset?".to_string(),
                                hidden_fields: vec![
                                    ("team_id".into(), team_id.to_string()),
                                    ("id".into(), dataset.id.to_string()),
                                ],
                            }

                            super::upsert::Upsert {
                                id: dataset.id,
                                trigger_id: format!("edit-trigger-{}-{}", dataset.id, team_id),
                                name: dataset.name,
                                models: models.clone(),
                                team_id: team_id,
                                combine_under_n_chars: dataset.combine_under_n_chars,
                                new_after_n_chars: dataset.new_after_n_chars,
                                _multipage_sections: true,
                                visibility: dataset.visibility,
                                can_set_visibility_to_company
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
