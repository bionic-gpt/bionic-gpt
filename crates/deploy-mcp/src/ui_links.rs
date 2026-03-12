use ssg_whiz::{
    FooterLinks, NavigationEntry, NavigationLink, NavigationMenu, NavigationModel, Section,
    SiteMeta,
};

pub fn navigation_links() -> NavigationModel {
    let pricing = crate::routes::marketing::Pricing {}.to_string();
    let blog = crate::routes::blog::Index {}.to_string();
    let docs = crate::routes::docs::Index {}.to_string();
    let enterprise = crate::routes::marketing::Enterprise {}.to_string();
    let mcp_servers = crate::routes::marketing::McpServers {}.to_string();
    let contact = crate::routes::marketing::Contact {}.to_string();
    let sign_in = crate::routes::SIGN_IN_UP.to_string();

    NavigationModel {
        home: crate::routes::marketing::Index {}.to_string(),
        logo_src: None,
        logo_alt: None,
        desktop_left: vec![
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
                ],
            )),
            NavigationEntry::Link(NavigationLink::new(
                "Enterprise",
                enterprise.clone(),
                Section::Enterprise,
            )),
            NavigationEntry::Link(NavigationLink::new(
                "MCP Servers",
                mcp_servers.clone(),
                Section::McpServers,
            )),
        ],
        desktop_right: vec![
            NavigationLink::new("Login", sign_in.clone(), Section::None),
            NavigationLink::new("Contact", contact.clone(), Section::Contact)
                .with_class("btn btn-primary btn-sm"),
        ],
        mobile: vec![
            NavigationLink::new(
                "Home",
                crate::routes::marketing::Index {}.to_string(),
                Section::Home,
            ),
            NavigationLink::new("Pricing", pricing, Section::Pricing),
            NavigationLink::new("Blog", blog, Section::Blog),
            NavigationLink::new("Documentation", docs, Section::Docs),
            NavigationLink::new("Enterprise", enterprise, Section::Enterprise),
            NavigationLink::new("MCP Servers", mcp_servers, Section::McpServers),
            NavigationLink::new("Contact", contact, Section::Contact),
            NavigationLink::new("Login", sign_in, Section::None),
        ],
    }
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
        base_url: "https://deploy.run".to_string(),
        site_name: "Deploy".to_string(),
        brand_name: "Deploy".to_string(),
        goatcounter: "https://deploy.goatcounter.com/count".to_string(),
    }
}
