use db::{ObjectStorage, Pool, TokioPostgresError};

pub fn upload(_pool: Pool, _object: ObjectStorage) -> Result<i32, TokioPostgresError> {
    todo!()
}

pub fn get(_pool: Pool, _id: i32) -> Result<ObjectStorage, TokioPostgresError> {
    todo!()
}
