use crate::integrations::{catalogue, Integration};
use crate::marketing::footer::Footer;
use crate::ui_links::footer_links;
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;
use ssg_whiz::Section;

#[derive(Clone, Copy)]
struct SkillCard {
    title: &'static str,
    description: &'static str,
    icon: &'static str,
}

struct SkillGroup {
    title: &'static str,
    skills: &'static [SkillCard],
}

const SKILL_GROUPS: &[SkillGroup] = &[
    SkillGroup {
        title: "Documents",
        skills: &[
            SkillCard {
                title: "Document co-authoring",
                description:
                    "Plan and write proposals, specifications, RFCs and decision documents.",
                icon: "document",
            },
            SkillCard {
                title: "Document comparison",
                description:
                    "Compare documents against rubrics and references with traceable evidence.",
                icon: "compare",
            },
            SkillCard {
                title: "Document and PDF generation",
                description: "Create polished printable reports, forms, checklists and PDFs.",
                icon: "file",
            },
            SkillCard {
                title: "Structured extraction",
                description: "Extract source-located facts from uploaded documents for analysis.",
                icon: "extract",
            },
        ],
    },
    SkillGroup {
        title: "Data & analysis",
        skills: &[
            SkillCard {
                title: "Dataset analysis",
                description: "Find and use evidence from your assistant's indexed datasets.",
                icon: "search",
            },
            SkillCard {
                title: "File analysis",
                description: "Inspect, filter, summarize and transform files in the workspace.",
                icon: "table",
            },
            SkillCard {
                title: "Database work",
                description: "Create, query and maintain SQLite databases and structured data.",
                icon: "database",
            },
        ],
    },
    SkillGroup {
        title: "Presentations & visual analysis",
        skills: &[
            SkillCard {
                title: "Presentation builder",
                description: "Create reveal.js slide decks and presentation-style artifacts.",
                icon: "presentation",
            },
            SkillCard {
                title: "Image analysis",
                description:
                    "Answer questions grounded in uploaded images, diagrams and screenshots.",
                icon: "image",
            },
        ],
    },
];

fn skill_icon(icon: &str) -> Element {
    let path = match icon {
        "compare" => "M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01",
        "file" => {
            "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8zM14 2v6h6M8 13h8M8 17h8"
        }
        "extract" => {
            "M8 3H5a2 2 0 0 0-2 2v3m13-5h3a2 2 0 0 1 2 2v3M3 16v3a2 2 0 0 0 2 2h3m8 0h3a2 2 0 0 0 2-2v-3M8 12h8M12 8v8"
        }
        "search" => "M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16zM21 21l-4.35-4.35",
        "table" => "M3 3h18v18H3zM3 9h18M3 15h18M9 3v18M15 3v18",
        "database" => {
            "M12 3c-4.97 0-9 1.34-9 3v12c0 1.66 4.03 3 9 3s9-1.34 9-3V6c0-1.66-4.03-3-9-3zM3 6c0 1.66 4.03 3 9 3s9-1.34 9-3M3 12c0 1.66 4.03 3 9 3s9-1.34 9-3"
        }
        "presentation" => "M3 4h18v12H3zM12 16v5M8 21h8M7 9h4m2 0h4m-10 3h10",
        "image" => "M4 4h16v16H4zM8.5 10a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3zM20 15l-5-5L5 20",
        _ => "M4 4h16v16H4zM8 8h8v8H8z",
    };
    rsx! {
        div { class: "grid size-11 shrink-0 place-items-center rounded-field border border-base-300 bg-base-100 text-primary",
            svg { class: "size-5", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.7", stroke_linecap: "round", stroke_linejoin: "round", path { d: "{path}" } }
        }
    }
}

fn integration_logo(integration: &Integration) -> Element {
    if integration.logo_url.is_empty() {
        rsx! { div { class: "grid size-7 place-items-center rounded-field border border-base-300 bg-base-100 text-primary text-xs font-bold", "{integration.title.chars().next().unwrap_or('•')}" } }
    } else {
        rsx! { img { class: "size-7 shrink-0 object-contain svg-icon", src: "{integration.logo_url}", alt: "", loading: "lazy" } }
    }
}

fn workflow_card(
    title: &'static str,
    skills: &'static str,
    integrations: &'static [&'static str],
    result: &'static str,
    catalogue: &[Integration],
) -> Element {
    rsx! {
        article { class: "card card-border h-full",
            div { class: "card-body gap-4 p-5",
                h3 { class: "card-title text-lg", "{title}" }
                div { class: "grid gap-3 sm:grid-cols-[1fr_auto_1fr] sm:items-center",
                    div { class: "rounded-box bg-base-200/60 p-3",
                        p { class: "mb-1 text-xs font-semibold uppercase tracking-wide opacity-60", "Skills" }
                        p { class: "text-sm font-medium", "{skills}" }
                    }
                    span { class: "text-center text-primary", "+" }
                    div { class: "rounded-box bg-base-200/60 p-3",
                        p { class: "mb-2 text-xs font-semibold uppercase tracking-wide opacity-60", "Integrations" }
                        div { class: "flex flex-wrap items-center gap-2",
                            for slug in integrations {
                                if let Some(integration) = catalogue.iter().find(|item| item.slug == *slug) {
                                    {integration_logo(integration)}
                                }
                            }
                        }
                    }
                }
                div { class: "flex items-center gap-3 border-t border-base-300 pt-3",
                    span { class: "text-lg text-primary", "→" }
                    p { class: "text-sm font-semibold", "{result}" }
                }
            }
        }
    }
}

pub fn page() -> String {
    let integrations = catalogue();
    let page = rsx! {
        Layout {
            title: "Skills | Bionic",
            description: "Explore reusable skills that help Bionic analyse information, create documents and presentations, and complete work with your connected systems.",
            mobile_menu: None,
            section: Section::Home,

            main { class: "mx-auto mt-16 grid w-full max-w-6xl gap-16 px-4 pb-16 md:mt-24 md:px-6 md:gap-20",
                header { class: "max-w-3xl",
                    p { class: "badge badge-outline", "Skills" }
                    h1 { class: "mt-5 text-4xl font-bold tracking-tight sm:text-5xl", "What can Bionic do?" }
                    p { class: "mt-5 text-lg leading-8 opacity-80", "Skills give a model a clear method for recurring work: how to analyse information, create useful documents and follow the workflows your organisation relies on." }
                    p { class: "mt-4 max-w-2xl text-sm leading-6 opacity-70", "For well-scoped tasks, that guidance can help a smaller model follow a reliable process—so you can match the model to the work." }
                }

                section { class: "grid gap-8",
                    div {
                        p { class: "badge badge-outline", "Capabilities" }
                        h2 { class: "mt-4 text-3xl font-bold tracking-tight", "Methods for the work your team does" }
                        p { class: "mt-3 max-w-2xl leading-7 opacity-75", "Browse the kinds of work a model can take on with reusable guidance. A skill explains when it applies and gives the model a workflow to follow with the tools and information available to your team." }
                    }
                    for group in SKILL_GROUPS {
                        div { class: "grid gap-4",
                            h3 { class: "text-xl font-bold tracking-tight", "{group.title}" }
                            div { class: "grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3",
                                for skill in group.skills {
                                    article { class: "card card-border h-full",
                                        div { class: "card-body gap-3 p-4",
                                            div { class: "flex min-w-0 items-center gap-3", {skill_icon(skill.icon)} h4 { class: "font-semibold", "{skill.title}" } }
                                            p { class: "text-sm leading-6 opacity-75", "{skill.description}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                section { class: "grid gap-8",
                    div {
                        p { class: "badge badge-outline", "Skills + integrations" }
                        h2 { class: "mt-4 text-3xl font-bold tracking-tight", "Combine know-how with the systems where work happens" }
                        p { class: "mt-3 max-w-2xl leading-7 opacity-75", "Describe the work. The model follows the relevant skill, uses your connected systems and produces a useful result." }
                    }
                    div { class: "grid gap-4 lg:grid-cols-3",
                        {workflow_card("Customer report", "Dataset analysis + document generation", &["salesforce", "google-sheets"], "A customer report", &integrations)}
                        {workflow_card("Evidence-backed review", "Structured extraction + document comparison", &["google-drive"], "A sourced comparison", &integrations)}
                        {workflow_card("Presentation briefing", "Dataset analysis + presentation builder", &["google-drive", "google-slides"], "A presentation artifact", &integrations)}
                    }
                }

                section { class: "grid gap-6 rounded-box border border-base-300 bg-base-200/30 p-6 md:grid-cols-2 md:items-center md:p-8",
                    div {
                        p { class: "badge badge-outline", "Build your own" }
                        h2 { class: "mt-4 text-3xl font-bold tracking-tight", "Package the way your organisation works" }
                        p { class: "mt-4 leading-7 opacity-80", "Package a repeatable category of work in SKILL.md. Explain when the method applies and how the model should carry it out, then add supporting references, templates, examples or executable helpers. The model can load the guidance when it matches the task." }
                        a { class: "btn btn-outline btn-sm mt-5 w-fit", href: "/architect-course/ai-computer/skills/", "Learn how skills work →" }
                    }
                    div { class: "overflow-x-auto rounded-box border border-base-300 bg-base-100 p-5 font-mono text-sm leading-7",
                        pre { "my-skill/\n├── SKILL.md\n├── references/\n│   └── guidance.md\n├── templates/\n│   └── report.typ\n├── examples/\n│   └── sample.md\n└── bin/\n    └── helper.sh" }
                    }
                }
            }

            Footer { margin_top: "mt-0", links: footer_links() }
        }
    };

    crate::render(page)
}
