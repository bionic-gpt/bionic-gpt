use axum_extra::routing::TypedPath;
use serde::Deserialize;

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/rate_limits")]
pub struct Index {
    pub team_id: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/rate_limits/upsert")]
pub struct Upsert {
    pub team_id: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/rate_limits/delete/{id}")]
pub struct Delete {
    pub team_id: String,
    pub id: i32,
}
