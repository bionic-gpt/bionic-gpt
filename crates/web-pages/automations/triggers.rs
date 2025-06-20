#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::automation_triggers::CronTrigger;
use dioxus::prelude::*;

pub fn page(team_id: i32, prompt_id: i32, prompt_name: String, rbac: Rbac, triggers: Vec<CronTrigger>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Automation Schedule",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem { text: "Automations".into(), href: Some(crate::routes::automations::Index { team_id }.to_string()) },
                        BreadcrumbItem { text: prompt_name.clone(), href: None },
                    ]
                }
            ),

            div {
                class: "p-4 max-w-xl w-full mx-auto space-y-6",

                Card {
                    CardHeader { title: "Add Cron Trigger" }
                    CardBody {
                        form {
                            class: "flex flex-col gap-4",
                            method: "post",
                            action: crate::routes::automations::AddCronTrigger { team_id, prompt_id }.to_string(),

                            div { class: "grid grid-cols-5 gap-2", 
                                Select { name: "minute", 
                                    option { value: "*", "*" }
                                    (0..60).for_each(|i| rsx!( option { value: "{i}", "{i}" } ));
                                }
                                Select { name: "hour", 
                                    option { value: "*", "*" }
                                    (0..24).for_each(|i| rsx!( option { value: "{i}", "{i}" } ));
                                }
                                Select { name: "day", 
                                    option { value: "*", "*" }
                                    (1..32).for_each(|i| rsx!( option { value: "{i}", "{i}" } ));
                                }
                                Select { name: "month", 
                                    option { value: "*", "*" }
                                    (1..13).for_each(|i| rsx!( option { value: "{i}", "{i}" } ));
                                }
                                Select { name: "weekday", 
                                    option { value: "*", "*" }
                                    (0..7).for_each(|i| rsx!( option { value: "{i}", "{i}" } ));
                                }
                            }
                            Button { button_type: ButtonType::Submit, button_scheme: ButtonScheme::Primary, "Add Trigger" }
                        }
                    }
                }

                if !triggers.is_empty() {
                    Card {
                        CardHeader { title: "Existing Triggers" }
                        CardBody {
                            table {
                                class: "table table-sm w-full",
                                thead { tr { th { "Cron" } th { "Action" } } }
                                tbody {
                                    for trigger in &triggers {
                                        tr {
                                            td { "{trigger.cron_expression}" }
                                            td {
                                                form {
                                                    method: "post",
                                                    action: crate::routes::automations::RemoveCronTrigger { team_id, prompt_id, trigger_id: trigger.id }.to_string(),
                                                    Button { button_type: ButtonType::Submit, button_scheme: ButtonScheme::Error, button_size: ButtonSize::Small, "Delete" }
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
        }
    };
    crate::render(page)
}
