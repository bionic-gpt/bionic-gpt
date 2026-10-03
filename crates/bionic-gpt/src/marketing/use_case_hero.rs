use dioxus::prelude::*;

// Keep media independent of tasks so scenes can gain video without changing sequencing.
struct ImageMedia {
    name: &'static str,
    height: u32,
    focus: &'static str,
    mobile_focus: &'static str,
}

impl ImageMedia {
    fn srcset(&self, format: &str) -> String {
        [768, 1280, 1920]
            .map(|width| format!("/use-case-hero/{}-{width}.{format} {width}w", self.name))
            .join(", ")
    }
}

struct Scene {
    media: ImageMedia,
    top: &'static str,
    tasks: [&'static str; 3],
    bottom: &'static str,
}

const SCENES: [Scene; 3] = [
    Scene {
        media: ImageMedia {
            name: "office",
            height: 1280,
            focus: "50% 45%",
            mobile_focus: "52% 45%",
        },
        top: "Bionic can",
        tasks: [
            "find your next 20 leads",
            "prepare your board pack",
            "review every open contract",
        ],
        bottom: "while you work",
    },
    Scene {
        media: ImageMedia {
            name: "paintball",
            height: 1323,
            focus: "50% 45%",
            mobile_focus: "40% 45%",
        },
        top: "Bionic can",
        tasks: [
            "compare 30 supplier bids",
            "analyse customer feedback",
            "draft your tender response",
        ],
        bottom: "while you play",
    },
    Scene {
        media: ImageMedia {
            name: "evening",
            height: 1280,
            focus: "50% 50%",
            mobile_focus: "60% 50%",
        },
        top: "Bionic can",
        tasks: [
            "reconcile the monthly accounts",
            "explain the revenue variance",
            "prepare tomorrow's report",
        ],
        bottom: "while you unwind",
    },
];

#[component]
pub fn UseCaseHero() -> Element {
    rsx! {
        section {
            class: "use-case-hero",
            "aria-labelledby": "use-case-hero-title",
            "data-use-case-hero": "",
            for (index, scene) in SCENES.iter().enumerate() {
                picture {
                    class: if index == 0 { "use-case-hero__scene is-active" } else { "use-case-hero__scene" },
                    "data-scene": "{scene.media.name}",
                    "data-top": "{scene.top}",
                    "data-bottom": "{scene.bottom}",
                    style: "--hero-focus: {scene.media.focus}; --hero-mobile-focus: {scene.media.mobile_focus}",
                    "aria-hidden": "true",
                    source {
                        r#type: "image/avif",
                        "sizes": "100vw",
                        "srcset": (index == 0).then(|| scene.media.srcset("avif")),
                        "data-srcset": (index != 0).then(|| scene.media.srcset("avif")),
                    }
                    source {
                        r#type: "image/webp",
                        "sizes": "100vw",
                        "srcset": (index == 0).then(|| scene.media.srcset("webp")),
                        "data-srcset": (index != 0).then(|| scene.media.srcset("webp")),
                    }
                    img {
                        alt: "",
                        width: 1920,
                        height: scene.media.height,
                        sizes: "100vw",
                        decoding: "async",
                        loading: "eager",
                        "fetchpriority": if index == 0 { "high" } else { "low" },
                        src: (index == 0).then(|| format!("/use-case-hero/{}-1280.webp", scene.media.name)),
                        srcset: (index == 0).then(|| scene.media.srcset("webp")),
                        "data-src": (index != 0).then(|| format!("/use-case-hero/{}-1280.webp", scene.media.name)),
                        "data-srcset": (index != 0).then(|| scene.media.srcset("webp")),
                    }
                }
            }
            div {
                class: "use-case-hero__content",
                h1 {
                    id: "use-case-hero-title",
                    class: "use-case-hero__sentence",
                    span { class: "use-case-hero__top", "{SCENES[0].top}" }
                    " "
                    span {
                        class: "use-case-hero__tasks",
                        for (scene_index, scene) in SCENES.iter().enumerate() {
                            for (task_index, task) in scene.tasks.iter().enumerate() {
                                span {
                                    class: if scene_index == 0 && task_index == 0 { "use-case-hero__task is-active" } else { "use-case-hero__task" },
                                    "data-scene": "{scene.media.name}",
                                    "aria-hidden": if scene_index == 0 && task_index == 0 { "false" } else { "true" },
                                    "{task}"
                                }
                            }
                        }
                    }
                    " "
                    span { class: "use-case-hero__bottom", "{SCENES[0].bottom}" }
                }
            }
            button {
                class: "use-case-hero__pause",
                r#type: "button",
                hidden: true,
                "aria-label": "Pause hero rotation",
                "aria-pressed": "false",
                "Pause"
            }
        }
        script { r#type: "module", src: "/use-case-hero.js" }
    }
}
