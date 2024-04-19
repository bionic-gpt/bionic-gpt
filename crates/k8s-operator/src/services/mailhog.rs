use crate::error::Error;
use kube::Client;
use crate::cli::apply;

const MAILHOG_YAML: &str = include_str!("../../config/mailhog.yaml");

// Large Language Model
pub async fn deploy(client: Client, _namespace: &str) -> Result<(), Error> {
    apply::apply(&client, MAILHOG_YAML, None).await.unwrap();

    Ok(())
}
