#![allow(non_snake_case)]
use daisy_rsx::*;
use db::queries::token_usage_metrics::{DailyApiRequests, DailyTokenUsage};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn TokenUsageChart(data: Vec<DailyTokenUsage>) -> Element {
    // Process data to group by date and separate prompt/completion tokens
    let mut daily_data: HashMap<time::Date, (i64, i64)> = HashMap::new();

    for item in &data {
        let entry = daily_data.entry(item.usage_date).or_insert((0, 0));
        match item.token_type {
            db::TokenUsageType::Prompt => entry.0 += item.total_tokens,
            db::TokenUsageType::Completion => entry.1 += item.total_tokens,
        }
    }

    let mut sorted_data: Vec<_> = daily_data.into_iter().collect();
    sorted_data.sort_by_key(|&(date, _)| date);

    let max_tokens = sorted_data
        .iter()
        .map(|(_, (prompt, completion))| prompt + completion)
        .max()
        .unwrap_or(1);

    rsx! {
        div {
            class: "w-full h-64",
            svg {
                width: "100%",
                height: "100%",
                view_box: "0 0 400 200",
                // SVG chart implementation with stacked bars
                for (i, (date, (prompt_tokens, completion_tokens))) in sorted_data.iter().enumerate() {
                    g {
                        // Stacked bar for prompt tokens (bottom)
                        rect {
                            x: "{i * 50 + 20}",
                            y: "{200 - (prompt_tokens * 180 / max_tokens)}",
                            width: "40",
                            height: "{prompt_tokens * 180 / max_tokens}",
                            fill: "#3b82f6",
                            class: "hover:opacity-80 cursor-pointer"
                        }
                        // Stacked bar for completion tokens (top)
                        rect {
                            x: "{i * 50 + 20}",
                            y: "{200 - ((prompt_tokens + completion_tokens) * 180 / max_tokens)}",
                            width: "40",
                            height: "{completion_tokens * 180 / max_tokens}",
                            fill: "#10b981",
                            class: "hover:opacity-80 cursor-pointer"
                        }
                        text {
                            x: "{i * 50 + 40}",
                            y: "195",
                            text_anchor: "middle",
                            font_size: "10",
                            "{date.month()}/{date.day()}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ApiRequestChart(data: Vec<DailyApiRequests>) -> Element {
    let max_requests = data.iter().map(|d| d.request_count).max().unwrap_or(1);

    rsx! {
        div {
            class: "w-full h-64",
            svg {
                width: "100%",
                height: "100%",
                view_box: "0 0 400 200",
                // SVG chart implementation with simple bars
                for (i, day_data) in data.iter().enumerate() {
                    g {
                        rect {
                            x: "{i * 50 + 20}",
                            y: "{200 - (day_data.request_count * 180 / max_requests)}",
                            width: "40",
                            height: "{day_data.request_count * 180 / max_requests}",
                            fill: "#6366f1",
                            class: "hover:opacity-80 cursor-pointer"
                        }
                        text {
                            x: "{i * 50 + 40}",
                            y: "195",
                            text_anchor: "middle",
                            font_size: "10",
                            "{day_data.request_date.month()}/{day_data.request_date.day()}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TokenUsageChartCard(data: Vec<DailyTokenUsage>, title: String) -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "{title}"
            }
            CardBody {
                TokenUsageChart {
                    data: data
                }
                div {
                    class: "flex justify-center mt-4 space-x-4",
                    div {
                        class: "flex items-center",
                        div {
                            class: "w-4 h-4 bg-blue-500 mr-2"
                        }
                        span {
                            class: "text-sm",
                            "Prompt Tokens"
                        }
                    }
                    div {
                        class: "flex items-center",
                        div {
                            class: "w-4 h-4 bg-green-500 mr-2"
                        }
                        span {
                            class: "text-sm",
                            "Completion Tokens"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ApiRequestChartCard(data: Vec<DailyApiRequests>, title: String) -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "{title}"
            }
            CardBody {
                ApiRequestChart {
                    data: data
                }
            }
        }
    }
}
