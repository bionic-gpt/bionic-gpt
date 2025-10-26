use axum::{extract::State, Json};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use tokio_postgres::Column;

use crate::{
    auth::ConnectionString,
    db,
    error::ApiResult,
    models::{
        ExecuteSqlRequest, ExecuteSqlResponse, ExplainFormat, ExplainPlan, ExplainQueryRequest,
        ExplainQueryResponse, SqlResultColumn,
    },
    sql_utils::{build_parameter_values, rewrite_named_parameters, row_to_json},
    state::AppState,
};

const MAX_RESULT_ROWS: i32 = 10_000;

pub async fn execute_sql(
    State(_state): State<AppState>,
    ConnectionString(conn): ConnectionString,
    Json(request): Json<ExecuteSqlRequest>,
) -> ApiResult<Json<ExecuteSqlResponse>> {
    let client = db::connect(&conn).await?;
    let rewritten = rewrite_named_parameters(&request.statement)?;
    let statement = client.prepare(&rewritten.sql).await?;
    let params =
        build_parameter_values(&statement, &rewritten.parameter_names, &request.parameters)?;
    let param_refs = params.as_refs();

    let max_rows = request.max_rows.clamp(1, MAX_RESULT_ROWS);
    let mut rows = client.query(&statement, &param_refs).await?;
    if rows.len() as i32 > max_rows {
        rows.truncate(max_rows as usize);
    }

    let columns = build_column_descriptors(&client, statement.columns()).await?;
    let mut result_rows = Vec::with_capacity(rows.len());
    for row in rows {
        result_rows.push(row_to_json(&row)?);
    }

    Ok(Json(ExecuteSqlResponse {
        columns,
        rows: result_rows,
    }))
}

pub async fn explain_query(
    State(_state): State<AppState>,
    ConnectionString(conn): ConnectionString,
    Json(request): Json<ExplainQueryRequest>,
) -> ApiResult<Json<ExplainQueryResponse>> {
    let client = db::connect(&conn).await?;
    let rewritten = rewrite_named_parameters(&request.query)?;
    let explain_sql = format!(
        "EXPLAIN ({}) {}",
        build_explain_options(&request),
        rewritten.sql
    );

    let statement = client.prepare(&explain_sql).await?;
    let params =
        build_parameter_values(&statement, &rewritten.parameter_names, &request.parameters)?;
    let param_refs = params.as_refs();

    let rows = client.query(&statement, &param_refs).await?;
    let plan = format_explain_plan(request.format.clone(), rows)?;

    Ok(Json(ExplainQueryResponse {
        format: request.format,
        plan,
    }))
}

fn build_explain_options(request: &ExplainQueryRequest) -> String {
    let mut options = Vec::new();
    let format = match request.format {
        ExplainFormat::Text => "TEXT",
        ExplainFormat::Json => "JSON",
        ExplainFormat::Yaml => "YAML",
        ExplainFormat::Xml => "XML",
    };
    options.push(format!("FORMAT {format}"));
    options.push(format!("ANALYZE {}", request.analyze));
    options.push(format!("VERBOSE {}", request.verbose));
    options.push(format!("COSTS {}", request.costs));
    options.push(format!("BUFFERS {}", request.buffers));
    options.push(format!("SETTINGS {}", request.settings));

    options.join(", ")
}

fn format_explain_plan(
    format: ExplainFormat,
    rows: Vec<tokio_postgres::Row>,
) -> ApiResult<ExplainPlan> {
    match format {
        ExplainFormat::Json => {
            let mut plans = Vec::with_capacity(rows.len());
            for row in rows {
                let value: Value = row.get(0);
                plans.push(value);
            }

            if plans.len() == 1 {
                let value = plans.into_iter().next().unwrap();
                match value {
                    Value::Array(items) => Ok(ExplainPlan::Sequence(items)),
                    Value::Object(map) => Ok(ExplainPlan::Map(map.into_iter().collect())),
                    other => Ok(ExplainPlan::Sequence(vec![other])),
                }
            } else {
                Ok(ExplainPlan::Sequence(plans))
            }
        }
        _ => {
            let mut lines = Vec::with_capacity(rows.len());
            for row in rows {
                let line: String = row.get(0);
                lines.push(line);
            }
            Ok(ExplainPlan::Text(lines.join("\n")))
        }
    }
}

async fn build_column_descriptors(
    client: &tokio_postgres::Client,
    columns: &[Column],
) -> ApiResult<Vec<SqlResultColumn>> {
    if columns.is_empty() {
        return Ok(Vec::new());
    }

    let mut table_oids: HashSet<u32> = HashSet::new();
    for column in columns {
        if let Some(oid) = column.table_oid() {
            table_oids.insert(oid);
        }
    }

    let table_meta = load_table_metadata(client, table_oids).await?;

    let mut result = Vec::with_capacity(columns.len());
    for column in columns {
        let table_oid = column.table_oid();
        let column_id = column.column_id();

        let (table_name, nullable) =
            if let (Some(table_oid), Some(column_id)) = (table_oid, column_id) {
                if let Some(info) = table_meta.get(&table_oid) {
                    let table = Some(info.qualified_name.clone());
                    let nullable = info.nullable_by_attnum.get(&column_id).copied();
                    (table, nullable)
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

        result.push(SqlResultColumn {
            name: column.name().into(),
            data_type: column.type_().name().into(),
            table: table_name,
            nullable,
        });
    }

    Ok(result)
}

struct TableMetadata {
    qualified_name: String,
    nullable_by_attnum: HashMap<i16, bool>,
}

async fn load_table_metadata(
    client: &tokio_postgres::Client,
    table_oids: HashSet<u32>,
) -> ApiResult<HashMap<u32, TableMetadata>> {
    if table_oids.is_empty() {
        return Ok(HashMap::new());
    }

    let ids: Vec<u32> = table_oids.into_iter().collect();

    let rows = client
        .query(
            "SELECT c.oid, n.nspname, c.relname
             FROM pg_class c
             JOIN pg_namespace n ON n.oid = c.relnamespace
             WHERE c.oid = ANY($1::oid[])",
            &[&ids],
        )
        .await?;

    let mut metadata = HashMap::new();
    for row in rows {
        let oid: u32 = row.get("oid");
        let schema: String = row.get("nspname");
        let table: String = row.get("relname");
        let qualified = format!("\"{schema}\".\"{table}\"");
        metadata.insert(
            oid,
            TableMetadata {
                qualified_name: qualified,
                nullable_by_attnum: HashMap::new(),
            },
        );
    }

    let rows = client
        .query(
            "SELECT attrelid, attnum, NOT attnotnull AS nullable
             FROM pg_attribute
             WHERE attrelid = ANY($1::oid[])
               AND attnum > 0
               AND NOT attisdropped",
            &[&ids],
        )
        .await?;

    for row in rows {
        let oid: u32 = row.get("attrelid");
        let attnum: i16 = row.get("attnum");
        let nullable: bool = row.get("nullable");
        if let Some(info) = metadata.get_mut(&oid) {
            info.nullable_by_attnum.insert(attnum, nullable);
        }
    }

    Ok(metadata)
}
