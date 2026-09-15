use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Feature {
    pub title: String,
    pub description: String,
    pub icon: String,
}

#[component]
pub fn Features(
    features: Vec<Feature>,
    title: String,
    description: String,
    class: Option<String>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        section {
            class: format!("{class} body-font"),
            div {
                class: "mx-auto",
                div {
                    class: "mb-8 lg:mb-16",
                    h2 {
                        class: "mb-4 text-4xl tracking-tight font-display",
                        "{title}"
                    }
                    p {
                        class: "text-gray-500 sm:text-xl dark:text-gray-400",
                        "{description}"
                    }
                }
                div {
                    class: "space-y-8 md:grid md:grid-cols-2 lg:grid-cols-3 md:gap-12 md:space-y-0",
                    for feature in features {
                        div {
                            div {
                                class: "mb-4 w-10 h-10 lg:h-12 lg:w-12",
                                img {
                                    alt: "testimonial",
                                    src: "{feature.icon}"
                                }
                            }
                            h3 {
                                class: "mb-2 font-display text-xl font-bold",
                                "{feature.title}"
                            }
                            p {
                                "{feature.description}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BionicFeatures(class: Option<String>) -> Element {
    let features = vec![
        Feature {
            title: String::from("Persistent Projects"),
            description: String::from(
                "Organize related chats, instructions, attachments, and history.",
            ),
            icon: "/features/systems.svg".to_string(),
        },
        Feature {
            title: String::from("Team-Based Permissions"),
            description: String::from(
                "Control data access and ensure security by allowing teams to manage permissions.",
            ),
            icon: "/features/team.svg".to_string(),
        },
        Feature {
            title: String::from("Full Observability"),
            description: String::from(
                "Gain insights into usage and compliance with detailed dashboards and logs.",
            ),
            icon: "/features/graph.svg".to_string(),
        },
        Feature {
            title: String::from("Cost Control"),
            description: String::from(
                "Set usage limits by user and team to manage costs effectively.",
            ),
            icon: "/features/costs.svg".to_string(),
        },
        Feature {
            title: String::from("Advanced Encryption"),
            description: String::from(
                "Ensure data security with encryption at rest, in transit, and during runtime.",
            ),
            icon: "/features/encryption.svg".to_string(),
        },
        Feature {
            title: String::from("Scalable Architecture"),
            description: String::from(
                "Built on Kubernetes for maximum scalability and reliability.",
            ),
            icon: "/features/kubernetes.svg".to_string(),
        },
    ];

    rsx! {
        Features {
            title: "Bionic Features",
            description: "A comprehensive solution for all your AI needs.",
            features: features,
            class: class
        }
    }
}

#[derive(Clone, PartialEq)]
struct MeetingBriefVignette {
    title: String,
    description: String,
    image: String,
    image_alt: String,
    image_left: bool,
}

fn meeting_brief_vignettes() -> Vec<MeetingBriefVignette> {
    vec![
        MeetingBriefVignette {
            title: "Connect your existing tools without custom code".to_string(),
            description: "Give Bionic an OpenAPI specification and it can discover and use the API. Connect systems such as Gmail, Salesforce and your internal applications without building a bespoke agent for every workflow.".to_string(),
            image: "/landing-page/meeting-brief.svg".to_string(),
            image_alt: "A chat prompt followed by a meeting brief with attendees, context and agenda".to_string(),
            image_left: false,
        },
        MeetingBriefVignette {
            title: "Built for long-horizon work".to_string(),
            description: "Bionic is designed to run demanding agent benchmarks such as AutomationBench. That means it can follow multi-step workflows, use several business systems and maintain context until the task is complete.".to_string(),
            image: "/landing-page/jordan-lee-eval.svg".to_string(),
            image_alt: "A chat request uses Gmail to find a phone number and updates the matching Salesforce contact".to_string(),
            image_left: true,
        },
        MeetingBriefVignette {
            title: "Built for work that challenges frontier models".to_string(),
            description: "Mercor’s APEX benchmarks test agents on multi-hour professional tasks in areas such as investment banking, consulting and law. Bionic skills give models repeatable methods for completing this work and producing the artifacts professionals expect.".to_string(),
            image: "/landing-page/apex-skill-lift.svg".to_string(),
            image_alt: "An investment banking task uses a specialist skill, source files and a spreadsheet to produce editable deliverables".to_string(),
            image_left: false,
        },
    ]
}

#[component]
pub fn CapabilityVignettes(class: Option<String>) -> Element {
    let class = class.unwrap_or_default();

    rsx! {
        section {
            class: format!("{class} grid gap-12"),
            for vignette in meeting_brief_vignettes() {
                div {
                    class: "grid gap-8 md:grid-cols-2 md:items-center",
                    div {
                        class: if vignette.image_left { "order-2 max-w-md md:order-2" } else { "order-2 max-w-md md:order-1" },
                        h2 {
                            class: "text-3xl font-bold tracking-tight sm:text-4xl",
                            "{vignette.title}"
                        }
                        p {
                            class: "mt-4 text-lg leading-8 opacity-80",
                            "{vignette.description}"
                        }
                    }
                    div {
                        class: if vignette.image_left { "order-1 overflow-hidden rounded-2xl bg-primary p-3 shadow-lg sm:p-5 md:order-1" } else { "order-1 overflow-hidden rounded-2xl bg-primary p-3 shadow-lg sm:p-5 md:order-2" },
                        img {
                            class: "h-auto w-full",
                            src: "{vignette.image}",
                            alt: "{vignette.image_alt}"
                        }
                    }
                }
            }
        }
    }
}
