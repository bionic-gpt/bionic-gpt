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
    assert_eq!(body["messages"][0]["id"], "msg-jordan-001");
}

#[tokio::test]
async fn reset_restores_salesforce_contact() {
    let service = app();
    let update = service
        .clone()
        .oneshot(
            Request::patch("/api/salesforce/services/data/v61.0/sobjects/Contact/003JORDANLEE")
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
            Request::get("/api/salesforce/services/data/v61.0/sobjects/Contact/003JORDANLEE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = axum::body::to_bytes(contact.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["Phone"], "+1 555 0100");
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
