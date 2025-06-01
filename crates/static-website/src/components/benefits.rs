use dioxus::prelude::*;

#[component]
pub fn Benefits(
    title: String,
    subtitle: String,
    benefit1: String,
    benefit1_desc: String,
    benefit2: String,
    benefit2_desc: String,
    benefit3: String,
    benefit3_desc: String,
    class: Option<String>,
) -> Element {
    rsx! {
        section {
            class: format!("lg:max-w-5xl {}", class.unwrap_or("".to_string())),
            div {
                class: "container mx-auto",
                div {
                    class: "flex flex-col text-center w-full mb-20",
                    h2 {
                        class: "text-primary tracking-widest font-medium title-font mb-1",
                        "{title}"
                    }
                    h1 {
                        class: "sm:text-3xl text-2xl font-medium title-font text-primary",
                        "{subtitle}"
                    }
                }
                div {
                    class: "flex flex-wrap -m-4",
                    div {
                        class: "p-4 md:w-1/3",
                        div {
                            class: "flex rounded-lg h-full bg-base-200 p-8 flex-col",
                            div {
                                class: "flex items-center mb-3",
                                div {
                                    class: "w-8 h-8 mr-3 inline-flex items-center justify-center rounded-full bg-indigo-500 text-white shrink-0",
                                    svg {
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        class: "w-5 h-5",
                                        view_box: "0 0 24 24",
                                        path { d: "M22 12h-4l-3 9L9 3l-3 9H2" }
                                    }
                                }
                                h2 {
                                    class: "text-lg title-font font-medium",
                                    "{benefit1}"
                                }
                            }
                            div {
                                class: "grow",
                                p {
                                    class: "leading-relaxed text-base",
                                    "{benefit1_desc}"
                                }
                            }
                        }
                    }
                    // Repeat for other sections with adjusted content
                    div {
                        class: "p-4 md:w-1/3",
                        div {
                            class: "flex rounded-lg h-full bg-base-200 p-8 flex-col",
                            div {
                                class: "flex items-center mb-3",
                                div {
                                    class: "w-8 h-8 mr-3 inline-flex items-center justify-center rounded-full bg-indigo-500 text-white shrink-0",
                                    svg {
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        class: "w-5 h-5",
                                        view_box: "0 0 24 24",
                                        path { d: "M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2" }
                                        circle { cx: "12", cy: "7", r: "4" }
                                    }
                                }
                                h2 {
                                    class: "text-lg title-font font-medium",
                                    "{benefit2}"
                                }
                            }
                            div {
                                class: "grow",
                                p {
                                    class: "leading-relaxed text-base",
                                    "{benefit2_desc}"
                                }
                            }
                        }
                    }
                    div {
                        class: "p-4 md:w-1/3",
                        div {
                            class: "flex rounded-lg h-full bg-base-200 p-8 flex-col",
                            div {
                                class: "flex items-center mb-3",
                                div {
                                    class: "w-8 h-8 mr-3 inline-flex items-center justify-center rounded-full bg-indigo-500 text-white shrink-0",
                                    svg {
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        class: "w-5 h-5",
                                        view_box: "0 0 24 24",
                                        circle { cx: "6", cy: "6", r: "3" }
                                        circle { cx: "6", cy: "18", r: "3" }
                                        path { d: "M20 4L8.12 15.88M14.47 14.48L20 20M8.12 8.12L12 12" }
                                    }
                                }
                                h2 {
                                    class: "text-lg title-font font-medium",
                                    "{benefit3}"
                                }
                            }
                            div {
                                class: "grow",
                                p {
                                    class: "leading-relaxed text-base",
                                    "{benefit3_desc}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
