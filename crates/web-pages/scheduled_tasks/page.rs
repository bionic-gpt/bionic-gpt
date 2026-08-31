#![allow(non_snake_case)]

use crate::app_layout::{Layout, SideBar};
use crate::components::card_item::CardItem;
use crate::components::confirm_modal::ConfirmModal;
use crate::SectionIntroduction;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::scheduled_tasks::ScheduledTask;
use dioxus::prelude::*;

pub fn page(rbac: Rbac, team_id: String, tasks: Vec<ScheduledTask>, locale: &str) -> String {
    let rendered = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::ScheduledTasks,
            team_id: team_id.clone(),
            rbac,
            title: "Scheduled",
            locale: Some(locale.to_string()),
            header: rsx! {
                Breadcrumb {
                    items: vec![BreadcrumbItem { text: "Scheduled".to_string(), href: None }]
                }
            },
            div {
                class: "p-4 max-w-4xl w-full mx-auto",
                SectionIntroduction {
                    header: "Scheduled tasks".to_string(),
                    subtitle: "Review prompts that will run automatically, update their schedule, or turn them off.".to_string(),
                    is_empty: tasks.is_empty(),
                    empty_text: "No scheduled tasks yet. Ask in chat to create one.".to_string(),
                }
                if !tasks.is_empty() {
                    div { class: "mt-6 flex flex-col gap-3",
                        for task in tasks {
                            ScheduledTaskCard { team_id: team_id.clone(), task }
                        }
                    }
                }
            }
        }
    };
    crate::render(rendered)
}

#[component]
fn ScheduledTaskCard(team_id: String, task: ScheduledTask) -> Element {
    let edit_trigger_id = format!("edit-scheduled-task-{}", task.id);
    let delete_trigger_id = format!("delete-scheduled-task-{}", task.id);
    let status = if task.enabled { "Enabled" } else { "Disabled" };
    let status_color = if task.enabled {
        BadgeColor::Success
    } else {
        BadgeColor::Neutral
    };
    let next_run = task.next_run_at.to_rfc3339();

    rsx! {
        div {
            CardItem {
                class: Some("w-full".to_string()),
                avatar_name: Some("S".to_string()),
                title: task.name.clone(),
                description: Some(rsx! {
                    div { class: "flex flex-wrap items-center gap-2",
                        Badge { badge_color: status_color, badge_style: BadgeStyle::Outline, badge_size: BadgeSize::Sm, "{status}" }
                        span { class: "font-mono text-xs", "{task.cron}" }
                        span { class: "text-xs", "{task.timezone}" }
                    }
                }),
                footer: Some(rsx! {
                    div { class: "space-y-1",
                        p { class: "line-clamp-2 whitespace-normal", "{task.prompt}" }
                        p { "Next run: {next_run}" }
                    }
                }),
                count_labels: vec![],
                action: Some(rsx! {
                    div { class: "flex flex-col gap-2 sm:flex-row",
                        Button { button_scheme: ButtonScheme::Neutral, popover_target: "{edit_trigger_id}", "Edit" }
                        Button { button_scheme: ButtonScheme::Error, popover_target: "{delete_trigger_id}", "Delete" }
                    }
                }),
            }
            super::form::EditForm { trigger_id: edit_trigger_id, team_id: team_id.clone(), task: task.clone() }
            ConfirmModal {
                action: crate::routes::scheduled_tasks::Delete { team_id, id: task.id }.to_string(),
                trigger_id: delete_trigger_id,
                submit_label: "Delete task".to_string(),
                heading: "Delete scheduled task".to_string(),
                warning: format!("Delete {}? This cannot be undone.", task.name),
                hidden_fields: vec![],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    fn task() -> ScheduledTask {
        ScheduledTask {
            id: 1,
            user_id: 2,
            team_id: 3,
            project_id: None,
            model_id: 1,
            name: "Daily summary".to_string(),
            prompt: "Summarize the inbox".to_string(),
            cron: "0 8 * * *".to_string(),
            timezone: "Europe/Berlin".to_string(),
            enabled: true,
            next_run_at: DateTime::<Utc>::UNIX_EPOCH.fixed_offset(),
            last_run_at: None,
            created_at: DateTime::<Utc>::UNIX_EPOCH.fixed_offset(),
            updated_at: DateTime::<Utc>::UNIX_EPOCH.fixed_offset(),
        }
    }

    #[test]
    fn task_card_contains_schedule_and_actions() {
        let html = dioxus_ssr::render_element(rsx! {
            ScheduledTaskCard { team_id: "team".to_string(), task: task() }
        });
        assert!(html.contains("Daily summary"));
        assert!(html.contains("0 8 * * *"));
        assert!(html.contains("Europe/Berlin"));
        assert!(html.contains("Edit"));
        assert!(html.contains("Delete"));
    }
}
