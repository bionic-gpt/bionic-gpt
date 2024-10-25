use super::super::{Authentication, CustomError};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries::prompts;
use db::Pool;
use web_pages::routes::console::Index;
use web_pages::{console, render_with_props};

pub async fn index(
    Index { team_id }: Index,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let prompts = prompts::prompts()
        .bind(&transaction, &team_id, &db::PromptType::Model)
        .all()
        .await?;

    let prompt = prompts::prompt()
        .bind(&transaction, &prompts.first().unwrap().id, &team_id)
        .one()
        .await?;

    let html = render_with_props(
        console::index::NewConversation,
        console::index::NewConversationProps {
            team_id,
            rbac,
            prompt,
            prompts,
        },
    );

    Ok(Html(html))
}
