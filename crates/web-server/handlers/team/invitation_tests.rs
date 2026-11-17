use super::*;
use crate::handlers::teams::accept_invite::{accept_invite, AcceptInviteForm};
use axum::Form;

// Run against a migrated development database, with the acceptance handlers using
// application_user rather than the fixture owner's privileges.
#[tokio::test]
async fn invitation_acceptance_security() {
    let owner_pool = db::create_pool(&std::env::var("DATABASE_URL").expect("DATABASE_URL"));
    let app_pool = db::create_pool(&std::env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL"));
    let app = app_pool.get().await.unwrap();
    assert_eq!(
        app.query_one("SELECT current_user::text", &[])
            .await
            .unwrap()
            .get::<_, String>(0),
        "application_user"
    );
    drop(app);
    let owner = owner_pool.get().await.unwrap();
    let prefix = uuid::Uuid::new_v4().to_string();
    let mut users = Vec::new();
    let mut teams = Vec::new();
    for index in 0..3 {
        let sub = format!("{prefix}-{index}");
        let email = format!("{sub}@example.test");
        let id: i32 = owner
            .query_one(
                "INSERT INTO iam.users (openid_sub,email) VALUES ($1,$2) RETURNING id",
                &[&sub, &email],
            )
            .await
            .unwrap()
            .get(0);
        owner
            .query(
                "SELECT set_config('row_level_security.user_id', $1, false)",
                &[&id.to_string()],
            )
            .await
            .unwrap();
        let team: i32 = owner
            .query_one(
                "INSERT INTO iam.teams (created_by_user_id) VALUES ($1) RETURNING id",
                &[&id],
            )
            .await
            .unwrap()
            .get(0);
        users.push((id, sub, email));
        teams.push(team);
    }
    let jwt = |index: usize| Jwt {
        sub: users[index].1.clone(),
        // Authorization must use the stored recipient, even if this claim differs.
        email: users[1].2.clone(),
        given_name: None,
        family_name: None,
    };
    let verifier = URL_SAFE_NO_PAD.encode(b"test invitation secret");
    let hash = URL_SAFE_NO_PAD.encode(Sha256::digest(b"test invitation secret"));
    let mut invitations = Vec::new();
    for index in 0..5 {
        let selector = format!("{prefix}-invite-{index}");
        let recipient = if index == 0 { &users[1].2 } else { &users[0].2 };
        let team = if index == 0 { teams[0] } else { teams[1] };
        let id: i32 = owner.query_one(
            "INSERT INTO iam.invitations (team_id,email,first_name,last_name,roles,invitation_selector,invitation_verifier_hash) VALUES ($1,$2,'Test','Invite',ARRAY['TeamManager']::public.role[],$3,$4) RETURNING id",
            &[&team, recipient, &selector, &hash],
        ).await.unwrap().get(0);
        invitations.push((id, selector));
    }
    let victim_team = teams[2];
    let post = |index: usize, id: i32| {
        let pool = app_pool.clone();
        let user = jwt(index);
        async move {
            // The original exploit fields are ignored, including redirect input.
            let form: AcceptInviteForm = serde_urlencoded::from_str(&format!(
                "invite_id={id}&new_team_id={}&team_id={}",
                victim_team, victim_team
            ))
            .unwrap();
            accept_invite(
                web_pages::routes::teams::AcceptInvite {},
                user,
                Extension(pool),
                Form(form),
            )
            .await
            .map(IntoResponse::into_response)
        }
    };

    // A foreign recipient cannot accept, even with a matching email claim.
    assert!(matches!(
        post(0, invitations[0].0).await,
        Err(CustomError::Authorization)
    ));
    assert!(matches!(post(0, -1).await, Err(CustomError::Authorization)));
    // An owned invitation grants only its own team, never the forged victim team.
    let response = post(0, invitations[1].0).await.unwrap();
    assert_eq!(
        response.headers()["location"],
        web_pages::routes::teams::Switch {
            team_id: db::team_public_id::encode(teams[1]).unwrap()
        }
        .to_string()
    );
    let membership = owner
        .query(
            "SELECT team_id,roles::text FROM iam.team_users WHERE user_id=$1",
            &[&users[0].0],
        )
        .await
        .unwrap();
    assert_eq!(membership.len(), 1);
    assert_eq!(membership[0].get::<_, i32>(0), teams[1]);
    assert_eq!(membership[0].get::<_, String>(1), "{TeamManager}");
    assert!(matches!(
        post(0, invitations[1].0).await,
        Err(CustomError::Authorization)
    ));
    assert!(owner
        .query_opt(
            "SELECT id FROM iam.invitations WHERE id=$1",
            &[&invitations[0].0]
        )
        .await
        .unwrap()
        .is_some());
    // Duplicate membership fails and must restore the claimed invitation.
    assert!(post(0, invitations[2].0).await.is_err());
    assert!(owner
        .query_opt(
            "SELECT id FROM iam.invitations WHERE id=$1",
            &[&invitations[2].0]
        )
        .await
        .unwrap()
        .is_some());
    owner
        .execute(
            "DELETE FROM iam.team_users WHERE user_id=$1",
            &[&users[0].0],
        )
        .await
        .unwrap();

    // Wrong validators and recipients must not consume an email invitation.
    assert!(matches!(
        accept_invitation(&app_pool, jwt(0), &invitations[3].1, "!").await,
        Err(CustomError::Authorization)
    ));
    assert!(matches!(
        accept_invitation(
            &app_pool,
            jwt(0),
            &invitations[3].1,
            &URL_SAFE_NO_PAD.encode(b"wrong")
        )
        .await,
        Err(CustomError::Authorization)
    ));
    assert!(matches!(
        accept_invitation(&app_pool, jwt(1), &invitations[3].1, &verifier).await,
        Err(CustomError::Authorization)
    ));
    assert_eq!(
        accept_invitation(&app_pool, jwt(0), &invitations[3].1, &verifier)
            .await
            .unwrap(),
        db::team_public_id::encode(teams[1]).unwrap()
    );
    assert!(matches!(
        accept_invitation(&app_pool, jwt(0), &invitations[3].1, &verifier).await,
        Err(CustomError::Authorization)
    ));
    owner
        .execute(
            "DELETE FROM iam.team_users WHERE user_id=$1",
            &[&users[0].0],
        )
        .await
        .unwrap();

    // Both entrypoints race for the same invitation; only one may accept it.
    let (first, second) = tokio::join!(
        post(0, invitations[4].0),
        accept_invitation(&app_pool, jwt(0), &invitations[4].1, &verifier)
    );
    assert_eq!(usize::from(first.is_ok()) + usize::from(second.is_ok()), 1);

    owner
        .execute(
            "DELETE FROM iam.invitations WHERE team_id = ANY($1)",
            &[&teams],
        )
        .await
        .unwrap();
    owner
        .execute(
            "DELETE FROM iam.team_users WHERE team_id = ANY($1)",
            &[&teams],
        )
        .await
        .unwrap();
    owner
        .execute("DELETE FROM iam.teams WHERE id = ANY($1)", &[&teams])
        .await
        .unwrap();
    let ids: Vec<i32> = users.iter().map(|u| u.0).collect();
    owner
        .execute("DELETE FROM iam.users WHERE id = ANY($1)", &[&ids])
        .await
        .unwrap();
}
