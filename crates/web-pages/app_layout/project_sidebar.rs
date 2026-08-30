#![allow(non_snake_case)]

use daisy_rsx::{Avatar, AvatarSize, AvatarType, Button, ButtonShape, ButtonSize, ButtonStyle};
use db::ProjectNav;
use dioxus::prelude::*;

fn project_initial(name: &str) -> String {
    name.trim()
        .chars()
        .next()
        .map(|character| character.to_uppercase().collect())
        .unwrap_or_else(|| "?".to_string())
}

#[component]
pub fn ProjectSidebar(
    team_id: String,
    projects: Vec<ProjectNav>,
    selected_project_id: Option<i32>,
    index_selected: bool,
    disabled: bool,
) -> Element {
    let index_href = if disabled {
        String::new()
    } else {
        crate::routes::projects::Index {
            team_id: team_id.clone(),
        }
        .to_string()
    };

    rsx! {
        ul {
            role: "list",
            class: "menu w-full pb-2",
            li {
                class: "menu-title flex-row items-center justify-between gap-2 pr-3",
                a {
                    class: if index_selected { "min-w-0 flex-1 text-primary" } else { "min-w-0 flex-1" },
                    href: index_href,
                    "aria-disabled": "{disabled}",
                    "aria-current": if index_selected { "page" } else { "false" },
                    "Projects"
                }
                Button {
                    class: "-mr-1 text-base-content/70",
                    disabled,
                    popover_target: "sidebar-new-project",
                    button_size: ButtonSize::ExtraSmall,
                    button_shape: ButtonShape::Square,
                    button_style: ButtonStyle::Ghost,
                    span { class: "text-lg font-normal leading-none", "+" }
                }
            }
            if !projects.is_empty() {
                li {
                    class: "max-h-64 overflow-y-auto overscroll-contain",
                    ul {
                        role: "list",
                        class: "ms-0 space-y-0.5 ps-0 pr-1 before:hidden",
                        for project in projects {
                            li {
                                a {
                                    class: if selected_project_id == Some(project.id) {
                                        "flex min-w-0 items-center gap-2 rounded-lg bg-base-200 px-3 py-1.5 text-sm font-medium"
                                    } else {
                                        "flex min-w-0 items-center gap-2 rounded-lg px-3 py-1.5 text-sm text-base-content/75 hover:bg-base-200"
                                    },
                                    href: crate::routes::projects::View {
                                        team_id: team_id.clone(),
                                        project_id: project.id,
                                    }.to_string(),
                                    title: "{project.name}",
                                    "aria-current": if selected_project_id == Some(project.id) { "page" } else { "false" },
                                    Avatar {
                                        avatar_size: AvatarSize::ExtraSmall,
                                        avatar_type: AvatarType::Team,
                                        name: project_initial(&project.name),
                                    }
                                    span {
                                        class: "min-w-0 truncate",
                                        "{project.name}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_initial_uses_first_character_and_has_a_fallback() {
        assert_eq!(project_initial("airbus"), "A");
        assert_eq!(project_initial(" tender"), "T");
        assert_eq!(project_initial(""), "?");
    }

    #[test]
    fn sidebar_renders_heading_avatars_and_project_selection_only() {
        let html = dioxus_ssr::render_element(rsx! {
            ProjectSidebar {
                team_id: "team".to_string(),
                projects: vec![
                    ProjectNav { id: 1, name: "Airbus Research".to_string() },
                    ProjectNav { id: 2, name: "Tender Analysis".to_string() },
                ],
                selected_project_id: Some(2),
                index_selected: false,
                disabled: true,
            }
        });

        assert!(html.contains(">Projects<"));
        assert!(html.contains(">A<"));
        assert!(html.contains(">T<"));
        assert!(html.contains("class=\"avatar\""));
        assert!(html.contains("before:hidden"));
        assert!(html.contains("data-target=\"sidebar-new-project\""));
        assert!(!html.contains("popovertarget"));
        assert!(!html.contains("New project"));
        assert!(!html.contains("<dialog"));
        assert!(html.contains("Airbus Research"));
        assert!(html.contains("Tender Analysis"));
        assert!(html.contains("aria-current=\"page\""));
        assert!(!html.contains("conversation"));
    }

    #[test]
    fn projects_heading_is_selected_on_the_index() {
        let html = dioxus_ssr::render_element(rsx! {
            ProjectSidebar {
                team_id: "team".to_string(),
                projects: vec![],
                selected_project_id: None,
                index_selected: true,
                disabled: true,
            }
        });

        assert!(html.contains("text-primary"));
        assert!(html.contains("aria-current=\"page\""));
    }
}
