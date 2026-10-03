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
    tasks: [Task; 3],
    bottom: &'static str,
}

struct Task {
    text: &'static str,
    icon: TaskIcon,
}

enum TaskIcon {
    Search,
    Presentation,
    FileCheck,
    Scale,
    Messages,
    FileEdit,
    Calculator,
    Trending,
    Clipboard,
}

impl TaskIcon {
    fn path(&self) -> &'static str {
        match self {
            Self::Search => "M21 21l-4.4-4.4 M19 11a8 8 0 1 1-16 0 8 8 0 0 1 16 0",
            Self::Presentation => "M3 3h18 M4 3v13h16V3 M12 16v5 M8 21h8 M7 12l3-3 3 2 4-5",
            Self::FileCheck => "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z M14 2v6h6 M8 15l3 3 5-6",
            Self::Scale => "M12 3v18 M7 21h10 M5 7h14 M5 7l-3 7h6z M19 7l-3 7h6z",
            Self::Messages => "M5 15l-3 3V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2z M8 18v1a2 2 0 0 0 2 2h9l3 2V11a2 2 0 0 0-2-2",
            Self::FileEdit => "M12 3H6a2 2 0 0 0-2 2v15a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-6 M16 3l5 5 M10 14l-1 5 5-1 8-8a2 2 0 0 0-5-5z",
            Self::Calculator => "M6 2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2z M8 6h8 M8 11h1 M15 11h1 M8 15h1 M15 15h1 M8 19h1 M15 19h1",
            Self::Trending => "M3 3v18h18 M6 15l5-5 4 3 6-7 M16 6h5v5",
            Self::Clipboard => "M9 3H6a2 2 0 0 0-2 2v15a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2h-3 M9 2h6v4H9z M8 11h8 M8 15h8 M8 19h5",
        }
    }
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
            Task {
                text: "find your next 20 leads",
                icon: TaskIcon::Search,
            },
            Task {
                text: "prepare your board pack",
                icon: TaskIcon::Presentation,
            },
            Task {
                text: "review every open contract",
                icon: TaskIcon::FileCheck,
            },
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
            Task {
                text: "compare 30 supplier bids",
                icon: TaskIcon::Scale,
            },
            Task {
                text: "analyse customer feedback",
                icon: TaskIcon::Messages,
            },
            Task {
                text: "draft your tender response",
                icon: TaskIcon::FileEdit,
            },
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
            Task {
                text: "reconcile the monthly accounts",
                icon: TaskIcon::Calculator,
            },
            Task {
                text: "explain the revenue variance",
                icon: TaskIcon::Trending,
            },
            Task {
                text: "prepare tomorrow's report",
                icon: TaskIcon::Clipboard,
            },
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
                                    span {
                                        class: "use-case-hero__task-copy",
                                        svg {
                                            class: "use-case-hero__task-icon",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2.5",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            "aria-hidden": "true",
                                            "focusable": "false",
                                            path { d: task.icon.path() }
                                        }
                                        span { "{task.text}" }
                                    }
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
