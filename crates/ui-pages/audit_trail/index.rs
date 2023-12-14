#![allow(non_snake_case)]
use daisy_rsx::*;
use db::{AuditTrail, Member};
use dioxus::prelude::*;

use crate::app_layout::{Layout, SideBar};

#[derive(Props, PartialEq)]
pub struct AuditProps {
    team_users: Vec<Member>,
    audits: Vec<AuditTrail>,
    team_id: i32,
    reset_search: bool,
}

#[inline_props]
pub fn Page(
    cx: Scope,
    team_users: Vec<Member>,
    audits: Vec<AuditTrail>,
    team_id: i32,
    is_sys_admin: bool,
    reset_search: bool,
) -> Element {
    cx.render(rsx! {

        Layout {
            section_class: "normal",
            selected_item: SideBar::AuditTrail,
            team_id: *team_id,
            is_sys_admin: *is_sys_admin,
            title: "Audit Trail",
            header: cx.render(rsx!(
                h3 { "Audit Trail" }
                Button {
                    drawer_trigger: super::filter::DRAW_TRIGGER,
                    button_scheme: ButtonScheme::Default,
                    "Filter"
                }
            ))
            super::table::AuditTable {
                audits: &audits
            }
            super::filter::FilterDrawer {
                team_users: team_users.clone(),
                reset_search: *reset_search,
                submit_action: crate::routes::audit_trail::index_route(*team_id)
            }
        }
    })
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
