#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::avatar_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[inline_props]
pub fn Page(cx: Scope, team_id: i32, rbac: Rbac) -> Element {
    cx.render(rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::None,
            title: "Your Profile",
            team_id: *team_id,
            rbac: rbac,
            header: cx.render(rsx!(
                h3 { "Your Profile" }
            )),
            BlankSlate {
                heading: "Your Installation",
                visual: avatar_svg.name,
                description: "Here you can see how you are progressing towards a Bionic-GPT full installation.",
            }

            Box {
                BoxHeader {
                    title: "Community Edition"
                }
                BoxBody {
                    Alert {
                        alert_color: AlertColor::Warn,
                        div {
                            p {
                                "The community edition of Bionic-GPT gives you a way to quickly
                                try out the software. However we do not recommend you deploy
                                this version to production."
                            }
                        }
                    }
                }
            }

            Box {
                BoxHeader {
                    title: "Enterprise Edition"
                }
                BoxBody {
                }
            }

            Box {
                BoxHeader {
                    title: "Enterprise Edition (Fully Licenced)"
                }
                BoxBody {
                }
            }
        }
    })
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
