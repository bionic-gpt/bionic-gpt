#![allow(non_snake_case)]
use crate::app_layout::Layout;
use crate::app_layout::SideBar;
use crate::ConfirmModal;
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
                h3 { "Datasets" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "new-dataset-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Dataset"
                }
            ),

            if datasets.is_empty() {
                BlankSlate {
                    heading: "Looks like you don't have any datasets yet",
                    visual: nav_ccsds_data_svg.name,
                    description: "Datasets allow you to organize your documents like folders"
                }
            } else {
                Card {
                    class: "has-data-table",
                    CardHeader {
                        title: "Datasets"
                    }
                    CardBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "Name" }
                                th { "Visibility" }
                                th {
                                    class: "max-sm:hidden",
                                    "Document Count"
                                }
                                th {
                                    class: "max-sm:hidden",
                                    "Chunking Strategy"
                                }
                                th {
                                    class: "text-right",
                                    "Action"
                                }
                            }
                            tbody {

                                for dataset in &datasets {
                                    tr {
                                        td {
                                            a {
                                                href: crate::routes::documents::Index{team_id, dataset_id: dataset.id}.to_string(),
                                                "{dataset.name}"
                                            }
                                        }
                                        td {
                                            crate::assistants::visibility::VisLabel {
                                                visibility: dataset.visibility
                                            }
                                        }
                                        td {
                                            class: "max-sm:hidden",
                                            "{dataset.count}"
                                        }
                                        td {
                                            class: "max-sm:hidden",
                                            Label {
                                                label_role: LabelRole::Highlight,
                                                "By Title"
                                            }
                                            }
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    href: crate::routes::documents::Index{team_id, dataset_id: dataset.id}.to_string(),
                                                    target: "_top",
                                                    "View"
                                                }

                                                if rbac.can_edit_dataset(dataset) {
                                                    DropDownLink {
                                                        popover_target: format!("edit-trigger-{}-{}",
                                                            dataset.id, team_id),
                                                        href: "#",
                                                        target: "_top",
                                                        "Edit"
                                                    }
                                                }
                                                DropDownLink {
                                                    popover_target: format!("delete-trigger-{}-{}",
                                                        dataset.id, team_id),
                                                    href: "#",
                                                    target: "_top",
                                                    "Delete"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
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
    };

    crate::render(page)
}
