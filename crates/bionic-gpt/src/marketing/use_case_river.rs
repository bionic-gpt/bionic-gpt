use dioxus::prelude::*;

#[component]
pub fn UseCaseRiver() -> Element {
    rsx! {
        section {
            class: "use-case-river",
            aria_labelledby: "use-case-river-title",
            section {
                class: "use-case-river__intro",
                div {
                    class: "use-case-river__eyebrow",
                    "Bionic at work"
                }
                h2 {
                    id: "use-case-river-title",
                    "Give Bionic work."
                    br {
                    }
                    "Not just questions."
                }
                p {
                    "From the first briefing of the day to work that keeps running after you leave, Bionic connects to the tools your business already uses and gets things done."
                }
            }
            section {
                class: "use-case-river__rows",
                article {
                    class: "use-case-river__row",
                    div {
                        class: "use-case-river__copy",
                        div {
                            class: "use-case-river__num",
                            "01"
                        }
                        h3 {
                            "Start every day one step ahead"
                        }
                        p {
                            "Wake up to a briefing built from your inbox, calendar and business systems — what changed, what matters and what needs your attention."
                        }
                        div {
                            class: "use-case-river__tools card",
                            span {
                                class: "use-case-river__tool-label",
                                "Works with"
                            }
                            IntegrationIcon { name: "Mail" }
                            IntegrationIcon { name: "Calendar" }
                            IntegrationIcon { name: "CRM" }
                            IntegrationIcon { name: "Docs" }
                        }
                    }
                    div {
                        class: "use-case-river__visual card",
                        div {
                            class: "use-case-river__window card bg-base-100",
                            div {
                                class: "use-case-river__conversation",
                                div {
                                    class: "chat chat-end use-case-river__prompt",
                                    div { class: "chat-bubble chat-bubble-primary",
                                        "Give me my morning briefing."
                                    }
                                }
                                div {
                                    class: "use-case-river__answer",
                                    div {
                                        class: "use-case-river__answer-body",
                                        p {
                                            class: "use-case-river__answer-copy",
                                            "Morning. Three things need your attention today."
                                        }
                                        div {
                                            class: "use-case-river__item",
                                            div {
                                                class: "use-case-river__icon",
                                                "aria-hidden": "true",
                                                "✉"
                                            }
                                            div {
                                                strong {
                                                    "3 emails need a response"
                                                }
                                                span {
                                                    "One customer issue and two sales opportunities."
                                                }
                                            }
                                        }
                                        div {
                                            class: "use-case-river__item",
                                            div {
                                                class: "use-case-river__icon",
                                                "aria-hidden": "true",
                                                "◷"
                                            }
                                            div {
                                                strong {
                                                    "Acme renewal · 09:30"
                                                }
                                                span {
                                                    "Sarah confirmed the revised budget yesterday."
                                                }
                                            }
                                        }
                                        div {
                                            class: "use-case-river__item",
                                            div {
                                                class: "use-case-river__icon",
                                                "aria-hidden": "true",
                                                "↗"
                                            }
                                            div {
                                                strong {
                                                    "Northstar moved to proposal"
                                                }
                                                span {
                                                    "The opportunity changed overnight."
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                article {
                    class: "use-case-river__row",
                    div {
                        class: "use-case-river__copy",
                        div {
                            class: "use-case-river__num",
                            "02"
                        }
                        h3 {
                            "Turn your inbox into actions"
                        }
                        p {
                            "Bionic can triage incoming email, separate routine messages from the ones that matter, prepare replies and take the next step in your systems."
                        }
                        div {
                            class: "use-case-river__tools card",
                            span {
                                class: "use-case-river__tool-label",
                                "Works with"
                            }
                            IntegrationIcon { name: "Mail" }
                            IntegrationIcon { name: "CRM" }
                            IntegrationIcon { name: "Docs" }
                        }
                    }
                    div {
                        class: "use-case-river__visual card",
                        div {
                            class: "use-case-river__window card bg-base-100",
                            div {
                                class: "use-case-river__conversation",
                                div {
                                    class: "chat chat-end use-case-river__prompt",
                                    div { class: "chat-bubble chat-bubble-primary",
                                        "Keep on top of my inbox. Only bother me when I need to do something."
                                    }
                                }
                                div {
                                    class: "use-case-river__answer",
                                    div {
                                        class: "use-case-river__answer-body",
                                        p {
                                            class: "use-case-river__answer-copy",
                                            "I checked 27 new messages. I handled the routine ones; these four need you."
                                        }
                                        div {
                                            class: "use-case-river__item",
                                            div {
                                                class: "use-case-river__icon",
                                                "aria-hidden": "true",
                                                "!"
                                            }
                                            div {
                                                strong {
                                                    "Customer needs a decision"
                                                }
                                                span {
                                                    "Contract question · I drafted a reply."
                                                }
                                            }
                                        }
                                        div {
                                            class: "use-case-river__item",
                                            div {
                                                class: "use-case-river__icon",
                                                "aria-hidden": "true",
                                                "★"
                                            }
                                            div {
                                                strong {
                                                    "New sales enquiry"
                                                }
                                                span {
                                                    "Looks qualified · I created the CRM record."
                                                }
                                            }
                                        }
                                        div {
                                            class: "use-case-river__item",
                                            div {
                                                class: "use-case-river__icon",
                                                "aria-hidden": "true",
                                                "✓"
                                            }
                                            div {
                                                strong {
                                                    "23 handled"
                                                }
                                                span {
                                                    "Receipts, notifications and routine requests."
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                article {
                    class: "use-case-river__row",
                    div {
                        class: "use-case-river__copy",
                        div {
                            class: "use-case-river__num",
                            "03"
                        }
                        h3 {
                            "Find your next customers"
                        }
                        p {
                            "Give Bionic a market or customer profile. It can research prospects, qualify them, enrich the useful ones and prepare personalized outreach."
                        }
                        div {
                            class: "use-case-river__tools card",
                            span {
                                class: "use-case-river__tool-label",
                                "Works with"
                            }
                            IntegrationIcon { name: "Web" }
                            IntegrationIcon { name: "CRM" }
                            IntegrationIcon { name: "Mail" }
                        }
                    }
                    div {
                        class: "use-case-river__visual card",
                        div {
                            class: "use-case-river__window card bg-base-100",
                            div {
                                class: "use-case-river__conversation",
                                div {
                                    class: "chat chat-end use-case-river__prompt",
                                    div { class: "chat-bubble chat-bubble-primary",
                                        "Find German manufacturers that look like good customers for us."
                                    }
                                }
                                div {
                                    class: "use-case-river__answer",
                                    div {
                                        class: "use-case-river__answer-body",
                                        p {
                                            class: "use-case-river__answer-copy",
                                            "I researched the market and found 12 strong matches. Here are the top three."
                                        }
                                        div {
                                            class: "use-case-river__leads",
                                            div {
                                                class: "use-case-river__lead card",
                                                div {
                                                    b {
                                                        "Nordwerk GmbH"
                                                    }
                                                    small {
                                                        "Relevant initiative found · decision maker identified"
                                                    }
                                                }
                                                span {
                                                    class: "use-case-river__score badge",
                                                    "92% fit"
                                                }
                                            }
                                            div {
                                                class: "use-case-river__lead card",
                                                div {
                                                    b {
                                                        "Rhein Systems AG"
                                                    }
                                                    small {
                                                        "1,400 employees · active expansion signal"
                                                    }
                                                }
                                                span {
                                                    class: "use-case-river__score badge",
                                                    "88% fit"
                                                }
                                            }
                                            div {
                                                class: "use-case-river__lead card",
                                                div {
                                                    b {
                                                        "Hansa Industrial"
                                                    }
                                                    small {
                                                        "Strong ICP match · outreach angle prepared"
                                                    }
                                                }
                                                span {
                                                    class: "use-case-river__score badge",
                                                    "84% fit"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                article {
                    class: "use-case-river__row",
                    div {
                        class: "use-case-river__copy",
                        div {
                            class: "use-case-river__num",
                            "04"
                        }
                        h3 {
                            "Keep working when you aren't"
                        }
                        p {
                            "Turn useful prompts into recurring work. Bionic can prepare reports, monitor changes and follow up automatically — without waiting for another chat."
                        }
                        div {
                            class: "use-case-river__tools card",
                            span {
                                class: "use-case-river__tool-label",
                                "Works with"
                            }
                            IntegrationIcon { name: "Clock" }
                            IntegrationIcon { name: "Mail" }
                            IntegrationIcon { name: "CRM" }
                            IntegrationIcon { name: "Docs" }
                        }
                    }
                    div {
                        class: "use-case-river__visual card",
                        div {
                            class: "use-case-river__window card bg-base-100",
                            div {
                                class: "use-case-river__conversation",
                                div {
                                    class: "chat chat-end use-case-river__prompt",
                                    div { class: "chat-bubble chat-bubble-primary",
                                        "Every Friday, prepare my sales report and flag anything I should worry about."
                                    }
                                }
                                div {
                                    class: "use-case-river__answer",
                                    div {
                                        class: "use-case-river__answer-body",
                                        p {
                                            class: "use-case-river__answer-copy",
                                            "Done. I’ll run this every Friday. This week’s report is ready now."
                                        }
                                        div {
                                            class: "use-case-river__status",
                                            span {
                                                class: "use-case-river__dot",
                                                "aria-hidden": "true",
                                            }
                                            div {
                                                strong {
                                                    "6 deals moved"
                                                }
                                                br {
                                                }
                                                span {
                                                    "Two progressed to proposal."
                                                }
                                            }
                                        }
                                        div {
                                            class: "use-case-river__status",
                                            span {
                                                class: "use-case-river__dot",
                                                "aria-hidden": "true",
                                            }
                                            div {
                                                strong {
                                                    "2 risks need attention"
                                                }
                                                br {
                                                }
                                                span {
                                                    "No activity for more than 14 days."
                                                }
                                            }
                                        }
                                        div {
                                            class: "use-case-river__status",
                                            span {
                                                class: "use-case-river__dot",
                                                "aria-hidden": "true",
                                            }
                                            div {
                                                strong {
                                                    "Report ready to share"
                                                }
                                                br {
                                                }
                                                span {
                                                    "I’ll do this again next Friday."
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn IntegrationIcon(name: &'static str) -> Element {
    let shape = match name {
        "Mail" => rsx! {
            rect {
                x: "3",
                y: "5",
                width: "18",
                height: "14",
                rx: "2",
            }
            path {
                d: "m4 7 8 6 8-6",
            }
        },
        "Calendar" => rsx! {
            rect {
                x: "3",
                y: "5",
                width: "18",
                height: "16",
                rx: "2",
            }
            path {
                d: "M7 3v4M17 3v4M3 10h18",
            }
        },
        "CRM" => rsx! {
            circle {
                cx: "9",
                cy: "8",
                r: "3",
            }
            path {
                d: "M3.5 19c.7-4 2.5-6 5.5-6s4.8 2 5.5 6M16 8h5M18.5 5.5v5",
            }
        },
        "Docs" => rsx! {
            path {
                d: "M6 3h8l4 4v14H6z",
            }
            path {
                d: "M14 3v5h5M9 13h6M9 17h6",
            }
        },
        "Web" => rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "9",
            }
            path {
                d: "M3 12h18M12 3c2.5 2.5 3.7 5.5 3.7 9S14.5 18.5 12 21M12 3C9.5 5.5 8.3 8.5 8.3 12S9.5 18.5 12 21",
            }
        },
        "Clock" => rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "9",
            }
            path {
                d: "M12 7v5l3 2",
            }
        },
        _ => rsx! {},
    };
    rsx! {
        span {
            class: "use-case-river__tool",
            title: name,
            role: "img",
            aria_label: name,
            svg {
                view_box: "0 0 24 24",
                "aria-hidden": "true",
                {shape}
            }
        }
    }
}
