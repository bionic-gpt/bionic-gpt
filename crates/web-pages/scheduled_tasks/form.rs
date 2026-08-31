#![allow(non_snake_case)]

use daisy_rsx::*;
use db::queries::scheduled_tasks::ScheduledTask;
use dioxus::prelude::*;

#[component]
pub fn EditForm(trigger_id: String, team_id: String, task: ScheduledTask) -> Element {
    rsx! {
        form {
            action: crate::routes::scheduled_tasks::Update { team_id }.to_string(),
            method: "post",
            Modal {
                trigger_id,
                ModalBody {
                    h3 { class: "font-bold text-lg mb-4", "Edit scheduled task" }
                    input { r#type: "hidden", name: "id", value: "{task.id}" }
                    div { class: "grid grid-cols-1 gap-4 md:grid-cols-2",
                        Fieldset {
                            legend: "Name",
                            input { class: "input input-bordered w-full", name: "name", value: "{task.name}", required: true }
                        }
                        Fieldset {
                            legend: "Status",
                            Select { class: "w-full", name: "enabled", value: "{task.enabled}",
                                SelectOption { value: "true", selected_value: "{task.enabled}", "Enabled" }
                                SelectOption { value: "false", selected_value: "{task.enabled}", "Disabled" }
                            }
                        }
                        Fieldset {
                            legend: "Cron",
                            input { class: "input input-bordered w-full font-mono", name: "cron", value: "{task.cron}", required: true }
                        }
                        Fieldset {
                            legend: "Timezone",
                            input { class: "input input-bordered w-full", name: "timezone", value: "{task.timezone}", required: true }
                        }
                    }
                    Fieldset {
                        class: "mt-4",
                        legend: "Prompt",
                        textarea { class: "textarea textarea-bordered min-h-32 w-full", name: "prompt", required: true, "{task.prompt}" }
                    }
                    ModalAction {
                        Button { class: "cancel-modal", button_scheme: ButtonScheme::Warning, "Cancel" }
                        Button { button_type: ButtonType::Submit, button_scheme: ButtonScheme::Primary, "Save task" }
                    }
                }
            }
        }
    }
}
