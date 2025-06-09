// All the routes of the application are mapped here and are typesafe
// https://docs.rs/axum-extra/latest/axum_extra/routing/trait.TypedPath.html

pub mod history {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/history")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/search")]
    pub struct Search {
        pub team_id: i32,
    }
}

pub mod rate_limits {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/rate_limits")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/rate_limits/upsert")]
    pub struct Upsert {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/rate_limits/delete/{id}")]
    pub struct Delete {
        pub team_id: i32,
        pub id: i32,
    }
}

pub mod api_keys {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/api_keys")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/api_keys/new")]
    pub struct New {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/api_keys/delete/{id}")]
    pub struct Delete {
        pub team_id: i32,
        pub id: i32,
    }
}

pub mod audit_trail {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/audit_trail")]
    pub struct Index {
        pub team_id: i32,
    }
}

pub mod document_pipelines {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/pipelines")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/pipelines/new")]
    pub struct New {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/pipelines/delete/{id}")]
    pub struct Delete {
        pub team_id: i32,
        pub id: i32,
    }
}

pub mod console {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/console")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/set_prompt")]
    pub struct SetPrompt {}

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/set_tools")]
    pub struct SetTools {}

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/console/{conversation_id}")]
    pub struct Conversation {
        pub team_id: i32,
        pub conversation_id: i64,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/send_message")]
    pub struct SendMessage {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/update_response")]
    pub struct UpdateResponse {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/console/delete/{id}")]
    pub struct Delete {
        pub team_id: i32,
        pub id: i64,
    }
}

pub mod prompts {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/assistant/{prompt_id}/console/{conversation_id}")]
    pub struct Conversation {
        pub team_id: i32,
        pub prompt_id: i32,
        pub conversation_id: i64,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/new_chat/{prompt_id}")]
    pub struct NewChat {
        pub team_id: i32,
        pub prompt_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/my_prompts")]
    pub struct MyPrompts {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/prompts/new")]
    pub struct New {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/prompts/edit/{prompt_id}")]
    pub struct Edit {
        pub team_id: i32,
        pub prompt_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/prompts")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/prompts/{id}/image")]
    pub struct Image {
        pub team_id: i32,
        pub id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/prompts/upsert")]
    pub struct Upsert {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/prompts/delete/{id}")]
    pub struct Delete {
        pub team_id: i32,
        pub id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/prompt/{prompt_id}/delete_conv/{conversation_id}")]
    pub struct DeleteConv {
        pub team_id: i32,
        pub prompt_id: i32,
        pub conversation_id: i64,
    }
}

pub mod models {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/models")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/models/new")]
    pub struct New {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/models/delete/{id}")]
    pub struct Delete {
        pub team_id: i32,
        pub id: i32,
    }
}

pub mod integrations {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/integrations")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/integration/{id}")]
    pub struct View {
        pub team_id: i32,
        pub id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/integrations/new")]
    pub struct New {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/integrations/edit/{id}")]
    pub struct Edit {
        pub team_id: i32,
        pub id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/integrations/delete/{id}")]
    pub struct Delete {
        pub team_id: i32,
        pub id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/integrations/{integration_id}/connect")]
    pub struct Connect {
        pub team_id: i32,
        pub integration_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/integrations/{integration_id}/oauth2/callback")]
    pub struct OAuth2Callback {
        pub team_id: i32,
        pub integration_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/integrations/{integration_id}/configure_api_key")]
    pub struct ConfigureApiKey {
        pub team_id: i32,
        pub integration_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/integrations/{integration_id}/connections/api-key/{connection_id}/delete")]
    pub struct DeleteApiKeyConnection {
        pub team_id: i32,
        pub integration_id: i32,
        pub connection_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/integrations/{integration_id}/connections/oauth2/{connection_id}/delete")]
    pub struct DeleteOauth2Connection {
        pub team_id: i32,
        pub integration_id: i32,
        pub connection_id: i32,
    }
}

pub mod workflows {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/workflows")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/workflow/{id}")]
    pub struct View {
        pub team_id: i32,
        pub id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/workflows/upsert")]
    pub struct Upsert {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/workflows/delete/{id}")]
    pub struct Delete {
        pub team_id: i32,
        pub id: i32,
    }
}

pub mod datasets {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/datasets")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/datasets/upsert")]
    pub struct Upsert {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/datasets/delete/{id}")]
    pub struct Delete {
        pub team_id: i32,
        pub id: i32,
    }
}

pub mod documents {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/dataset/{dataset_id}/documents")]
    pub struct Index {
        pub team_id: i32,
        pub dataset_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/bulk_import")]
    pub struct Bulk {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/dataset/{dataset_id}/doc_upload")]
    pub struct Upload {
        pub team_id: i32,
        pub dataset_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/processing/{document_id}")]
    pub struct Processing {
        pub team_id: i32,
        pub document_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/delete_doc/{document_id}")]
    pub struct Delete {
        pub team_id: i32,
        pub document_id: i32,
    }
}

pub mod teams {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/delete_team")]
    pub struct Delete {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/switch")]
    pub struct Switch {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/new")]
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
    #[typed_path("/app/team/{team_id}")]
    pub struct Index {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/teams_popup")]
    pub struct Popup {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/create_invite")]
    pub struct CreateInvite {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/invite/{invite_selector}/{invite_validator}")]
    pub struct AcceptInvite {
        pub invite_selector: String,
        pub invite_validator: String,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/delete")]
    pub struct Delete {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/delete_invite")]
    pub struct DeleteInvite {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/set_name")]
    pub struct SetName {
        pub team_id: i32,
    }
}

pub mod profile {
    use axum_extra::routing::TypedPath;
    use serde::Deserialize;

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/set_details")]
    pub struct SetDetails {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/profile")]
    pub struct Profile {
        pub team_id: i32,
    }

    #[derive(TypedPath, Deserialize)]
    #[typed_path("/app/team/{team_id}/profile_popup")]
    pub struct ProfilePopup {
        pub team_id: i32,
    }
}
