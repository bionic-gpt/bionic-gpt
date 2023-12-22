use crate::{authentication::Authentication, errors::CustomError};
use axum::{
    extract::Extension,
    response::{IntoResponse, Redirect},
};
use db::types;
use db::types::public::{DatasetConnection, Visibility};
use db::Pool;
use db::{queries, Transaction};

// After a user has logged in or registered, check they have an entry in
// the team table. If not, then create one.
pub async fn authentication_handler(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    dbg!("User setup");

    let user = queries::users::user_by_openid_sub()
        .bind(&transaction, &current_user.sub)
        .one()
        .await;

    let user_id = if let Ok(user) = user {
        user.id
    } else {
        queries::users::insert()
            .bind(
                &transaction,
                &current_user.sub,
                &current_user.email,
                &current_user.given_name,
                &current_user.family_name,
            )
            .one()
            .await?
    };

    transaction
        .query(
            &format!("SET LOCAL row_level_security.user_id = {}", user_id),
            &[],
        )
        .await?;

    let org = queries::teams::get_primary_team()
        .bind(&transaction, &user_id)
        .one()
        .await;

    if let Ok(org) = org {
        return Ok(Redirect::to(&ui_pages::routes::console::index_route(
            org.id,
        )));
    }
    let inserted_org_id = setup_user(&transaction, user_id).await?;

    transaction.commit().await?;

    Ok(Redirect::to(&ui_pages::routes::console::index_route(
        inserted_org_id,
    )))
}

// Creates the users default prompt and anything else they need
async fn setup_user(transaction: &Transaction<'_>, current_user: i32) -> Result<i32, CustomError> {
    let inserted_org_id = queries::teams::insert_team()
        .bind(transaction)
        .one()
        .await?;

    let roles = vec![
        types::public::Role::TeamManager,
        types::public::Role::Collaborator,
    ];

    queries::teams::add_user_to_team()
        .bind(transaction, &current_user, &inserted_org_id, &roles)
        .await?;

    let model = queries::models::get_system_model()
        .bind(transaction)
        .one()
        .await?;

    let system_prompt: Option<String> = None;

    queries::prompts::insert()
        .bind(
            transaction,
            &inserted_org_id,
            &model.id,
            &"Default (Exclude All Datasets)",
            &Visibility::Private,
            &DatasetConnection::None,
            &system_prompt,
            &3,
            &10,
            &1024,
            &100,
            &0.7,
            &0.1,
        )
        .one()
        .await?;

    Ok(inserted_org_id)
}
