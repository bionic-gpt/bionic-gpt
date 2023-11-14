use db::Visibility;
use dioxus::prelude::VirtualDom;

pub mod api_keys;
pub mod app_layout;
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
pub mod training;

pub fn render(mut virtual_dom: VirtualDom) -> String {
    let _ = virtual_dom.rebuild();
    let html = dioxus_ssr::render(&virtual_dom);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

pub mod routes {

    pub mod console {
        pub static INDEX: &str = "/app/team/:organisation_id/console";
        pub static CONVERSATION: &str = "/app/team/:organisation_id/console/:conversation_id";
        pub static SEND_MESSAGE: &str = "/app/team/:organisation_id/send_message";
        pub static UPDATE_RESPONSE: &str = "/app/team/:organisation_id/update_response";
        pub static NEW_CHAT: &str = "/app/team/:organisation_id/new_chat";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/console", organisation_id)
        }

        pub fn conversation_route(organisation_id: i32, conversation_id: i64) -> String {
            format!("/app/team/{}/console/{}", organisation_id, conversation_id)
        }

        pub fn send_message_route(organisation_id: i32) -> String {
            format!("/app/team/{}/send_message", organisation_id)
        }

        pub fn update_response_route(organisation_id: i32) -> String {
            format!("/app/team/{}/update_response", organisation_id)
        }

        pub fn new_chat_route(organisation_id: i32) -> String {
            format!("/app/team/{}/new_chat", organisation_id)
        }
    }

    pub mod training {
        pub static INDEX: &str = "/app/team/:organisation_id/training";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/training", organisation_id)
        }
    }

    pub mod prompts {
        pub static INDEX: &str = "/app/team/:organisation_id/prompts";
        pub static NEW: &str = "/app/team/:organisation_id/prompts/new";
        pub static EDIT: &str = "/app/team/:organisation_id/prompts/:prompt_id/edit";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/prompts", organisation_id)
        }

        pub fn new_route(organisation_id: i32) -> String {
            format!("/app/team/{}/prompts/new", organisation_id)
        }

        pub fn edit_route(organisation_id: i32, prompt_id: i32) -> String {
            format!("/app/team/{}/prompts/{}/edit", organisation_id, prompt_id)
        }
    }

    pub mod models {
        pub static INDEX: &str = "/app/team/:organisation_id/models";
        pub static NEW: &str = "/app/team/:organisation_id/models/new";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/models", organisation_id)
        }

        pub fn new_route(organisation_id: i32) -> String {
            format!("/app/team/{}/models/new", organisation_id)
        }
    }

    pub mod datasets {
        pub static INDEX: &str = "/app/team/:organisation_id/datasets";
        pub static NEW: &str = "/app/team/:organisation_id/datasets/new";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/datasets", organisation_id)
        }

        pub fn new_route(organisation_id: i32) -> String {
            format!("/app/team/{}/datasets/new", organisation_id)
        }
    }

    pub mod documents {
        pub static INDEX: &str = "/app/team/:organisation_id/dataset/:dataset_id/documents";
        pub static BULK: &str = "/app/team/:organisation_id/bulk_import";
        pub static UPLOAD: &str = "/app/team/:organisation_id/dataset/:dataset_id/doc_upload";
        pub static DELETE: &str = "/app/team/:organisation_id/delete_doc/:document_id";
        pub static STATUS: &str = "/app/team/doc_status/:document_id";

        pub fn index_route(organisation_id: i32, dataset_id: i32) -> String {
            format!(
                "/app/team/{}/dataset/{}/documents",
                organisation_id, dataset_id
            )
        }

        pub fn bulk_route(organisation_id: i32) -> String {
            format!("/app/team/{}/bulk_import", organisation_id)
        }

        pub fn upload_route(organisation_id: i32, dataset_id: i32) -> String {
            format!(
                "/app/team/{}/dataset/{}/doc_upload",
                organisation_id, dataset_id
            )
        }

        pub fn delete_route(organisation_id: i32, document_id: i32) -> String {
            format!("/app/team/{}/delete_doc/{}", organisation_id, document_id)
        }

        pub fn status_route(document_id: i32) -> String {
            format!("/app/team/doc_status/{}", document_id)
        }
    }

    pub mod api_keys {
        pub static INDEX: &str = "/app/team/:organisation_id/api_keys";
        pub static NEW: &str = "/app/team/:organisation_id/api_keys/new";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/api_keys", organisation_id)
        }

        pub fn new_route(organisation_id: i32) -> String {
            format!("/app/team/{}/api_keys/new", organisation_id)
        }
    }

    pub mod team {
        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}", organisation_id)
        }

        pub fn switch_route(organisation_id: i32) -> String {
            format!("/app/team/{}/switch", organisation_id)
        }

        pub fn teams_popup_route(organisation_id: i32) -> String {
            format!("/app/team/{}/teams_popup", organisation_id)
        }

        pub fn create_route(organisation_id: i32) -> String {
            format!("/app/team/{}/create_invite", organisation_id)
        }

        pub fn delete_route(organisation_id: i32) -> String {
            format!("/app/team/{}/delete", organisation_id)
        }

        pub fn set_name_route(organisation_id: i32) -> String {
            format!("/app/team/{}/set_name", organisation_id)
        }

        pub fn new_team_route(organisation_id: i32) -> String {
            format!("/app/team/{}/new", organisation_id)
        }
    }

    pub mod profile {

        pub fn set_details_route(organisation_id: i32) -> String {
            format!("/app/team/{}/set_details", organisation_id)
        }

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/profile", organisation_id)
        }

        pub fn profile_popup_route(organisation_id: i32) -> String {
            format!("/app/team/{}/profile_popup", organisation_id)
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
