use crate::integrations::{catalogue, Integration};
use crate::marketing::footer::Footer;
use crate::ui_links::footer_links;
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;
use ssg_whiz::Section;

fn integration_logo(integration: &Integration) -> Element {
    if integration.logo_url.is_empty() {
        rsx! {
            div {
                class: "grid size-11 shrink-0 place-items-center rounded-field border border-base-300 bg-base-100 text-primary",
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

            main {
                class: "mx-auto mt-16 grid w-full max-w-6xl gap-12 px-4 pb-16 md:mt-24 md:px-6",
                header {
                    class: "max-w-3xl",
                    p { class: "badge badge-outline", "Integrations" }
                    h1 {
                        class: "mt-5 text-4xl font-bold tracking-tight sm:text-5xl",
                        "Connect Bionic to the tools your business already uses"
                    }
                    p {
                        class: "mt-5 text-lg leading-8 opacity-80",
                        "Use an OpenAPI specification to give Bionic governed access to business systems and APIs."
                    }
                    p {
                        class: "mt-4 text-sm font-semibold opacity-70",
                        "{count} curated OpenAPI integrations"
                    }
                }

                section {
                    class: "grid gap-5 md:grid-cols-2",
                    article {
                        class: "card card-border",
                        div {
                            class: "card-body gap-5",
                            div {
                                class: "flex flex-wrap gap-2",
                                for integration in integrations.iter().take(5) {
                                    {integration_logo(integration)}
                                }
                            }
                            div {
                                h2 { class: "card-title", "Ready-to-use integrations" }
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
                                h2 { class: "card-title", "Bring your own API" }
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

                section {
                    id: "integration-catalogue",
                    class: "grid gap-6 scroll-mt-8",
                    div {
                        class: "grid gap-2",
                        div {
                            h2 { class: "text-2xl font-bold tracking-tight", "Integration catalogue" }
                            p { class: "mt-2 text-sm opacity-70", "Every integration is backed by an OpenAPI specification." }
                        }
                    }
                    div {
                        id: "integration-cards",
                        class: "grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3",
                        for integration in integrations.iter() {
                            a {
                                class: "integration-card card card-border h-full transition-colors hover:border-primary/50 hover:bg-base-200/40",
                                href: "/docs/integrations/specs/{integration.filename}",
                                aria_label: "{integration.title}: {integration.description}",
                                div {
                                    class: "card-body gap-3 p-4",
                                    div {
                                        class: "flex min-w-0 items-center gap-3",
                                        {integration_logo(integration)}
                                        h3 { class: "min-w-0 truncate font-semibold", "{integration.title}" }
                                    }
                                    p {
                                        class: "line-clamp-2 min-h-10 text-sm leading-5 opacity-75",
                                        "{integration.description}"
                                    }
                                    div {
                                        class: "flex items-center justify-between pt-1",
                                        span { class: "badge badge-outline badge-sm", "OpenAPI" }
                                        span { class: "text-primary", "→" }
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
