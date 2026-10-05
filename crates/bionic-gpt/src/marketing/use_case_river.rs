use dioxus::prelude::*;

#[component]
pub fn UseCaseRiver() -> Element {
    rsx! {
        section {
            class: "use-case-river text-base-content",
            aria_labelledby: "use-case-river-title",
            section {
                class: "use-case-river__intro mb-16 max-w-3xl md:mb-20",
                div {
                    class: "text-primary text-sm font-bold uppercase tracking-[0.1em]",
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
                class: "grid gap-20 md:gap-28",
                article {
                    class: "use-case-river__row grid items-start gap-8 md:grid-cols-[minmax(0,1fr)_minmax(0,1.08fr)] md:gap-12 lg:gap-20",
                    div {
                        class: "use-case-river__copy min-w-0 pt-2",
                        div {
                            class: "mb-5 text-sm font-bold text-primary",
                            "01"
                        }
                        h3 {
                            "Start every day one step ahead"
                        }
                        p {
                            "Wake up to a briefing built from your inbox, calendar and business systems — what changed, what matters and what needs your attention."
                        }
                        div {
                            class: "use-case-river__tools card mt-7 w-fit max-w-full flex-row flex-wrap items-center gap-2.5 border border-base-300 bg-base-100 py-3 pr-12 pl-3 shadow-sm",
                            span {
                                class: "mr-0.5 text-sm opacity-60",
                                "Works with"
                            }
                            IntegrationIcon { name: "Mail" }
                            IntegrationIcon { name: "Calendar" }
                            IntegrationIcon { name: "CRM" }
                            IntegrationIcon { name: "Docs" }
                        }
                    }
                    div {
                        class: "use-case-river__visual card min-w-0 min-h-[410px] items-center justify-center rounded-[calc(var(--radius-box)*1.75)] border border-base-300 bg-base-200 p-5 md:p-6",
                        div {
                            class: "card w-full overflow-hidden rounded-box border border-base-300 bg-base-100 p-4 shadow-md sm:p-6",
                            div {
                                class: "grid gap-5",
                                div {
                                    class: "chat chat-end p-0",
                                    div { class: "chat-bubble chat-bubble-primary",
                                        "Give me my morning briefing."
                                    }
                                }
                                div {
                                    class: "block",
                                    div {
                                        class: "min-w-0",
                                        p {
                                            class: "mb-3.5 text-sm leading-relaxed opacity-75 sm:text-base",
                                            "Morning. Three things need your attention today."
                                        }
                                        div {
                                            class: "use-case-river__item flex items-start gap-3 border-t border-base-300 py-3.5",
                                            div {
                                                class: "grid size-9 shrink-0 place-items-center rounded-field bg-primary/10 text-primary",
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
                                            class: "use-case-river__item flex items-start gap-3 border-t border-base-300 py-3.5",
                                            div {
                                                class: "grid size-9 shrink-0 place-items-center rounded-field bg-primary/10 text-primary",
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
                                            class: "use-case-river__item flex items-start gap-3 border-t border-base-300 py-3.5",
                                            div {
                                                class: "grid size-9 shrink-0 place-items-center rounded-field bg-primary/10 text-primary",
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
                    class: "use-case-river__row grid items-start gap-8 md:grid-cols-[minmax(0,1fr)_minmax(0,1.08fr)] md:gap-12 lg:gap-20",
                    div {
                        class: "use-case-river__copy min-w-0 pt-2",
                        div {
                            class: "mb-5 text-sm font-bold text-primary",
                            "02"
                        }
                        h3 {
                            "Turn your inbox into actions"
                        }
                        p {
                            "Bionic can triage incoming email, separate routine messages from the ones that matter, prepare replies and take the next step in your systems."
                        }
                        div {
                            class: "use-case-river__tools card mt-7 w-fit max-w-full flex-row flex-wrap items-center gap-2.5 border border-base-300 bg-base-100 py-3 pr-12 pl-3 shadow-sm",
                            span {
                                class: "mr-0.5 text-sm opacity-60",
                                "Works with"
                            }
                            IntegrationIcon { name: "Mail" }
                            IntegrationIcon { name: "CRM" }
                            IntegrationIcon { name: "Docs" }
                        }
                    }
                    div {
                        class: "use-case-river__visual card min-w-0 min-h-[410px] items-center justify-center rounded-[calc(var(--radius-box)*1.75)] border border-base-300 bg-base-200 p-5 md:p-6",
                        div {
                            class: "card w-full overflow-hidden rounded-box border border-base-300 bg-base-100 p-4 shadow-md sm:p-6",
                            div {
                                class: "grid gap-5",
                                div {
                                    class: "chat chat-end p-0",
                                    div { class: "chat-bubble chat-bubble-primary",
                                        "Keep on top of my inbox. Only bother me when I need to do something."
                                    }
                                }
                                div {
                                    class: "block",
                                    div {
                                        class: "min-w-0",
                                        p {
                                            class: "mb-3.5 text-sm leading-relaxed opacity-75 sm:text-base",
                                            "I checked 27 new messages. I handled the routine ones; these four need you."
                                        }
                                        div {
                                            class: "use-case-river__item flex items-start gap-3 border-t border-base-300 py-3.5",
                                            div {
                                                class: "grid size-9 shrink-0 place-items-center rounded-field bg-primary/10 text-primary",
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
                                            class: "use-case-river__item flex items-start gap-3 border-t border-base-300 py-3.5",
                                            div {
                                                class: "grid size-9 shrink-0 place-items-center rounded-field bg-primary/10 text-primary",
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
                                            class: "use-case-river__item flex items-start gap-3 border-t border-base-300 py-3.5",
                                            div {
                                                class: "grid size-9 shrink-0 place-items-center rounded-field bg-primary/10 text-primary",
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
                    class: "use-case-river__row grid items-start gap-8 md:grid-cols-[minmax(0,1fr)_minmax(0,1.08fr)] md:gap-12 lg:gap-20",
                    div {
                        class: "use-case-river__copy min-w-0 pt-2",
                        div {
                            class: "mb-5 text-sm font-bold text-primary",
                            "03"
                        }
                        h3 {
                            "Find your next customers"
                        }
                        p {
                            "Give Bionic a market or customer profile. It can research prospects, qualify them, enrich the useful ones and prepare personalized outreach."
                        }
                        div {
                            class: "use-case-river__tools card mt-7 w-fit max-w-full flex-row flex-wrap items-center gap-2.5 border border-base-300 bg-base-100 py-3 pr-12 pl-3 shadow-sm",
                            span {
                                class: "mr-0.5 text-sm opacity-60",
                                "Works with"
                            }
                            IntegrationIcon { name: "Web" }
                            IntegrationIcon { name: "CRM" }
                            IntegrationIcon { name: "Mail" }
                        }
                    }
                    div {
                        class: "use-case-river__visual card min-w-0 min-h-[410px] items-center justify-center rounded-[calc(var(--radius-box)*1.75)] border border-base-300 bg-base-200 p-5 md:p-6",
                        div {
                            class: "card w-full overflow-hidden rounded-box border border-base-300 bg-base-100 p-4 shadow-md sm:p-6",
                            div {
                                class: "grid gap-5",
                                div {
                                    class: "chat chat-end p-0",
                                    div { class: "chat-bubble chat-bubble-primary",
                                        "Find German manufacturers that look like good customers for us."
                                    }
                                }
                                div {
                                    class: "block",
                                    div {
                                        class: "min-w-0",
                                        p {
                                            class: "mb-3.5 text-sm leading-relaxed opacity-75 sm:text-base",
                                            "I researched the market and found 12 strong matches. Here are the top three."
                                        }
                                        div {
                                            class: "grid gap-2.5",
                                            div {
                                                class: "use-case-river__lead card grid min-w-0 grid-cols-[minmax(0,1fr)_auto] gap-3 rounded-field border border-base-300 p-3.5 max-[400px]:grid-cols-1",
                                                div {
                                                    b {
                                                        "Nordwerk GmbH"
                                                    }
                                                    small {
                                                        "Relevant initiative found · decision maker identified"
                                                    }
                                                }
                                                span {
                                                    class: "badge h-fit border-0 bg-primary/10 text-xs font-bold whitespace-nowrap text-primary",
                                                    "92% fit"
                                                }
                                            }
                                            div {
                                                class: "use-case-river__lead card grid min-w-0 grid-cols-[minmax(0,1fr)_auto] gap-3 rounded-field border border-base-300 p-3.5 max-[400px]:grid-cols-1",
                                                div {
                                                    b {
                                                        "Rhein Systems AG"
                                                    }
                                                    small {
                                                        "1,400 employees · active expansion signal"
                                                    }
                                                }
                                                span {
                                                    class: "badge h-fit border-0 bg-primary/10 text-xs font-bold whitespace-nowrap text-primary",
                                                    "88% fit"
                                                }
                                            }
                                            div {
                                                class: "use-case-river__lead card grid min-w-0 grid-cols-[minmax(0,1fr)_auto] gap-3 rounded-field border border-base-300 p-3.5 max-[400px]:grid-cols-1",
                                                div {
                                                    b {
                                                        "Hansa Industrial"
                                                    }
                                                    small {
                                                        "Strong ICP match · outreach angle prepared"
                                                    }
                                                }
                                                span {
                                                    class: "badge h-fit border-0 bg-primary/10 text-xs font-bold whitespace-nowrap text-primary",
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
                    class: "use-case-river__row grid items-start gap-8 md:grid-cols-[minmax(0,1fr)_minmax(0,1.08fr)] md:gap-12 lg:gap-20",
                    div {
                        class: "use-case-river__copy min-w-0 pt-2",
                        div {
                            class: "mb-5 text-sm font-bold text-primary",
                            "04"
                        }
                        h3 {
                            "Keep working when you aren't"
                        }
                        p {
                            "Turn useful prompts into recurring work. Bionic can prepare reports, monitor changes and follow up automatically — without waiting for another chat."
                        }
                        div {
                            class: "use-case-river__tools card mt-7 w-fit max-w-full flex-row flex-wrap items-center gap-2.5 border border-base-300 bg-base-100 py-3 pr-12 pl-3 shadow-sm",
                            span {
                                class: "mr-0.5 text-sm opacity-60",
                                "Works with"
                            }
                            IntegrationIcon { name: "Clock" }
                            IntegrationIcon { name: "Mail" }
                            IntegrationIcon { name: "CRM" }
                            IntegrationIcon { name: "Docs" }
                        }
                    }
                    div {
                        class: "use-case-river__visual card min-w-0 min-h-[410px] items-center justify-center rounded-[calc(var(--radius-box)*1.75)] border border-base-300 bg-base-200 p-5 md:p-6",
                        div {
                            class: "card w-full overflow-hidden rounded-box border border-base-300 bg-base-100 p-4 shadow-md sm:p-6",
                            div {
                                class: "grid gap-5",
                                div {
                                    class: "chat chat-end p-0",
                                    div { class: "chat-bubble chat-bubble-primary",
                                        "Every Friday, prepare my sales report and flag anything I should worry about."
                                    }
                                }
                                div {
                                    class: "block",
                                    div {
                                        class: "min-w-0",
                                        p {
                                            class: "mb-3.5 text-sm leading-relaxed opacity-75 sm:text-base",
                                            "Done. I’ll run this every Friday. This week’s report is ready now."
                                        }
                                        div {
                                            class: "use-case-river__status flex items-center gap-2.5 border-t border-base-300 py-3 text-sm",
                                            span {
                                                class: "size-2 shrink-0 rounded-full bg-primary",
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
                                            class: "use-case-river__status flex items-center gap-2.5 border-t border-base-300 py-3 text-sm",
                                            span {
                                                class: "size-2 shrink-0 rounded-full bg-primary",
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
                                            class: "use-case-river__status flex items-center gap-2.5 border-t border-base-300 py-3 text-sm",
                                            span {
                                                class: "size-2 shrink-0 rounded-full bg-primary",
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
            class: "use-case-river__tool grid size-11 place-items-center rounded-field border border-base-300 bg-base-100 max-[400px]:size-9",
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
