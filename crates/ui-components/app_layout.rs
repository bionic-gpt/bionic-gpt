#![allow(non_snake_case)]
use super::logout_form::LogoutForm;
use assets::files::*;
use dioxus::prelude::*;
use primer_rsx::{AppLayout, NavGroup, NavItem};

#[derive(PartialEq, Eq, Debug)]
pub enum SideBar {
    None,
    ApiKeys,
    Console,
    Training,
    Prompts,
    Models,
    Datasets,
    BulkImport,
    Team,
    Profile,
    Switch,
}

impl std::fmt::Display for SideBar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Props)]
pub struct LayoutProps<'a> {
    selected_item: SideBar,
    title: &'a str,
    header: Element<'a>,
    children: Element<'a>,
    team_id: i32,
    section_class: &'a str,
}

pub fn Layout<'a>(cx: Scope<'a, LayoutProps<'a>>) -> Element {
    cx.render(rsx! {
        AppLayout {
            title: cx.props.title,
            css_href1: primer_view_components_css.name,
            css_href2: index_css.name,
            js_href: index_js.name,
            section_class: cx.props.section_class,
            fav_icon_src: favicon_svg.name,
            header: cx.render(rsx!(
                &cx.props.header
            )),
            sidebar: cx.render(rsx!(
                NavGroup {
                    heading: "AI Chat",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::Console.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::console::index_route(cx.props.team_id),
                            icon: nav_service_requests_svg.name,
                            title: "Chat Console"
                        }
                        NavItem {
                            id: SideBar::Prompts.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::prompts::index_route(cx.props.team_id),
                            icon: nav_dashboard_svg.name,
                            title: "Prompts"
                        }
                    ))
                }
                NavGroup {
                    heading: "Documents",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::Datasets.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::datasets::index_route(cx.props.team_id),
                            icon: nav_ccsds_data_svg.name,
                            title: "Team Datasets"
                        }
                        NavItem {
                            id: SideBar::BulkImport.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::documents::bulk_route(cx.props.team_id),
                            icon: nav_ccsds_data_svg.name,
                            title: "Document Pipeline"
                        }
                    ))
                }
                NavGroup {
                    heading: "Models & Fine Tuning",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::Models.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::models::index_route(cx.props.team_id),
                            icon: nav_phonebook_svg.name,
                            title: "Model Setup"
                        }
                        NavItem {
                            id: SideBar::Training.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::training::index_route(cx.props.team_id),
                            icon: nav_space_objects_svg.name,
                            title: "Training Runs"
                        }
                    ))
                }
                NavGroup {
                    heading: "Developers",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::ApiKeys.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::api_keys::index_route(cx.props.team_id),
                            icon: nav_api_keys_svg.name,
                            title: "API Keys"
                        }
                    ))
                }
                NavGroup {
                    heading: "Collaboration",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::Team.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::team::index_route(cx.props.team_id),
                            icon: nav_members_svg.name,
                            title: "Team Members"
                        }
                        NavItem {
                            id: SideBar::Switch.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::team::switch_route(cx.props.team_id),
                            icon: nav_teams_svg.name,
                            title: "Your Teams"
                        }
                    ))
                }
            )),
            sidebar_header: cx.render(rsx!(
                turbo-frame {
                    id: "teams-popup",
                    class: "width-full",
                    src: "{super::routes::team::teams_popup_route(cx.props.team_id)}"
                }
            )),
            sidebar_footer: cx.render(rsx!(
                turbo-frame {
                    id: "profile-popup",
                    class: "width-full",
                    src: "{super::routes::profile::profile_popup_route(cx.props.team_id)}"
                }
            )),
            &cx.props.children
            snack-bar {}
        }
        LogoutForm {}
    })
}
