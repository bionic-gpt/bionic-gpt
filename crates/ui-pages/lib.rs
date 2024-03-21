use db::Visibility;
use dioxus::prelude::VirtualDom;

pub mod api_keys;
pub mod app_layout;
pub mod audit_trail;
pub mod console;
pub mod datasets;
pub mod documents;
pub mod enterprise;
pub mod logout_form;
pub mod model_form;
pub mod models;
pub mod pipelines;
pub mod profile;
pub mod profile_popup;
pub mod prompts;
pub mod team_members;
pub mod teams;

pub fn render(mut virtual_dom: VirtualDom) -> String {
    let _ = virtual_dom.rebuild();
    let html = dioxus_ssr::render(&virtual_dom);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

pub mod routes {

    pub mod audit_trail {
        pub static INDEX: &str = "/app/team/:team_id/audit_trail";

        pub fn index_route(team_id: i32) -> String {
            format!("/app/team/{}/audit_trail", team_id)
        }
    }

    pub mod document_pipelines {
        pub static INDEX: &str = "/app/team/:team_id/pipelines";
        pub static NEW: &str = "/app/team/:team_id/pipelines/new";
        pub static DELETE: &str = "/app/team/:team_id/pipelines/delete/:id";

        pub fn index_route(team_id: i32) -> String {
            format!("/app/team/{}/pipelines", team_id)
        }

        pub fn new_route(team_id: i32) -> String {
            format!("/app/team/{}/pipelines/new", team_id)
        }

        pub fn delete_route(team_id: i32, id: i32) -> String {
            format!("/app/team/{}/pipelines/delete/{}", team_id, id)
        }
    }

    pub mod console {
        pub static INDEX: &str = "/app/team/:team_id/console";
        pub static CONVERSATION: &str = "/app/team/:team_id/console/:conversation_id";
        pub static SEND_MESSAGE: &str = "/app/team/:team_id/send_message";
        pub static UPDATE_RESPONSE: &str = "/app/team/:team_id/update_response";
        pub static NEW_CHAT: &str = "/app/team/:team_id/new_chat";
        pub static DELETE: &str = "/app/team/:team_id/console/delete/:id";

        pub fn index_route(team_id: i32) -> String {
            format!("/app/team/{}/console", team_id)
        }

        pub fn conversation_route(team_id: i32, conversation_id: i64) -> String {
            format!("/app/team/{}/console/{}", team_id, conversation_id)
        }

        pub fn send_message_route(team_id: i32) -> String {
            format!("/app/team/{}/send_message", team_id)
        }

        pub fn update_response_route(team_id: i32) -> String {
            format!("/app/team/{}/update_response", team_id)
        }

        pub fn new_chat_route(team_id: i32) -> String {
            format!("/app/team/{}/new_chat", team_id)
        }

        pub fn delete_route(team_id: i32, id: i64) -> String {
            format!("/app/team/{}/console/delete/{}", team_id, id)
        }
    }

    pub mod training {
        pub static INDEX: &str = "/app/team/:team_id/training";

        pub fn index_route(team_id: i32) -> String {
            format!("/app/team/{}/training", team_id)
        }
    }

    pub mod prompts {
        pub static INDEX: &str = "/app/team/:team_id/prompts";
        pub static NEW: &str = "/app/team/:team_id/prompts/new";
        pub static EDIT: &str = "/app/team/:team_id/prompts/:prompt_id/edit";
        pub static DELETE: &str = "/app/team/:team_id/prompts/delete/:id";

        pub fn index_route(team_id: i32) -> String {
            format!("/app/team/{}/prompts", team_id)
        }

        pub fn new_route(team_id: i32) -> String {
            format!("/app/team/{}/prompts/new", team_id)
        }

        pub fn edit_route(team_id: i32, prompt_id: i32) -> String {
            format!("/app/team/{}/prompts/{}/edit", team_id, prompt_id)
        }

        pub fn delete_route(team_id: i32, id: i32) -> String {
            format!("/app/team/{}/prompts/delete/{}", team_id, id)
        }
    }

    pub mod models {
        pub static INDEX: &str = "/app/team/:team_id/models";
        pub static NEW: &str = "/app/team/:team_id/models/new";
        pub static DELETE: &str = "/app/team/:team_id/models/delete/:id";

        pub fn index_route(team_id: i32) -> String {
            format!("/app/team/{}/models", team_id)
        }

        pub fn new_route(team_id: i32) -> String {
            format!("/app/team/{}/models/new", team_id)
        }

        pub fn delete_route(team_id: i32, id: i32) -> String {
            format!("/app/team/{}/models/delete/{}", team_id, id)
        }
    }

    pub mod datasets {
        pub static INDEX: &str = "/app/team/:team_id/datasets";
        pub static NEW: &str = "/app/team/:team_id/datasets/new";
        pub static DELETE: &str = "/app/team/:team_id/datasets/delete/:id";

        pub fn index_route(team_id: i32) -> String {
            format!("/app/team/{}/datasets", team_id)
        }

        pub fn new_route(team_id: i32) -> String {
            format!("/app/team/{}/datasets/new", team_id)
        }

        pub fn delete_route(team_id: i32, id: i32) -> String {
            format!("/app/team/{}/datasets/delete/{}", team_id, id)
        }
    }

    pub mod documents {
        pub static INDEX: &str = "/app/team/:team_id/dataset/:dataset_id/documents";
        pub static BULK: &str = "/app/team/:team_id/bulk_import";
        pub static UPLOAD: &str = "/app/team/:team_id/dataset/:dataset_id/doc_upload";
        pub static DELETE: &str = "/app/team/:team_id/delete_doc/:document_id";

        pub fn index_route(team_id: i32, dataset_id: i32) -> String {
            format!("/app/team/{}/dataset/{}/documents", team_id, dataset_id)
        }

        pub fn upload_route(team_id: i32, dataset_id: i32) -> String {
            format!("/app/team/{}/dataset/{}/doc_upload", team_id, dataset_id)
        }

        pub fn delete_route(team_id: i32, document_id: i32) -> String {
            format!("/app/team/{}/delete_doc/{}", team_id, document_id)
        }
    }

    pub mod api_keys {
        pub static INDEX: &str = "/app/team/:team_id/api_keys";
        pub static NEW: &str = "/app/team/:team_id/api_keys/new";
        pub static DELETE: &str = "/app/team/:team_id/api_keys/delete/:id";

        pub fn index_route(team_id: i32) -> String {
            format!("/app/team/{}/api_keys", team_id)
        }

        pub fn new_route(team_id: i32) -> String {
            format!("/app/team/{}/api_keys/new", team_id)
        }

        pub fn delete_route(team_id: i32, id: i32) -> String {
            format!("/app/team/{}/api_keys/delete/{}", team_id, id)
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
