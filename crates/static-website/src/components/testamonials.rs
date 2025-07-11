use dioxus::prelude::*;

#[component]
pub fn Testamonial(text: String, job: String, person: String, img: String) -> Element {
    rsx! {
        div {
            class: "h-full bg-base-200 p-8 rounded-sm",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                fill: "currentColor",
                class: "block w-5 h-5 text-gray-400 mb-4",
                view_box: "0 0 975.036 975.036",
                path {
                    d: "M925.036 57.197h-304c-27.6 0-50 22.4-50 50v304c0 27.601 22.4 50 50 50h145.5c-1.9 79.601-20.4 143.3-55.4 191.2-27.6 37.8-69.399 69.1-125.3 93.8-25.7 11.3-36.8 41.7-24.8 67.101l36 76c11.6 24.399 40.3 35.1 65.1 24.399 66.2-28.6 122.101-64.8 167.7-108.8 55.601-53.7 93.7-114.3 114.3-181.9 20.601-67.6 30.9-159.8 30.9-276.8v-239c0-27.599-22.401-50-50-50zM106.036 913.497c65.4-28.5 121-64.699 166.9-108.6 56.1-53.7 94.4-114.1 115-181.2 20.6-67.1 30.899-159.6 30.899-277.5v-239c0-27.6-22.399-50-50-50h-304c-27.6 0-50 22.4-50 50v304c0 27.601 22.4 50 50 50h145.5c-1.9 79.601-20.4 143.3-55.4 191.2-27.6 37.8-69.4 69.1-125.3 93.8-25.7 11.3-36.8 41.7-24.8 67.101l35.9 75.8c11.601 24.399 40.501 35.2 65.301 24.399z"
                }
            }
            p {
                class: "leading-relaxed mb-6",
                "{text}"
            }
            a {
                class: "inline-flex items-center",
                img {
                    alt: "testimonial",
                    src: img,
                    class: "w-12 h-12 rounded-full shrink-0 object-cover object-center",
                }
                span {
                    class: "grow flex flex-col pl-4",
                    span {
                        class: "title-font font-medium text-gray-900",
                        "{person}"
                    }
                    span {
                        class: "text-gray-500 text-sm",
                        "{job}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Testamonials(
    text1: String,
    job1: String,
    person1: String,
    img1: String,
    text2: String,
    job2: String,
    person2: String,
    img2: String,
    class: Option<String>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        section {
            class: format!("mx-auto lg:max-w-5xl {class}"),
            div {
                class: "container mx-auto",
                h1 {
                    class: "text-3xl font-medium font-display title-font mb-12 text-center",
                    "Testimonials"
                }
                div {
                    class: "flex flex-col md:flex-row gap-8",
                    div {
                        Testamonial {
                            person: person1,
                            text: text1,
                            job: job1,
                            img: img1
                        }
                    }
                    div {
                        Testamonial {
                            person: person2,
                            text: text2,
                            job: job2,
                            img: img2
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Testamonial1() -> Element {
    rsx! {
        Testamonials {
            text1: "Having the flexibility to use the best model for the job has been a game-changer. Bionic's support for multiple models ensures we can tailor solutions to specific challenges, delivering optimal results every time.",
            job1: "Data Scientist",
            person1: "Emmat",
            img1: "/partners/emmat.png",
            text2: "Bionic's observability feature, which logs all messages into and out of the models, has been critical for ensuring compliance in our organization. It gives us peace of mind and robust accountability.",
            job2: "Compliance Officer",
            person2: "Patrick",
            img2: "/partners/patrick.png"
        }
    }
}

#[component]
pub fn Testamonial2() -> Element {
    rsx! {
        Testamonials {
            text1: "The no-code RAG pipeline, combined with the team-based privacy model, has revolutionized
                how we handle sensitive data and collaboration, making deployment secure and seamless.",
            job1: "CEO GTEdge.ai",
            img1: "/partners/tom-bendien.png",
            person1: "Tom",
            text2: "The new, clean, and intuitive interface has made adopting Bionic across teams effortless.
                It’s a joy to use and has lowered the learning curve significantly. We're excited for the new possbilities.",
            job2: "Digital Adoption Specialist",
            person2: "Aisha",
            img2: "/partners/aisha.png"
        }
    }
}
