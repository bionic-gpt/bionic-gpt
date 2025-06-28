#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CountLabel {
    pub count: usize,
    pub label: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct CardItemProps {
    pub class: Option<String>,
    pub popover_target: Option<String>,
    pub image_src: Option<String>,
    pub avatar_name: Option<String>,

    pub title: String,
    pub description: Option<Element>,
    pub footer: Option<Element>,

    pub count_labels: Vec<CountLabel>,

    pub action: Option<Element>,
}

#[component]
pub fn CardItem(props: CardItemProps) -> Element {
    rsx! {
        Card {
            class: props.class.clone().unwrap_or_else(|| "p-3 mt-5 flex flex-row justify-between".to_string()),
            popover_target: props.popover_target.clone(),
            div {
                class: "flex flex-col items-center",
                if let Some(src) = props.image_src.clone() {
                    img {
                        class: "border border-neutral-content rounded p-2",
                        src: "{src}",
                        width: "48",
                        height: "48",
                    }
                } else if let Some(name) = props.avatar_name.clone() {
                    Avatar {
                        avatar_size: AvatarSize::Medium,
                        name: "{name}"
                    }
                }
            }
            div {
                class: "mx-4 flex flex-col flex-1 min-w-0",
                h2 { class: "font-semibold text-base mb-1 truncate", "{props.title}" }
                if let Some(desc) = props.description.clone() {
                    div { class: "text-sm text-base-content/70 truncate", {desc} }
                }
                if let Some(foot) = props.footer.clone() {
                    div { class: "text-xs text-base-content/70 truncate", {foot} }
                }
            }
            div {
                class: "flex flex-row items-center gap-5",
                for entry in props.count_labels.iter() {
                    div {
                        class: "flex flex-col justify-center text-center",
                        div { "{entry.count}" }
                        div {
                            class: "text-base-content/70",
                            "{entry.label}"
                            if entry.count != 1 {
                                "s"
                            }
                        }
                    }
                }
                if let Some(action) = props.action {
                    div { class: "ml-4 flex flex-col justify-center gap-2", {action} }
                }
            }
        }
    }
}
