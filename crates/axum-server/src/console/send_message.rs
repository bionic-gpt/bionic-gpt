use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries::{chats, prompts};
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct Message {
    pub message: String,
    pub prompt_id: Option<i32>,
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

        // Turn the users message into something the vector database can use
        let embeddings = open_api::get_embeddings(&message.message)
            .await
            .map_err(|e| CustomError::ExternalApi(e.to_string()))?;

        let related_context = if let Some(prompt_id) = message.prompt_id {
            // Which datasets does the prompt use
            let datasets = prompts::prompt_datasets()
                .bind(&transaction, &prompt_id)
                .all()
                .await?;
            // We just need the id's
            let datasets: Vec<i32> = datasets.iter().map(|dataset| dataset.dataset_id).collect();

            // Format the embeddings in PGVector format
            let embedding_data = pgvector::Vector::from(embeddings.clone());

            // Find sections of documents that are related to the users question
            let related_context = transaction
                .query(
                    "
                        SELECT 
                            text 
                        FROM 
                            embeddings
                        WHERE
                            document_id IN 
                                (SELECT id FROM documents WHERE dataset_id = ANY($1))
                        ORDER BY 
                            embeddings <-> $2 LIMIT 1;
                    ",
                    &[&datasets, &embedding_data],
                )
                .await?;

            // Just get the text from the returned rows
            let related_context: Vec<String> = related_context
                .into_iter()
                .map(|content| content.get(0))
                .collect();
            related_context
        } else {
            Default::default()
        };
        let related_context = related_context.join(" ");

        // Get the prompt template or use a default one
        let template = if let Some(prompt_id) = message.prompt_id {
            let prompt = prompts::prompt()
                .bind(&transaction, &prompt_id)
                .one()
                .await?;
            prompt.template
        } else {
            "The prompt below is a question to answer, a task to complete, 
or a conversation to respond to; decide which and write an appropriate response. 
You can use the data in the Data section to help with your reply.
### Prompt:
{{.Input}}
### Data:
{{.Data}}
### Response:"
                .to_string()
        };

        // Combine the users question, relevant content and the prompt template
        // to generate a prompt we send to the large language model

        let prompt = template.replace("{{.Input}}", &message.message);
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
