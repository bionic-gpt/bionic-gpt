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
            title: String::from("Agentic Assistants"),
            description: String::from("Connect assistants to your systems and your data."),
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
