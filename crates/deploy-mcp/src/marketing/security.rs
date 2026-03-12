use dioxus::prelude::*;

#[component]
pub fn Shield(title: String, subtitle: String, footer: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 200 200",
            width: "100",
            height: "100",
            xmlns: "http://www.w3.org/2000/svg",

            defs {
                path {
                    id: "curve",
                    d: "M20,98a80,80 0 0,0 160,0",
                    fill: "none"
                }
            }

            circle {
                cx: "100",
                cy: "100",
                r: "95",
                stroke: "currentColor",
                stroke_width: "3",
                fill: "none"
            }
            circle {
                cx: "100",
                cy: "100",
                r: "75",
                stroke: "currentColor",
                stroke_width: "3",
                fill: "none"
            }

            text {
                x: "100",
                y: "105",
                text_anchor: "middle",
                font_size: "32",
                font_family: "Arial",
                fill: "currentColor",
                font_weight: "bold",
                "{title}"
            }

            text {
                x: "100",
                y: "135",
                text_anchor: "middle",
                font_size: "20",
                font_family: "Arial",
                fill: "currentColor",
                font_weight: "bold",
                "{subtitle}"
            }

            text {
                font_size: "14",
                font_family: "Arial",
                fill: "currentColor",
                font_weight: "bold",
                textPath {
                    href: "#curve",
                    start_offset: "50%",
                    text_anchor: "middle",
                    dominant_baseline: "hanging",
                    "{footer}"
                }
            }
        }
    }
}

#[component]
pub fn Security(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        section {
            class: format!("{class} md:flex flex-row gap-8"),
            div {
                class: "flex-1",
                h2 {
                    class: "text-3xl tracking-tight font-display mb-4",
                    "Built with Enterprise Security, Privacy, and Compliance at Its Core"
                }
                p {
                    class: "mb-4",
                    "Deploy MCP is designed with enterprise-grade security, privacy, and compliance controls from day one."
                }
                p {
                    "Run fully managed in our cloud or deploy on premise to keep sensitive data inside your perimeter."
                }
            }
            div {
                class: "mt-12 md:mt-0 flex-1 grid grid-cols-2 gap-8",
                div {
                    class: "text-center block mx-auto",
                    Shield {
                        title: "SOC 2",
                        subtitle: "Type II",
                        footer: "Service Organisations"
                    }
                }
                div {
                    class: "text-center block mx-auto",
                    Shield {
                        title: "ISO",
                        subtitle: "27001",
                        footer: "Information Security"
                    }
                }
                div {
                    class: "text-center block mx-auto",
                    Shield {
                        title: "ISO",
                        subtitle: "27017",
                        footer: "Cloud Security"
                    }
                }
                div {
                    class: "text-center block mx-auto",
                    Shield {
                        title: "GDPR",
                        subtitle: "",
                        footer: "GDPR Compliant"
                    }
                }
            }
        }
    }
}
