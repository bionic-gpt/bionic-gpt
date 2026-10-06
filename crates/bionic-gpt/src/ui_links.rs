use ssg_whiz::{
    FooterLinks, NavigationEntry, NavigationLink, NavigationMenu, NavigationModel, Section,
    SiteMeta,
};

pub struct NavigationLinks {
    pub home: String,
    pub blog: String,
    pub docs: String,
    pub architect_course: String,
    pub go_bionic: String,
    pub product_chat: String,
    pub product_projects: String,
    pub product_datasets: String,
    pub product_skills: String,
    pub product_integrations: String,
    pub product_developers: String,
}

impl NavigationLinks {
    fn into_model(self) -> NavigationModel {
        let Self {
            home,
            blog,
            docs,
            architect_course,
            go_bionic,
            product_chat,
            product_projects,
            product_datasets,
            product_skills,
            product_integrations,
            product_developers,
        } = self;
        let github_href = "https://github.com/bionic-gpt/bionic-gpt";
        NavigationModel {
            home: home.clone(),
            logo_src: None,
            logo_alt: None,
            desktop_left: vec![
                NavigationEntry::Menu(NavigationMenu::new(
                    "Product",
                    vec![
                        NavigationLink::new("Chat", product_chat.clone(), Section::None),
                        NavigationLink::new(
                            "Integrations",
                            product_integrations.clone(),
                            Section::None,
                        ),
                        NavigationLink::new("Skills", product_skills.clone(), Section::None),
                        NavigationLink::new("Projects", product_projects.clone(), Section::None),
                        NavigationLink::new("Datasets", product_datasets.clone(), Section::None),
                        NavigationLink::new(
                            "Developers",
                            product_developers.clone(),
                            Section::None,
                        ),
                    ],
                )),
                NavigationEntry::Menu(NavigationMenu::new(
                    "Resources",
                    vec![
                        NavigationLink::new("Blog", blog.clone(), Section::Blog),
                        NavigationLink::new("Documentation", docs.clone(), Section::Docs),
                        NavigationLink::new(
                            "Zero to Agentic AI Hero",
                            architect_course.clone(),
                            Section::ArchitectCourse,
                        ),
                    ],
                )),
            ],
            desktop_right: vec![
                NavigationLink::external("GitHub", github_href, Section::None).with_badge_image(
                    "https://img.shields.io/github/stars/bionic-gpt/bionic-gpt",
                    "Github",
                ),
                NavigationLink::new("Go Bionic", go_bionic.clone(), Section::None)
                    .with_class("btn btn-primary btn-sm"),
            ],
            mobile: vec![
                NavigationLink::new("Chat", product_chat.clone(), Section::None),
                NavigationLink::new("Integrations", product_integrations.clone(), Section::None),
                NavigationLink::new("Skills", product_skills.clone(), Section::None),
                NavigationLink::new("Projects", product_projects.clone(), Section::None),
                NavigationLink::new("Datasets", product_datasets.clone(), Section::None),
                NavigationLink::new("Developers", product_developers.clone(), Section::None),
                NavigationLink::new("Blog", blog, Section::Blog),
                NavigationLink::new("Documentation", docs, Section::Docs),
                NavigationLink::new(
                    "Zero to Agentic AI Hero",
                    architect_course,
                    Section::ArchitectCourse,
                ),
                NavigationLink::external("GitHub", github_href, Section::None)
                    .with_class("shrink-0 flex gap-1 items-center underline pl-4"),
                NavigationLink::new("Go Bionic", go_bionic, Section::None)
                    .with_class("btn btn-primary btn-sm"),
            ],
        }
    }
}

pub fn navigation_links() -> NavigationModel {
    NavigationLinks {
        home: crate::routes::marketing::Index {}.to_string(),
        blog: crate::routes::blog::Index {}.to_string(),
        docs: crate::routes::docs::Index {}.to_string(),
        architect_course: crate::routes::architect_course::Index {}.to_string(),
        go_bionic: crate::routes::marketing::GoBionic {}.to_string(),
        product_chat: crate::routes::product::Chat {}.to_string(),
        product_projects: crate::routes::product::Projects {}.to_string(),
        product_datasets: crate::routes::product::Datasets {}.to_string(),
        product_skills: crate::routes::product::Skills {}.to_string(),
        product_integrations: crate::routes::product::Integrations {}.to_string(),
        product_developers: crate::routes::product::Developers {}.to_string(),
    }
    .into_model()
}

pub fn footer_links() -> FooterLinks {
    FooterLinks {
        blog: crate::routes::blog::Index {}.to_string(),
        pricing: crate::routes::marketing::GoBionic {}.to_string(),
        contact: crate::routes::marketing::Contact {}.to_string(),
        terms: crate::routes::marketing::Terms {}.to_string(),
        privacy: crate::routes::marketing::Privacy {}.to_string(),
        about: None,
    }
}

pub fn site_meta() -> SiteMeta {
    SiteMeta {
        base_url: "https://bionic-gpt.com".to_string(),
        site_name: "Bionic GPT".to_string(),
        brand_name: "Bionic".to_string(),
        goatcounter: "https://bionicgpt.goatcounter.com/count".to_string(),
    }
}
