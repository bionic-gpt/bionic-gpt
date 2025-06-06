use crate::{self as db, authz, Pool, Transaction};
use deadpool_postgres::Client;
use std::fmt;

pub struct AuthorizedTransaction<'a> {
    pub client: Client,
    pub transaction: Transaction<'a>,
    pub rbac: authz::Rbac,
}

#[derive(Debug)]
pub enum AuthorizedTransactionError {
    PoolError(db::PoolError),
    PostgresError(db::TokioPostgresError),
}

impl fmt::Display for AuthorizedTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthorizedTransactionError::PoolError(e) => write!(f, "Pool Error: {}", e),
            AuthorizedTransactionError::PostgresError(e) => write!(f, "Postgres Error: {}", e),
        }
    }
}

impl std::error::Error for AuthorizedTransactionError {}

impl From<db::PoolError> for AuthorizedTransactionError {
    fn from(err: db::PoolError) -> Self {
        AuthorizedTransactionError::PoolError(err)
    }
}

impl From<db::TokioPostgresError> for AuthorizedTransactionError {
    fn from(err: db::TokioPostgresError) -> Self {
        AuthorizedTransactionError::PostgresError(err)
    }
}

impl<'a> AuthorizedTransaction<'a> {
    pub async fn new(
        pool: &Pool,
        auth: &authz::Authentication,
        team_id: i32,
    ) -> Result<Self, AuthorizedTransactionError> {
        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;
        let rbac = authz::get_permissions(&transaction, auth, team_id).await?;
        Ok(Self {
            client,
            transaction,
            rbac,
        })
    }

    pub async fn commit(self) -> Result<(), AuthorizedTransactionError> {
        self.transaction.commit().await?;
        Ok(())
    }
}
