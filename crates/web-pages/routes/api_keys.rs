use axum_extra::routing::TypedPath;
use serde::Deserialize;

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/api_keys")]
pub struct Index {
    pub team_id: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/api_keys/new")]
pub struct New {
    pub team_id: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/api_keys/delete/{id}")]
pub struct Delete {
    pub team_id: String,
    pub id: i32,
}
