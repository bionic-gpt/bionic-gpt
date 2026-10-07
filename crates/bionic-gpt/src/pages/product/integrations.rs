use crate::integrations::{catalogue, Integration};
use crate::marketing::{
    footer::Footer,
    layout::{MarketingSection, PageContainer, PageRhythm, PageWidth},
    product_hero::ProductHero,
};
use crate::ui_links::footer_links;
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;
use ssg_whiz::Section;

fn integration_logo(integration: &Integration, compact: bool) -> Element {
    if integration.logo_url.is_empty() {
        rsx! {
            div {
                class: if compact { "grid size-9 shrink-0 place-items-center rounded-field border border-base-300 bg-base-100 text-primary" } else { "grid size-11 shrink-0 place-items-center rounded-field border border-base-300 bg-base-100 text-primary" },
                svg {
                    class: "size-6",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.7",
                    path { d: "M12 3v3m0 12v3M3 12h3m12 0h3M5.64 5.64l2.12 2.12m8.48 8.48 2.12 2.12m0-12.72-2.12 2.12m-8.48 8.48-2.12 2.12" }
                    circle { cx: "12", cy: "12", r: "4" }
                }
            }
        }
    } else if compact {
        rsx! {
            img {
                class: "size-8 shrink-0 object-contain svg-icon",
                src: "{integration.logo_url}",
                alt: "",
                loading: "lazy",
            }
        }
    } else {
        rsx! {
            div {
                class: "grid size-11 shrink-0 place-items-center rounded-field border border-base-300 bg-base-100",
                img {
                    class: "size-6 svg-icon",
                    src: "{integration.logo_url}",
                    alt: "",
                    loading: "lazy",
                }
            }
        }
    }
}

pub fn page() -> String {
    let integrations = catalogue();
    let count = integrations.len();
    let docs_url = "/docs/integrations/curated/";
    let create_url = "/docs/integrations/creating/";

    let page = rsx! {
        Layout {
            title: "OpenAPI Integrations | Bionic",
            description: "Connect Bionic to the APIs your business already uses with curated OpenAPI integrations or your own API specification.",
            mobile_menu: None,
            section: Section::Home,

            PageContainer { width: PageWidth::Wide, rhythm: Some(PageRhythm::Integrations), class: Some("mt-16 pb-16 md:mt-24".to_string()),
                ProductHero {
                    eyebrow: "Integrations".to_string(),
                    title: "Connect Bionic to the tools your business already uses".to_string(),
                    subtitle: "Use an OpenAPI specification to give Bionic governed access to business systems and APIs.".to_string(),
                    claim: Some(format!("{count} curated OpenAPI integrations")),
                    supporting: None,
                    primary_cta: "Browse integrations".to_string(),
                    primary_href: docs_url.to_string(),
                    secondary_cta: Some("Create an integration".to_string()),
                    secondary_href: Some(create_url.to_string()),
                    image: "/product/integrations.png".to_string(),
                    image_alt: "Bionic integrations catalog".to_string(),
                }

                MarketingSection { class: Some("site-marketing-section--compact md:grid-cols-2".to_string()),
                    article {
                        class: "card card-border",
                        div {
                            class: "card-body gap-5",
                            div {
                                class: "flex flex-wrap gap-2",
                                for slug in [
                                    "salesforce",
                                    "google-gmail",
                                    "google-calendar",
                                    "google-drive",
                                    "microsoft-teams",
                                ] {
                                    if let Some(integration) = integrations
                                        .iter()
                                        .find(|integration| integration.slug == slug)
                                    {
                                        {integration_logo(integration, false)}
                                    }
                                }
                            }
                            div {
                                h3 { class: "card-title", "Ready-to-use integrations" }
                                p {
                                    class: "mt-2 leading-7 opacity-80",
                                    "Start with a curated collection of vendor OpenAPI specifications for common enterprise systems. Upload a spec to Bionic to make its API operations available as tools."
                                }
                            }
                            a {
                                class: "btn btn-primary btn-sm mt-auto w-fit",
                                href: "{docs_url}",
                                "Installing Integrations"
                            }
                        }
                    }
                    article {
                        class: "card card-border",
                        div {
                            class: "card-body gap-5",
                            div {
                                class: "grid size-12 place-items-center rounded-box border border-base-300 bg-base-100 text-primary",
                                svg {
                                    class: "size-7",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "1.7",
                                    path { d: "M12 4v16m8-8H4" }
                                    rect { x: "3", y: "3", width: "18", height: "18", rx: "4" }
                                }
                            }
                            div {
                                h3 { class: "card-title", "Bring your own API" }
                                p {
                                    class: "mt-2 leading-7 opacity-80",
                                    "If your system has an OpenAPI API, you can connect it to Bionic. Add your specification and make the operations available to your team."
                                }
                            }
                            a {
                                class: "btn btn-outline btn-sm mt-auto w-fit",
                                href: "{create_url}",
                                "Create an integration →"
                            }
                        }
                    }
                }

                MarketingSection { class: Some("site-marketing-section--compact".to_string()),
                    div {
                        h2 { "Integrations" }
                        p {
                            class: "mt-2 text-sm opacity-70",
                            "Browse the curated OpenAPI collection."
                        }
                    }
                    div {
                        class: "grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3",
                        for integration in integrations.iter() {
                            article {
                                class: "integration-card card border border-base-300 h-full",
                                div {
                                    class: "card-body gap-3 p-4",
                                    div {
                                        class: "flex min-w-0 items-center gap-3",
                                        {integration_logo(integration, true)}
                                        h3 {
                                            class: "min-w-0 truncate",
                                            "{integration.title}"
                                        }
                                    }
                                    p {
                                        class: "line-clamp-2 min-h-10 text-sm leading-5 opacity-75",
                                        "{integration.description}"
                                    }
                                    div {
                                        class: "pt-1",
                                        span { class: "badge badge-outline badge-sm", "OpenAPI" }
                                    }
                                }
                            }
                        }
                    }
                }

            }

            Footer {
                margin_top: "mt-0",
                links: footer_links()
            }
        }
    };

    crate::render(page)
}
