use ssg_whiz::{
    FooterLinks, NavigationEntry, NavigationLink, NavigationMenu, NavigationModel, Section,
    SiteMeta,
};

pub struct NavigationLinks {
    pub home: String,
    pub pricing: String,
    pub blog: String,
    pub docs: String,
    pub architect_course: String,
    pub partners: String,
    pub contact: String,
    pub product_chat: String,
    pub product_assistants: String,
    pub product_integrations: String,
    pub product_automations: String,
    pub product_developers: String,
    pub sign_in_up: String,
}

impl NavigationLinks {
    fn into_model(self) -> NavigationModel {
        let Self {
            home,
            pricing,
            blog,
            docs,
            architect_course,
            partners,
            contact,
            product_chat,
            product_assistants,
            product_integrations,
            product_automations,
            product_developers,
            sign_in_up,
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
                        NavigationLink::new("Chat", product_chat, Section::None),
                        NavigationLink::new("Assistants", product_assistants, Section::None),
                        NavigationLink::new("Integrations", product_integrations, Section::None),
                        NavigationLink::new("Automations", product_automations, Section::None),
                        NavigationLink::new("Developers", product_developers, Section::None),
                    ],
                )),
                NavigationEntry::Link(NavigationLink::new(
                    "Pricing",
                    pricing.clone(),
                    Section::Pricing,
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
                NavigationEntry::Link(NavigationLink::new(
                    "Partners",
                    partners.clone(),
                    Section::Partners,
                )),
            ],
            desktop_right: vec![
                NavigationLink::external("GitHub", github_href, Section::None).with_badge_image(
                    "https://img.shields.io/github/stars/bionic-gpt/bionic-gpt",
                    "Github",
                ),
                NavigationLink::new("Login", sign_in_up.clone(), Section::None),
                NavigationLink::new("Book a Call", contact.clone(), Section::Contact)
                    .with_class("btn btn-primary btn-sm"),
            ],
            mobile: vec![
                NavigationLink::new("Home", home, Section::Home),
                NavigationLink::new("Pricing", pricing, Section::Pricing),
                NavigationLink::new("Blog", blog, Section::Blog),
                NavigationLink::new("Documentation", docs, Section::Docs),
                NavigationLink::new(
                    "Zero to Agentic AI Hero",
                    architect_course,
                    Section::ArchitectCourse,
                ),
                NavigationLink::new("Partners", partners, Section::Partners),
                NavigationLink::new("Book a Call", contact, Section::Contact),
                NavigationLink::external("Star us on GitHub", github_href, Section::None)
                    .with_class("shrink-0 flex gap-1 items-center underline pl-4"),
            ],
        }
    }
}

pub fn navigation_links() -> NavigationModel {
    NavigationLinks {
        home: crate::routes::marketing::Index {}.to_string(),
        pricing: crate::routes::marketing::Pricing {}.to_string(),
        blog: crate::routes::blog::Index {}.to_string(),
        docs: crate::routes::docs::Index {}.to_string(),
        architect_course: crate::routes::architect_course::Index {}.to_string(),
        partners: crate::routes::marketing::PartnersPage {}.to_string(),
        contact: crate::routes::marketing::Contact {}.to_string(),
        product_chat: crate::routes::product::Chat {}.to_string(),
        product_assistants: crate::routes::product::Assistants {}.to_string(),
        product_integrations: crate::routes::product::Integrations {}.to_string(),
        product_automations: crate::routes::product::Automations {}.to_string(),
        product_developers: crate::routes::product::Developers {}.to_string(),
        sign_in_up: crate::routes::SIGN_IN_UP.to_string(),
    }
    .into_model()
}

pub fn footer_links() -> FooterLinks {
    FooterLinks {
        blog: crate::routes::blog::Index {}.to_string(),
        pricing: crate::routes::marketing::Pricing {}.to_string(),
        contact: crate::routes::marketing::Contact {}.to_string(),
        terms: crate::routes::marketing::Terms {}.to_string(),
        privacy: crate::routes::marketing::Privacy {}.to_string(),
        about: None,
        variant: None,
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
