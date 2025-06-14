#![allow(non_snake_case)]
use daisy_rsx::*;
use db::queries::models::ModelWithPrompt;
use dioxus::prelude::*;

#[component]
pub fn ModelCard(
    team_id: i32,
    model: ModelWithPrompt,
    has_function_calling: bool,
    has_vision: bool,
    has_tool_use: bool,
) -> Element {
    let display_name = if model.display_name.is_empty() {
        model.name.clone()
    } else {
        model.display_name.clone()
    };
    let description: String = model
        .description
        .chars()
        .filter(|&c| c != '\n' && c != '\t' && c != '\r')
        .collect();

    rsx!(
        Card {
            class: "p-3 mt-5 flex flex-row justify-between",
            div {
                class: "flex flex-row",
                Avatar { avatar_size: AvatarSize::Medium, avatar_type: AvatarType::User }
                div {
                    class: "ml-4 text-sm flex flex-col justify-center flex-1 min-w-0",
                    h2 { class: "font-semibold text-base mb-1", "{display_name}" }
                    if !description.is_empty() {
                        p { class: "text-sm text-base-content/70 truncate mb-2", "{description}" }
                    }
                    div { class: "flex items-center gap-2 text-xs text-gray-500", super::model_type::Model { model_type: model.model_type } }
                    div {
                        class: "flex gap-2 mt-1 text-xs",
                        if has_function_calling { span { class: "badge badge-ghost", "Functions" } }
                        if has_vision { span { class: "badge badge-ghost", "Vision" } }
                        if has_tool_use { span { class: "badge badge-ghost", "Tools" } }
                    }
                }
            }
            div {
                class: "flex flex-row gap-5",
                div {
                    class: "flex flex-col justify-center text-center",
                    div { "{model.tpm_limit}" }
                    div { class: "text-base-content/70", "TPM" }
                }
                div {
                    class: "flex flex-col justify-center text-center",
                    div { "{model.rpm_limit}" }
                    div { class: "text-base-content/70", "RPM" }
                }
                div {
                    class: "flex flex-col justify-center text-center",
                    div { "{model.context_size}" }
                    div { class: "text-base-content/70", "Context" }
                }
                div {
                    class: "flex flex-col justify-center ml-4 gap-2",
                    DropDown {
                        direction: Direction::Bottom,
                        button_text: "...",
                        DropDownLink {
                            href: crate::routes::models::Edit{team_id, id: model.id}.to_string(),
                            "Edit"
                        }
                        DropDownLink {
                            popover_target: format!("delete-trigger-{}-{}", model.id, team_id),
                            href: "#",
                            target: "_top",
                            "Delete"
                        }
                    }
                }
            }
        }
    )
}
