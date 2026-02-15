use axum_extra::routing::TypedPath;
use serde::Deserialize;

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/providers")]
pub struct Index {
    pub team_id: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/providers/new")]
pub struct New {
    pub team_id: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/providers/edit/{id}")]
pub struct Edit {
    pub team_id: String,
    pub id: i32,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/providers/upsert")]
pub struct Upsert {
    pub team_id: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{team_id}/providers/delete/{id}")]
pub struct Delete {
    pub team_id: String,
    pub id: i32,
}
