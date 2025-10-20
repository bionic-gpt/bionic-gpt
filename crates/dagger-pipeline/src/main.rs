//mod args;
//mod pipeline;

use dagger_sdk::{File, HostDirectoryOpts, Query, Service};
use eyre::Result;

pub(crate) const BASE_IMAGE: &str = "purtontech/rust-on-nails-devcontainer:1.3.18";
pub(crate) const POSTGRES_IMAGE: &str = "ankane/pgvector";
pub(crate) const DB_PASSWORD: &str = "testpassword";
pub(crate) const DATABASE_URL: &str =
    "postgresql://postgres:testpassword@postgres:5432/postgres?sslmode=disable";

#[tokio::main]
async fn main() -> Result<()> {
    dagger_sdk::connect(|client| async move {
        let backend = build_backend(&client).await;

        println!("exe {:?}", backend.name().await.unwrap());

        Ok(())
    })
    .await?;

    Ok(())
}

async fn build_backend(client: &Query) -> File {
    let host_source_dir = client.host().directory_opts(
        ".",
        HostDirectoryOpts {
            exclude: Some(vec![
                "crates/web-assets/node_modules",
                "crates/web-assets/dist",
                "target",
            ]),
            include: None,
            no_cache: None,
            gitignore: None,
        },
    );

    let postgres_service = postgres_service(client);

    let after_postgres = client
        .container()
        .from(BASE_IMAGE)
        .with_directory("/workspace", host_source_dir)
        .with_workdir("/workspace")
        .with_user("root")
        .with_service_binding("postgres", postgres_service)
        .with_env_variable("DATABASE_URL", DATABASE_URL)
        .with_env_variable("APP_DATABASE_URL", DATABASE_URL);

    let after_migrations = after_postgres.with_exec(vec![
        "dbmate",
        "--wait",
        "--migrations-dir",
        "crates/db/migrations",
        "up",
    ]);

    let after_node = after_migrations
        .with_exec(vec!["npm", "--prefix", "/crates/web-assets", "install"])
        .with_exec(vec![
            "npm",
            "--prefix",
            "/crates/web-assets",
            "run",
            "release",
        ]);

    let after_rust =
        after_node
            .with_workdir("/workspace")
            .with_exec(vec!["cargo", "build", "--release"]);

    after_rust.file("target/release/web-server")
}

fn postgres_service(client: &Query) -> Service {
    client
        .container()
        .from(POSTGRES_IMAGE)
        .with_env_variable("POSTGRES_PASSWORD", DB_PASSWORD)
        .with_exposed_port(5432)
        .as_service()
}
