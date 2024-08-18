use db::Visibility;
use dioxus::prelude::{ComponentFunction, VirtualDom};

pub mod api_keys;
pub mod app_layout;
pub mod audit_trail;
pub mod console;
pub mod datasets;
pub mod documents;
pub mod guardrails;
pub mod history;
pub mod licence;
pub mod logout_form;
pub mod model_form;
pub mod models;
pub mod pipelines;
pub mod profile;
pub mod profile_popup;
pub mod prompts;
pub mod rate_limits;
pub mod team;
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

    pub mod history {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/history")]
        pub struct Index {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/search")]
        pub struct Search {
            pub team_id: i32,
        }
    }

    pub mod licence {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/licence")]
        pub struct Index {
            pub team_id: i32,
        }
    }

    pub mod guardrails {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/guardrails")]
        pub struct Index {
            pub team_id: i32,
        }
    }

    pub mod rate_limits {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/rate_limits")]
        pub struct Index {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/rate_limits/upsert")]
        pub struct Upsert {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/rate_limits/delete/:id")]
        pub struct Delete {
            pub team_id: i32,
            pub id: i32,
        }
    }

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

    pub mod prompts {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/assistant/:prompt_id/console/:conversation_id")]
        pub struct Conversation {
            pub team_id: i32,
            pub prompt_id: i32,
            pub conversation_id: i64,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/new_chat/:prompt_id")]
        pub struct NewChat {
            pub team_id: i32,
            pub prompt_id: i32,
        }

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

    pub mod teams {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/switch")]
        pub struct Switch {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/new")]
        pub struct New {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/accept_invite")]
        pub struct AcceptInvite {}
    }

    pub mod team {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id")]
        pub struct Index {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/teams_popup")]
        pub struct Popup {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/create_invite")]
        pub struct CreateInvite {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/invite/:invite_selector/:invite_validator")]
        pub struct AcceptInvite {
            pub invite_selector: String,
            pub invite_validator: String,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/delete")]
        pub struct Delete {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/set_name")]
        pub struct SetName {
            pub team_id: i32,
        }
    }

    pub mod profile {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/set_details")]
        pub struct SetDetails {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/profile")]
        pub struct Profile {
            pub team_id: i32,
        }

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/app/team/:team_id/profile_popup")]
        pub struct ProfilePopup {
            pub team_id: i32,
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
