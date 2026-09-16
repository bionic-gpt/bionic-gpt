use crate::store::World;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn routes() -> Router<Arc<World>> {
    let mut router = Router::new()
        .route("/services/data/v61.0/query", get(query))
        .route("/services/data/v61.0/search", get(search))
        .route(
            "/services/data/v61.0/sobjects/{s_object_type}/{id}",
            get(get_record).patch(update_record).delete(delete_record),
        )
        .route(
            "/services/data/v61.0/actions/standard/convertLead",
            post(action),
        )
        .route(
            "/services/data/v61.0/actions/standard/emailSimple",
            post(action),
        )
        .route(
            "/services/data/v61.0/analytics/reports/{report_id}",
            get(report),
        )
        .route(
            "/services/data/v61.0/actions/custom/flow/{flow_name}",
            post(action),
        );

    for object_type in [
        "Case",
        "Lead",
        "Note",
        "Task",
        "Event",
        "Account",
        "Contact",
        "Campaign",
        "Document",
        "Attachment",
        "CaseComment",
        "ContentNote",
        "Opportunity",
        "CampaignMember",
        "ContentVersion",
        "ContentDocumentLink",
    ] {
        router = router.route(
            &format!("/services/data/v61.0/sobjects/{object_type}"),
            post(create_record),
        );
    }
    router
}

#[derive(Deserialize, Default)]
struct QueryParams {
    q: Option<String>,
}

async fn create_record(
    State(world): State<Arc<World>>,
    Path(object_type): Path<String>,
    Json(body): Json<Value>,
) -> Json<Value> {
    Json(
        json!({"id": world.write().await.salesforce.create(&object_type, body)["id"], "success": true, "errors": []}),
    )
}

async fn get_record(
    State(world): State<Arc<World>>,
    Path((object_type, id)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    world
        .read()
        .await
        .salesforce
        .records
        .get(&object_type)
        .and_then(|records| records.get(&id))
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_record(
    State(world): State<Arc<World>>,
    Path((object_type, id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> StatusCode {
    let mut state = world.write().await;
    let Some(record) = state
        .salesforce
        .records
        .get_mut(&object_type)
        .and_then(|records| records.get_mut(&id))
    else {
        return StatusCode::NOT_FOUND;
    };
    if let (Value::Object(existing), Value::Object(changes)) = (record, body) {
        existing.extend(changes);
    }
    StatusCode::NO_CONTENT
}

async fn delete_record(
    State(world): State<Arc<World>>,
    Path((object_type, id)): Path<(String, String)>,
) -> StatusCode {
    if world
        .write()
        .await
        .salesforce
        .records
        .get_mut(&object_type)
        .and_then(|records| records.remove(&id))
        .is_some()
    {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn query(
    State(world): State<Arc<World>>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Value>, StatusCode> {
    let Some(soql) = params.q else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let object_type = soql
        .split_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
        .find(|parts| parts[0].eq_ignore_ascii_case("from"))
        .map(|parts| parts[1])
        .ok_or(StatusCode::BAD_REQUEST)?;
    let records = world
        .read()
        .await
        .salesforce
        .records
        .get(object_type)
        .map(|records| records.values().cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    Ok(Json(
        json!({"totalSize": records.len(), "done": true, "records": records}),
    ))
}

async fn search(
    State(world): State<Arc<World>>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Value>, StatusCode> {
    let Some(search) = params.q else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let needle = search.to_lowercase();
    let records = world
        .read()
        .await
        .salesforce
        .records
        .values()
        .flat_map(|records| records.values())
        .filter(|record| record.to_string().to_lowercase().contains(&needle))
        .cloned()
        .collect::<Vec<_>>();
    Ok(Json(json!({"searchRecords": records})))
}

async fn action(Json(body): Json<Value>) -> Json<Value> {
    Json(json!([{"success": true, "errors": [], "result": body}]))
}

async fn report(Path(_report_id): Path<String>, Query(_params): Query<QueryParams>) -> Json<Value> {
    Json(json!({"reportMetadata": {}, "groupingsDown": [], "groupingsAcross": [], "factMap": {}}))
}
