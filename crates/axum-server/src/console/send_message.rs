use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries::{chats, prompts};
use db::{DatasetConnection, Pool, Transaction};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Message {
    pub message: String,
    pub prompt_id: i32,
}

pub async fn send_message(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Path(team_id): Path<i32>,
    Form(message): Form<Message>,
) -> Result<impl IntoResponse, CustomError> {
    if message.validate().is_ok() {
        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;

        super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

        // Get the prompt
        let prompt = prompts::prompt()
            .bind(&transaction, &message.prompt_id)
            .one()
            .await?;

        // Get related context
        let related_context = get_related_context(
            &transaction,
            &message.message,
            prompt.dataset_connection,
            message.prompt_id,
            team_id,
        )
        .await?;
        let related_context = related_context.join(" ");

        // Combine the users question, relevant content and the prompt template
        // to generate a prompt we send to the large language model

        let prompt = prompt.template.replace("{{.Input}}", &message.message);
        let prompt = prompt.replace("{{.Data}}", &related_context);

        dbg!(&prompt);

        // Store the prompt, ready for the front end webcomponent to pickup
        chats::new_chat()
            .bind(
                &transaction,
                &current_user.user_id,
                &team_id,
                &message.message,
                &prompt,
            )
            .await?;

        transaction.commit().await?;

        crate::layout::redirect_and_snackbar(
            &ui_components::routes::console::index_route(team_id),
            "Prompt is now being processed",
        )
    } else {
        crate::layout::redirect_and_snackbar(
            &ui_components::routes::console::index_route(team_id),
            "Problem Processing Form",
        )
    }
}

// Query the vector database using a similarity search.
// The prompt decides how we use the datasets
async fn get_related_context(
    transaction: &Transaction<'_>,
    message: &str,
    dataset_connection: DatasetConnection,
    prompt_id: i32,
    organisation_id: i32,
) -> Result<Vec<String>, CustomError> {
    if dataset_connection == DatasetConnection::None {
        return Ok(Default::default());
    }

    // Turn the users message into something the vector database can use
    let embeddings = open_api::get_embeddings(message)
        .await
        .map_err(|e| CustomError::ExternalApi(e.to_string()))?;

    // Which datasets does the prompt use
    let datasets = prompts::prompt_datasets()
        .bind(transaction, &prompt_id)
        .all()
        .await?;
    // We just need the id's
    let datasets: Vec<i32> = datasets.iter().map(|dataset| dataset.dataset_id).collect();

    // Format the embeddings in PGVector format
    let embedding_data = pgvector::Vector::from(embeddings.clone());

    match dataset_connection {
        DatasetConnection::None => Ok(Default::default()),
        DatasetConnection::All => {
            // Find sections of documents that are related to the users question
            let related_context = transaction
                .query(
                    "
                            SELECT 
                                text 
                            FROM 
                                embeddings
                            WHERE
                                document_id IN (
                                    SELECT id FROM documents WHERE dataset_id IN (
                                        SELECT id FROM datasets WHERE organisation_id IN (
                                            SELECT organisation_id FROM organisation_users 
                                            WHERE user_id = current_app_user()
                                            AND organisation_id = $1
                                        )
                                    )
                                )
                            ORDER BY 
                                embeddings <-> $2 LIMIT 1;
                        ",
                    &[&organisation_id, &embedding_data],
                )
                .await?;

            // Just get the text from the returned rows
            let related_context: Vec<String> = related_context
                .into_iter()
                .map(|content| content.get(0))
                .collect();
            Ok(related_context)
        }
        DatasetConnection::Selected => {
            // Find sections of documents that are related to the users question
            let related_context = transaction
                .query(
                    "
                        SELECT 
                            text 
                        FROM 
                            embeddings
                        WHERE
                            document_id IN (
                                SELECT id FROM documents WHERE dataset_id IN (
                                    SELECT id FROM datasets WHERE organisation_id IN (
                                        SELECT organisation_id FROM organisation_users 
                                        WHERE user_id = current_app_user()
                                        AND organisation_id = $1
                                    )
                                    AND dataset_id = ANY($2)
                                )
                            )
                        ORDER BY 
                            embeddings <-> $3 LIMIT 1;
                        ",
                    &[&organisation_id, &datasets, &embedding_data],
                )
                .await?;

            // Just get the text from the returned rows
            let related_context: Vec<String> = related_context
                .into_iter()
                .map(|content| content.get(0))
                .collect();
            Ok(related_context)
        }
    }
}
