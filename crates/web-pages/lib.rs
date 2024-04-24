use db::Visibility;
use dioxus::prelude::{ComponentFunction, VirtualDom};

pub mod api_keys;
pub mod app_layout;
pub mod audit_trail;
pub mod console;
pub mod datasets;
pub mod documents;
pub mod logout_form;
pub mod model_form;
pub mod models;
pub mod pipelines;
pub mod profile;
pub mod profile_popup;
pub mod prompts;
pub mod team_members;
pub mod teams;

// Generic function to render a component and its props to a string
pub fn render_with_props<P: Clone + 'static, M: 'static>(
    root: impl ComponentFunction<P, M>,
    root_props: P,
) -> String {
    let mut vdom = VirtualDom::new_with_props(root, root_props);
    vdom.rebuild_in_place();
    let html = dioxus_ssr::render(&vdom);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

// All the routes of the application are mapped here and are typesafe
// https://docs.rs/axum-extra/latest/axum_extra/routing/trait.TypedPath.html
pub mod routes {

    pub mod api_keys {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/api_keys")]
        pub struct Index {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/api_keys/new")]
        pub struct New {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/api_keys/delete/:id")]
        pub struct Delete {
            pub team_id: i32,
            pub id: i32,
        }
    }

    pub mod audit_trail {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/audit_trail")]
        pub struct Index {
            pub team_id: i32,
        }
    }

    pub mod document_pipelines {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/pipelines")]
        pub struct Index {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/pipelines/new")]
        pub struct New {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/pipelines/delete/:id")]
        pub struct Delete {
            pub team_id: i32,
            pub id: i32,
        }
    }

    pub mod console {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/console")]
        pub struct Index {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/console/:conversation_id")]
        pub struct Conversation {
            pub team_id: i32,
            pub conversation_id: i64,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/send_message")]
        pub struct SendMessage {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/update_response")]
        pub struct UpdateResponse {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/new_chat")]
        pub struct NewChat {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/console/delete/:id")]
        pub struct Delete {
            pub team_id: i32,
            pub id: i64,
        }
    }

    pub mod training {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/training")]
        pub struct Index {
            pub team_id: i32,
        }
    }

    pub mod prompts {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/prompts")]
        pub struct Index {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/prompts/upsert")]
        pub struct Upsert {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/prompts/delete/:id")]
        pub struct Delete {
            pub team_id: i32,
            pub id: i32,
        }
    }

    pub mod models {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/models")]
        pub struct Index {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/models/new")]
        pub struct New {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/models/delete/:id")]
        pub struct Delete {
            pub team_id: i32,
            pub id: i32,
        }
    }

    pub mod datasets {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/datasets")]
        pub struct Index {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/datasets/new")]
        pub struct New {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/datasets/delete/:id")]
        pub struct Delete {
            pub team_id: i32,
            pub id: i32,
        }
    }

    pub mod documents {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/dataset/:dataset_id/documents")]
        pub struct Index {
            pub team_id: i32,
            pub dataset_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/bulk_import")]
        pub struct Bulk {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/dataset/:dataset_id/doc_upload")]
        pub struct Upload {
            pub team_id: i32,
            pub dataset_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/processing/:document_id")]
        pub struct Processing {
            pub team_id: i32,
            pub document_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/delete_doc/:document_id")]
        pub struct Delete {
            pub team_id: i32,
            pub document_id: i32,
        }
    }

    pub mod team {
        pub fn index_route(team_id: i32) -> String {
            format!("/app/team/{}", team_id)
        }

        pub fn switch_route(team_id: i32) -> String {
            format!("/app/team/{}/switch", team_id)
        }

        pub fn teams_popup_route(team_id: i32) -> String {
            format!("/app/team/{}/teams_popup", team_id)
        }

        pub fn create_route(team_id: i32) -> String {
            format!("/app/team/{}/create_invite", team_id)
        }

        pub fn delete_route(team_id: i32) -> String {
            format!("/app/team/{}/delete", team_id)
        }

        pub fn set_name_route(team_id: i32) -> String {
            format!("/app/team/{}/set_name", team_id)
        }

        pub fn new_team_route(team_id: i32) -> String {
            format!("/app/team/{}/new", team_id)
        }
    }

    pub mod profile {

        pub fn set_details_route(team_id: i32) -> String {
            format!("/app/team/{}/set_details", team_id)
        }

        pub fn index_route(team_id: i32) -> String {
            format!("/app/team/{}/profile", team_id)
        }

        pub fn profile_popup_route(team_id: i32) -> String {
            format!("/app/team/{}/profile_popup", team_id)
        }
    }
}

pub fn visibility_to_string(visibility: Visibility) -> String {
    match visibility {
        Visibility::Private => "Private".to_string(),
        Visibility::Team => "Team".to_string(),
        Visibility::Company => "Company".to_string(),
    }
}

pub fn string_to_visibility(visibility: &str) -> Visibility {
    match visibility {
        "Team" => Visibility::Team,
        "Company" => Visibility::Company,
        _ => Visibility::Private,
    }
}
