use crate::marketing::{
    footer::Footer,
    layout::{MarketingSection, PageContainer, PageRhythm, PageWidth, SectionHeader},
    product_hero::ProductHero,
};
use crate::ui_links::footer_links;
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;
use ssg_whiz::Section;

pub fn page() -> String {
    let skills_url = crate::routes::product::Skills {}.to_string();
    let integrations_url = crate::routes::product::Integrations {}.to_string();
    let filesystem_tree = r#"├── attachments/
│   ├── index.json
│   └── <uploaded_file_name>
├── datasets/
│   ├── index.json
│   └── <dataset_id>/
│       ├── metadata.json
│       └── files/
│           └── <document_id>/
│               ├── metadata.json
│               └── chunks/
│                   └── <chunk_id>.txt
├── output/
│   └── <generated_file_or_directory>
├── work/
│   └── <working_file_or_directory>
└── skills/
    ├── <skill>/SKILL.md
    └── <connector>/
        ├── SKILL.md
        └── operations/<function>.md"#;

    let page = rsx! {
        Layout {
            title: "The Agent | Bionic",
            description: "Bionic brings together your organisation's skills, knowledge and systems in one place. Ask it to research, analyse, create or take action — it finds the context and tools it needs to get the work done.",
            mobile_menu: None,
            section: Section::Home,

            PageContainer { width: PageWidth::Wide, rhythm: Some(PageRhythm::Product), class: Some("mt-16 pb-16 md:mt-24".to_string()),
                section { class: "grid gap-8",
                    ProductHero {
                        eyebrow: "The Agent".to_string(),
                        title: "One AI that can work across your organisation".to_string(),
                        subtitle: "Bionic brings together your organisation's skills, knowledge and systems in one place. Ask it to research, analyse, create or take action — it finds the context and tools it needs to get the work done.".to_string(),
                        claim: Some("Open source. Self-hosted. Under your control.".to_string()),
                        supporting: None,
                        primary_cta: "Get started".to_string(),
                        primary_href: crate::routes::SIGN_IN_UP.to_string(),
                        secondary_cta: Some("Explore Skills".to_string()),
                        secondary_href: Some(skills_url.clone()),
                        image: "/blog/enterprise-integrations/screenshots/03-the-chat.png".to_string(),
                        image_alt: "Bionic finding Jordan Lee’s email in Gmail and updating her phone number in Salesforce".to_string(),
                    }
                }

                MarketingSection {
                    SectionHeader {
                        title: "Give your AI a computer.".to_string(),
                        body: Some("The Agent does more than call APIs. Each task can be given a workspace where it can read and create files, run code, use skills and integrations, and keep the context it needs while it works.".to_string()),
                    }
                    div { class: "grid items-start gap-6 lg:grid-cols-[0.85fr_1.15fr] lg:gap-10",
                        div { class: "grid gap-8 sm:grid-cols-2 lg:grid-cols-1",
                            article {
                                h3 { "A workspace for the task" }
                                p { class: "mt-3 leading-7 opacity-75", "Files, skills and context are made available when the Agent needs them. It can use shell, Python and other tools to work with those resources and create new artifacts." }
                            }
                            article {
                                h3 { "Secure execution" }
                                p { class: "mt-3 leading-7 opacity-75", "The workspace runs inside a controlled execution environment. Network access and credentials can be mediated by the platform, allowing the Agent to work without giving it unrestricted access to your infrastructure." }
                            }
                        }
                        div { class: "max-w-full overflow-x-auto rounded-xl border border-base-300 bg-neutral p-5 text-neutral-content sm:p-6",
                            pre { class: "whitespace-pre text-xs leading-6 sm:text-sm", code { "{filesystem_tree}" } }
                        }
                    }
                }

                MarketingSection { class: Some("border-y border-base-300 py-10 md:py-12".to_string()),
                    h2 { "Everything it needs to get the work done." }
                    div { class: "grid gap-8 sm:grid-cols-3 sm:gap-10",
                        article {
                            h3 { "Skills" }
                            p { class: "mt-3 leading-7 opacity-75", "Reusable instructions, methods and tools that teach the Agent how to perform specialised work." }
                            a { class: "mt-4 inline-block text-sm font-semibold text-primary hover:underline", href: "{skills_url}", "Explore Skills →" }
                        }
                        article {
                            h3 { "Integrations" }
                            p { class: "mt-3 leading-7 opacity-75", "Governed access to the APIs and systems your organisation already uses." }
                            a { class: "mt-4 inline-block text-sm font-semibold text-primary hover:underline", href: "{integrations_url}", "Explore Integrations →" }
                        }
                        article {
                            h3 { "Knowledge" }
                            p { class: "mt-3 leading-7 opacity-75", "Organisational information and context the Agent can use while working." }
                        }
                    }
                }

                section { class: "max-w-3xl",
                    h2 { "Your AI. Your infrastructure." }
                    p { class: "mt-5 text-lg leading-8 opacity-80", "Run Bionic on-premise, in your private cloud or in an air-gapped environment, with control over your data and the models you use." }
                }
            }

            Footer { margin_top: "mt-0", links: footer_links() }
        }
    };

    crate::render(page)
}
