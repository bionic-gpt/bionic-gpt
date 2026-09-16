use crate::store::World;
use axum::{
    extract::{Multipart, Path, Query, State},
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
            "/services/data/v61.0/sobjects/{s_object_type}",
            post(create_record),
        )
        .route(
            "/services/data/v61.0/actions/standard/convertLead",
            post(convert_lead),
        )
        .route(
            "/services/data/v61.0/actions/standard/emailSimple",
            post(email_simple),
        )
        .route(
            "/services/data/v61.0/analytics/reports/{report_id}",
            get(report),
        )
        .route(
            "/services/data/v61.0/actions/custom/flow/{flow_name}",
            post(launch_flow),
        );

    router = router.route(
        "/services/data/v61.0/sobjects/ContentVersion",
        post(upload_content_version),
    );

    router
}

async fn upload_content_version(
    State(world): State<Arc<World>>,
    mut multipart: Multipart,
) -> Result<Json<Value>, StatusCode> {
    let mut record = serde_json::Map::new();
    let mut has_version_data = false;
    let mut has_path = false;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let name = field.name().unwrap_or_default().to_string();
        if name == "VersionData" {
            let bytes = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            record.insert("VersionData".to_string(), json!({"size": bytes.len()}));
            has_version_data = true;
        } else {
            let value = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            if name == "PathOnClient" {
                has_path = true;
            }
            record.insert(name, Value::String(value));
        }
    }
    if !has_version_data || !has_path {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(json!({
        "id": world.write().await.salesforce.create("ContentVersion", Value::Object(record))["id"],
        "success": true,
        "errors": []
    })))
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
    let parsed = parse_soql(&soql).ok_or(StatusCode::BAD_REQUEST)?;
    let state = world.read().await;
    let records = state
        .salesforce
        .records
        .get(&parsed.object_type)
        .map(|records| {
            let mut records = records
                .values()
                .filter(|record| matches_filters(record, &parsed.filters))
                .map(|record| project_record(record, &parsed.fields))
                .collect::<Vec<_>>();
            if let Some((field, descending)) = &parsed.order_by {
                records.sort_by(|left, right| {
                    let ordering = left[field].to_string().cmp(&right[field].to_string());
                    if *descending {
                        ordering.reverse()
                    } else {
                        ordering
                    }
                });
            }
            if let Some(limit) = parsed.limit {
                records.truncate(limit);
            }
            records
        })
        .unwrap_or_default();
    let total = records.len();
    Ok(Json(json!({
        "totalSize": total,
        "count": total,
        "done": true,
        "results": records.clone(),
        "records": records,
    })))
}

async fn search(
    State(world): State<Arc<World>>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Value>, StatusCode> {
    let Some(search) = params.q else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let needle = search
        .split('{')
        .nth(1)
        .and_then(|part| part.split('}').next())
        .unwrap_or(&search)
        .to_lowercase();
    let upper_search = search.to_ascii_uppercase();
    let object_types = upper_search
        .find("RETURNING")
        .map(|returning_start| {
            let returning = &search[returning_start + "RETURNING".len()..];
            returning
                .split(',')
                .map(|item| item.split('(').next().unwrap_or(item).trim().to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let state = world.read().await;
    let records = state
        .salesforce
        .records
        .iter()
        .filter(|(object_type, _)| {
            object_types.is_empty()
                || object_types
                    .iter()
                    .any(|wanted| wanted.eq_ignore_ascii_case(object_type))
        })
        .flat_map(|(_, records)| records.values())
        .filter(|record| record.to_string().to_lowercase().contains(&needle))
        .cloned()
        .collect::<Vec<_>>();
    Ok(Json(json!({"query": search, "searchRecords": records})))
}

struct ParsedSoql {
    object_type: String,
    fields: Vec<String>,
    filters: Vec<(String, String)>,
    order_by: Option<(String, bool)>,
    limit: Option<usize>,
}

fn parse_soql(query: &str) -> Option<ParsedSoql> {
    let upper = query.to_ascii_uppercase();
    let from = upper.find(" FROM ")?;
    if !upper.starts_with("SELECT ") {
        return None;
    }
    let fields = query[7..from]
        .split(',')
        .map(str::trim)
        .filter(|field| !field.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    let rest = &query[from + 6..];
    let rest_upper = &upper[from + 6..];
    let object_end = [
        rest_upper.find(" WHERE "),
        rest_upper.find(" ORDER BY "),
        rest_upper.find(" LIMIT "),
    ]
    .into_iter()
    .flatten()
    .min()
    .unwrap_or(rest.len());
    let object_type = rest[..object_end].trim().to_string();
    if object_type.is_empty() {
        return None;
    }
    let filters = rest_upper
        .find(" WHERE ")
        .map(|start| {
            let end = [rest_upper.find(" ORDER BY "), rest_upper.find(" LIMIT ")]
                .into_iter()
                .flatten()
                .filter(|end| *end > start)
                .min()
                .unwrap_or(rest.len());
            rest[start + 7..end]
                .split(" AND ")
                .filter_map(|filter| {
                    filter.split_once('=').map(|(field, value)| {
                        (
                            field.trim().to_string(),
                            value.trim().trim_matches('\'').to_string(),
                        )
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let order_by = rest_upper.find(" ORDER BY ").map(|start| {
        let end = rest_upper
            .find(" LIMIT ")
            .filter(|end| *end > start)
            .unwrap_or(rest.len());
        let mut parts = rest[start + 10..end].split_whitespace();
        let field = parts.next().unwrap_or_default().to_string();
        let descending = parts
            .next()
            .is_some_and(|direction| direction.eq_ignore_ascii_case("DESC"));
        (field, descending)
    });
    let limit = rest_upper
        .find(" LIMIT ")
        .and_then(|start| rest[start + 7..].trim().parse().ok());
    Some(ParsedSoql {
        object_type,
        fields,
        filters,
        order_by,
        limit,
    })
}

fn matches_filters(record: &Value, filters: &[(String, String)]) -> bool {
    filters.iter().all(|(field, expected)| {
        record[field]
            .as_str()
            .is_some_and(|actual| actual == expected)
            || record[field].to_string().trim_matches('"') == expected
    })
}

fn project_record(record: &Value, fields: &[String]) -> Value {
    if fields.iter().any(|field| field == "*") {
        return record.clone();
    }
    let mut projected = serde_json::Map::new();
    for field in fields {
        if let Some(value) = record.get(field) {
            projected.insert(field.clone(), value.clone());
        }
    }
    if let Some(attributes) = record.get("attributes") {
        projected.insert("attributes".to_string(), attributes.clone());
    }
    Value::Object(projected)
}

async fn convert_lead(State(world): State<Arc<World>>, Json(body): Json<Value>) -> Json<Value> {
    let mut state = world.write().await;
    let lead_id = body.get("leadId").and_then(Value::as_str);
    let contact_id = lead_id.and_then(|id| {
        state
            .salesforce
            .records
            .get("Lead")
            .and_then(|leads| leads.get(id))
            .cloned()
            .map(|lead| {
                let mut contact = serde_json::Map::new();
                if let Some(fields) = lead.as_object() {
                    for field in ["FirstName", "LastName", "Email", "Phone"] {
                        if let Some(value) = fields.get(field) {
                            contact.insert(field.to_string(), value.clone());
                        }
                    }
                }
                state.salesforce.create("Contact", Value::Object(contact))["id"].clone()
            })
    });
    if let Some(id) = lead_id {
        if let Some(lead) = state
            .salesforce
            .records
            .get_mut("Lead")
            .and_then(|leads| leads.get_mut(id))
        {
            lead["IsConverted"] = Value::Bool(true);
        }
    }
    Json(json!([{"success": true, "errors": [], "contactId": contact_id, "leadId": lead_id}]))
}

async fn email_simple(Json(body): Json<Value>) -> Json<Value> {
    Json(json!([{"success": true, "errors": [], "result": body}]))
}

async fn launch_flow(Path(flow_name): Path<String>, Json(body): Json<Value>) -> Json<Value> {
    Json(json!([{"success": true, "errors": [], "flowName": flow_name, "result": body}]))
}

async fn report(Path(_report_id): Path<String>, Query(_params): Query<QueryParams>) -> Json<Value> {
    Json(json!({"reportMetadata": {}, "groupingsDown": [], "groupingsAcross": [], "factMap": {}}))
}
