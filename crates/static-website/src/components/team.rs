use dioxus::prelude::*;

#[component]
pub fn ContactCard(img: String, name: String, role: String) -> Element {
    rsx! {
        div {
            class: "p-2 lg:w-1/3 md:w-1/2 w-full",
            div {
                class: "h-full flex items-center border-gray-200 border p-4 rounded-lg",
                img {
                    alt: "team",
                    class: "w-16 h-16 bg-gray-100 object-cover object-center shrink-0 rounded-full mr-4",
                    src: "{img}",
                }
                div {
                    class: "grow",
                    h2 {
                        class: "font-medium",
                        "{name}"
                    }
                    p {
                        class: "text-gray-500",
                        "{role}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Team() -> Element {
    rsx! {
        section {
            class: "lg:max-w-5xl mx-auto",
            div {
                class: "container py-24 mx-auto",
                div {
                    class: "flex flex-col text-center w-full mb-20",
                    h1 {
                        class: "sm:text-3xl text-2xl font-medium mb-4",
                        "Our Team"
                    }
                    p {
                        class: "lg:w-2/3 mx-auto leading-relaxed",
                        img {
                            src: "/contact-us/ian-and-dio.jpeg"
                        }
                    }
                }
                div {
                    class: "flex flex-wrap -m-2",ContactCard {
                        name: "Dio".to_string(),
                        role: "CEO".to_string(),
                        img: "/contact-us/dio.png".to_string()
                    }
                    ContactCard {
                        name: "Ian".to_string(),
                        role: "CTO".to_string(),
                        img: "/contact-us/ian.png".to_string()
                    }
                    ContactCard {
                        name: "Affifa R".to_string(),
                        role: "UI/UX Designer".to_string(),
                        img: "/contact-us/affifa-r.png".to_string()
                    }
                    ContactCard {
                        name: "John D".to_string(),
                        role: "Head of Engineering".to_string(),
                        img: "/contact-us/john-d.png".to_string()
                    }
                    ContactCard {
                        name: "Ashar P".to_string(),
                        role: "Growth Engineer".to_string(),
                        img: "/contact-us/ashar-p.jpeg".to_string()
                    }
                    ContactCard {
                        name: "Diana D".to_string(),
                        role: "AI Researcher".to_string(),
                        img: "/contact-us/diane-d.jpeg".to_string()
                    }
                    ContactCard {
                        name: "Nattaliia T".to_string(),
                        role: "QA Engineer".to_string(),
                        img: "/contact-us/nattaliia-t.jpeg".to_string()
                    }
                    ContactCard {
                        name: "Anastasia P".to_string(),
                        role: "Sales & Marketing".to_string(),
                        img: "/contact-us/anastasia-p.jpeg".to_string()
                    }
                    ContactCard {
                        name: "Martin M".to_string(),
                        role: "Product Manager".to_string(),
                        img: "/contact-us/martin-m.jpeg".to_string()
                    }

                }
            }
        }
    }
}
