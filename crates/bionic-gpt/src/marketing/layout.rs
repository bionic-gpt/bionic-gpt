use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum PageWidth {
    Wide,
    Content,
}

#[derive(Clone, PartialEq)]
pub enum PageGutters {
    Standard,
    Roomy,
    Home,
}

#[derive(Clone, PartialEq)]
pub enum PageRhythm {
    Home,
    Product,
    Skills,
    Integrations,
    Legacy,
    Solution,
}

#[component]
pub fn PageContainer(
    width: PageWidth,
    gutters: Option<PageGutters>,
    rhythm: Option<PageRhythm>,
    class: Option<String>,
    children: Element,
) -> Element {
    let width_class = match width {
        PageWidth::Wide => "site-container-wide",
        PageWidth::Content => "site-container-content",
    };
    let gutters_class = match gutters {
        Some(PageGutters::Roomy) => "site-gutters-roomy",
        Some(PageGutters::Home) => "site-gutters-home",
        Some(PageGutters::Standard) | None => "site-gutters-standard",
    };
    let rhythm_class = match rhythm {
        Some(PageRhythm::Home) => "site-page-rhythm-home",
        Some(PageRhythm::Product) => "site-page-rhythm-product",
        Some(PageRhythm::Skills) => "site-page-rhythm-skills",
        Some(PageRhythm::Integrations) => "site-page-rhythm-integrations",
        Some(PageRhythm::Legacy) => "site-page-rhythm-legacy",
        Some(PageRhythm::Solution) => "site-page-rhythm-solution",
        None => "",
    };
    let class = class.unwrap_or_default();

    rsx! {
        main {
            class: "site-container grid {width_class} {gutters_class} {rhythm_class} {class}",
            {children}
        }
    }
}

#[component]
pub fn MarketingSection(class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        section {
            class: "site-marketing-section {class}",
            {children}
        }
    }
}

#[component]
pub fn SectionHeader(
    title: String,
    eyebrow: Option<String>,
    body: Option<String>,
    class: Option<String>,
) -> Element {
    let class = class.unwrap_or_default();
    let title_margin = if eyebrow.is_some() { "mt-5" } else { "" };
    rsx! {
        header {
            class: "site-section-header max-w-3xl {class}",
            if let Some(eyebrow) = eyebrow {
                p { class: "badge badge-outline", "{eyebrow}" }
            }
            h2 { class: "{title_margin}", "{title}" }
            if let Some(body) = body {
                p { class: "site-section-lead mt-4", "{body}" }
            }
        }
    }
}
