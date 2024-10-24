use db::{ObjectStorage, Pool, TokioPostgresError};

pub fn upload(
    _pool: Pool,
    _user_id: i32,
    _team_id: i32,
    _file_name: &str,
    _bytes: &[u8],
) -> Result<i32, TokioPostgresError> {
    todo!()
}

pub fn get(_pool: Pool, _id: i32) -> Result<ObjectStorage, TokioPostgresError> {
    todo!()
}
