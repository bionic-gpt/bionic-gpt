use dioxus::prelude::*;

#[component]
pub fn Shield(text: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 16 16",
            fill: "currentColor",
            stroke: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            width: "60",
            height: "60",
            g {
                id: "SVGRepo_bgCarrier",
                stroke_width: "0"
            }
            g {
                id: "SVGRepo_tracerCarrier",
                stroke_linecap: "round",
                stroke_linejoin: "round"
            }
            g {
                id: "SVGRepo_iconCarrier",
                path {
                    fill_rule: "evenodd",
                    clip_rule: "evenodd",
                    d: "M8 16L4.35009 13.3929C2.24773 11.8912 1 9.46667 1
                    6.88306V3L8 0L15 3V6.88306C15 9.46667 13.7523 11.8912 
                    11.6499 13.3929L8 16ZM12.2071 5.70711L10.7929 4.29289L7
                     8.08579L5.20711 6.29289L3.79289 7.70711L7 10.9142L12.2071 
                     5.70711Z",
                    fill: "currentColor"
                }
            }
        }
        h3 {
            class: "mt-4",
            "{text}"
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
                    class: "text-3xl tracking-tight text-primary mb-4",
                    "Built with Enterprise Security, Privacy, and Compliance at Its Core"
                }
                p {
                    class: "mb-4",
                    "Bionic-GPT was built with enterprise security, privacy, and compliance in mind from day one."
                }
                p {
                    "Choose Between On Premise or Private Cloud Deployment and keep your data 100% safe."
                }
            }
            div {
                class: "mt-12 md:mt-0 flex-1 grid grid-cols-2 gap-8",
                div {
                    class: "text-center block mx-auto",
                    Shield {
                        text: "ISO 27001"
                    }
                }
                div {
                    class: "text-center block mx-auto",
                    Shield {
                        text: "SOC II"
                    }
                }
                div {
                    class: "text-center block mx-auto",
                    Shield {
                        text: "GDPR"
                    }
                }
                div {
                    class: "text-center block mx-auto",
                    Shield {
                        text: "Compliance"
                    }
                }
            }
        }
    }
}
