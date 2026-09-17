use crate::store::World;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::{collections::BTreeSet, sync::Arc};

pub fn routes() -> Router<Arc<World>> {
    Router::new()
        .route("/v4/spreadsheets/{spreadsheet_id}", get(get_spreadsheet))
        .route(
            "/v4/spreadsheets/{spreadsheet_id}/values/{range}",
            get(get_values),
        )
}

async fn get_spreadsheet(
    State(world): State<Arc<World>>,
    Path(spreadsheet_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let state = world.read().await;
    let spreadsheet = state
        .google_sheets
        .spreadsheets
        .get(&spreadsheet_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    let sheets = spreadsheet["worksheets"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(index, worksheet)| {
            json!({
                "properties": {
                    "sheetId": index,
                    "title": worksheet["title"],
                    "index": index,
                    "sheetType": "GRID",
                }
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(json!({
        "spreadsheetId": spreadsheet_id,
        "properties": {"title": spreadsheet["title"]},
        "sheets": sheets,
        "spreadsheetUrl": format!("https://docs.google.com/spreadsheets/d/{spreadsheet_id}"),
    })))
}

async fn get_values(
    State(world): State<Arc<World>>,
    Path((spreadsheet_id, range)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    let state = world.read().await;
    let spreadsheet = state
        .google_sheets
        .spreadsheets
        .get(&spreadsheet_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    let requested_sheet = range.split('!').next().unwrap_or(&range).trim_matches('\'');
    let worksheet = spreadsheet["worksheets"]
        .as_array()
        .and_then(|worksheets| {
            worksheets.iter().find(|worksheet| {
                worksheet["title"] == requested_sheet || worksheet["id"] == requested_sheet
            })
        })
        .ok_or(StatusCode::NOT_FOUND)?;
    let rows = worksheet["rows"]
        .as_array()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut columns = BTreeSet::new();
    for row in rows {
        if let Some(cells) = row["cells"].as_object() {
            columns.extend(cells.keys().cloned());
        }
    }
    let mut columns = columns.into_iter().collect::<Vec<_>>();
    columns.sort_by_key(|column| match column.as_str() {
        "Section" => 0,
        "Requirement" => 1,
        _ => 2,
    });
    let mut values = vec![columns
        .iter()
        .cloned()
        .map(Value::String)
        .collect::<Vec<_>>()];
    values.extend(rows.iter().map(|row| {
        columns
            .iter()
            .map(|column| row["cells"].get(column).cloned().unwrap_or(Value::Null))
            .collect::<Vec<_>>()
    }));

    Ok(Json(json!({
        "range": range,
        "majorDimension": "ROWS",
        "values": values,
    })))
}
