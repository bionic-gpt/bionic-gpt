use axum::{
    extract::{Query, State},
    Json,
};
use indexmap::IndexMap;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use tokio_postgres::Client;

use crate::{
    auth::ConnectionString,
    db,
    error::ApiResult,
    models::{
        AnalyzeDbHealthResponse, AnalyzeIndexesResponse, AnalyzeQueryIndexesRequest,
        AnalyzeWorkloadIndexesRequest, HealthCheckResult, HealthCheckStatus, IndexRecommendation,
        RecommendationConfidence, WorkloadQuery,
    },
    sql_utils::{build_parameter_values, extract_filter_columns, rewrite_named_parameters},
    state::AppState,
};

const MAX_INDEX_RECOMMENDATIONS: i32 = 25;

pub async fn analyze_workload_indexes(
    State(_state): State<AppState>,
    ConnectionString(conn): ConnectionString,
    Json(request): Json<AnalyzeWorkloadIndexesRequest>,
) -> ApiResult<Json<AnalyzeIndexesResponse>> {
    if request.workload.is_empty() {
        return Ok(Json(AnalyzeIndexesResponse {
            recommendations: Vec::new(),
        }));
    }

    let client = db::connect(&conn).await?;
    let max = request
        .max_recommendations
        .clamp(1, MAX_INDEX_RECOMMENDATIONS) as usize;

    let mut aggregated: IndexMap<String, AggregatedRecommendation> = IndexMap::new();

    for query in &request.workload {
        let mut recs = analyze_query_for_indexes(&client, query).await?;
        for mut rec in recs.drain(..) {
            let frequency = query.frequency.max(1);
            let entry = aggregated.entry(rec.statement.clone()).or_insert_with(|| {
                AggregatedRecommendation {
                    recommendation: rec.clone(),
                    total_frequency: 0,
                }
            });
            entry.total_frequency += frequency;
            if query.frequency > 1 {
                rec.reason = format!(
                    "{} (workload frequency hint: {})",
                    rec.reason, query.frequency
                );
                entry.recommendation = rec;
            }
        }
    }

    let mut ranked: Vec<_> = aggregated.into_values().collect();
    ranked.sort_by(|a, b| b.total_frequency.cmp(&a.total_frequency));

    let mut recommendations = Vec::new();
    for mut item in ranked.into_iter().take(max) {
        if item.total_frequency > 1 {
            item.recommendation.reason = format!(
                "{} (aggregated frequency score: {})",
                item.recommendation.reason, item.total_frequency
            );
        }
        recommendations.push(item.recommendation);
    }

    Ok(Json(AnalyzeIndexesResponse { recommendations }))
}

pub async fn analyze_query_indexes(
    State(_state): State<AppState>,
    ConnectionString(conn): ConnectionString,
    Json(request): Json<AnalyzeQueryIndexesRequest>,
) -> ApiResult<Json<AnalyzeIndexesResponse>> {
    let client = db::connect(&conn).await?;
    let max = request
        .max_recommendations
        .clamp(1, MAX_INDEX_RECOMMENDATIONS) as usize;

    let query = WorkloadQuery {
        query: request.query,
        parameters: request.parameters,
        frequency: 1,
    };

    let mut recommendations = analyze_query_for_indexes(&client, &query).await?;
    recommendations.truncate(max);

    Ok(Json(AnalyzeIndexesResponse { recommendations }))
}

pub async fn db_health(
    State(_state): State<AppState>,
    ConnectionString(conn): ConnectionString,
    Query(params): Query<DbHealthParams>,
) -> ApiResult<Json<AnalyzeDbHealthResponse>> {
    let client = db::connect(&conn).await?;

    let mut checks = Vec::new();
    checks.push(connection_health(&client, params.include_diagnostics).await?);
    checks.push(autovacuum_health(&client, params.include_diagnostics).await?);
    checks.push(deadlock_health(&client, params.include_diagnostics).await?);
    checks.push(long_running_queries_health(&client, params.include_diagnostics).await?);

    Ok(Json(AnalyzeDbHealthResponse { checks }))
}

#[derive(Debug, serde::Deserialize)]
pub struct DbHealthParams {
    #[serde(default)]
    pub include_diagnostics: bool,
}

#[derive(Clone)]
struct AggregatedRecommendation {
    recommendation: IndexRecommendation,
    total_frequency: i64,
}

async fn analyze_query_for_indexes(
    client: &Client,
    workload_query: &WorkloadQuery,
) -> ApiResult<Vec<IndexRecommendation>> {
    let candidates =
        find_seq_scan_candidates(client, &workload_query.query, &workload_query.parameters).await?;

    let mut seen = HashSet::new();
    let mut ranked = Vec::new();
    for candidate in candidates {
        if candidate.columns.is_empty() {
            continue;
        }

        let statement = build_index_statement(&candidate);
        if !seen.insert(statement.clone()) {
            continue;
        }

        let reason = if let Some(filter) = &candidate.filter {
            format!(
                "Sequential scan on {}.{} with filter `{}`",
                candidate.schema, candidate.relation, filter
            )
        } else {
            format!(
                "Sequential scan on {}.{} without index coverage",
                candidate.schema, candidate.relation
            )
        };

        let recommendation = IndexRecommendation {
            statement,
            reason,
            estimated_improvement_percent: None,
            confidence: Some(RecommendationConfidence::Medium),
        };

        ranked.push((candidate.total_cost.unwrap_or(0.0), recommendation));
    }

    ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    Ok(ranked.into_iter().map(|(_, rec)| rec).collect())
}

async fn find_seq_scan_candidates(
    client: &Client,
    sql: &str,
    parameters: &crate::models::SqlParameters,
) -> ApiResult<Vec<SeqScanCandidate>> {
    let rewritten = rewrite_named_parameters(sql)?;
    let explain_sql = format!("EXPLAIN (FORMAT JSON, COSTS true) {}", rewritten.sql);
    let statement = client.prepare(&explain_sql).await?;
    let params = build_parameter_values(&statement, &rewritten.parameter_names, parameters)?;
    let param_refs = params.as_refs();

    let rows = client.query(&statement, &param_refs).await?;
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let value: serde_json::Value = rows[0].get(0);
    let mut candidates = Vec::new();

    if let Some(array) = value.as_array() {
        for item in array {
            if let Some(plan) = item.get("Plan") {
                collect_seq_scans(plan, &mut candidates);
            }
        }
    }

    candidates.sort_by(|a, b| {
        b.total_cost
            .partial_cmp(&a.total_cost)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(candidates)
}

fn collect_seq_scans(node: &serde_json::Value, candidates: &mut Vec<SeqScanCandidate>) {
    if let Some(obj) = node.as_object() {
        if let Some(node_type) = obj.get("Node Type").and_then(|v| v.as_str()) {
            if node_type == "Seq Scan" {
                let relation = obj
                    .get("Relation Name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let schema = obj
                    .get("Schema")
                    .and_then(|v| v.as_str())
                    .unwrap_or("public")
                    .to_string();
                let filter = obj
                    .get("Filter")
                    .and_then(|v| v.as_str())
                    .map(|v| v.to_string());
                let mut columns = filter
                    .as_deref()
                    .map(extract_filter_columns)
                    .unwrap_or_default();

                if columns.len() > 4 {
                    columns.truncate(4);
                }

                let cost = obj
                    .get("Total Cost")
                    .and_then(|v| v.as_f64())
                    .or_else(|| obj.get("Plan Rows").and_then(|v| v.as_f64()));

                if !relation.is_empty() && !columns.is_empty() {
                    candidates.push(SeqScanCandidate {
                        schema,
                        relation,
                        filter,
                        columns,
                        total_cost: cost,
                    });
                }
            }
        }

        if let Some(children) = obj.get("Plans").and_then(|v| v.as_array()) {
            for child in children {
                collect_seq_scans(child, candidates);
            }
        }
    }
}

fn build_index_statement(candidate: &SeqScanCandidate) -> String {
    let schema = quote_ident(&candidate.schema);
    let relation = quote_ident(&candidate.relation);

    let mut index_name = format!(
        "idx_{}_{}",
        candidate
            .relation
            .replace(|c: char| !c.is_ascii_alphanumeric(), "_"),
        candidate
            .columns
            .iter()
            .map(|c| c.replace(|ch: char| !ch.is_ascii_alphanumeric(), "_"))
            .collect::<Vec<_>>()
            .join("_")
    )
    .to_lowercase();

    if index_name.len() > 60 {
        index_name.truncate(60);
    }

    if index_name.is_empty() {
        index_name = "idx_recommendation".into();
    }

    let columns = candidate
        .columns
        .iter()
        .map(|column| quote_ident(column))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "CREATE INDEX IF NOT EXISTS {} ON {}.{} ({})",
        quote_ident(&index_name),
        schema,
        relation,
        columns
    )
}

fn quote_ident(input: &str) -> String {
    format!("\"{}\"", input.replace('"', "\"\""))
}

struct SeqScanCandidate {
    schema: String,
    relation: String,
    filter: Option<String>,
    columns: Vec<String>,
    total_cost: Option<f64>,
}

fn to_detail_map(value: serde_json::Value) -> Option<HashMap<String, serde_json::Value>> {
    value
        .as_object()
        .map(|map| map.clone().into_iter().collect())
}

async fn connection_health(
    client: &Client,
    include_diagnostics: bool,
) -> ApiResult<HealthCheckResult> {
    let max_connections: i64 = client
        .query_one(
            "SELECT setting::bigint FROM pg_settings WHERE name = 'max_connections'",
            &[],
        )
        .await?
        .get(0);

    let active_connections: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM pg_stat_activity WHERE state <> 'idle'",
            &[],
        )
        .await?
        .get(0);

    let ratio = if max_connections > 0 {
        active_connections as f64 / max_connections as f64
    } else {
        0.0
    };

    let (status, recommendations) = if ratio < 0.7 {
        (HealthCheckStatus::Pass, None)
    } else if ratio < 0.9 {
        (
            HealthCheckStatus::Warn,
            Some(vec![
                "Consider reducing idle connections or increasing `max_connections`.".to_string(),
            ]),
        )
    } else {
        (
            HealthCheckStatus::Fail,
            Some(vec![
                "Connection usage is saturated; scale application connections or increase `max_connections`."
                    .to_string(),
            ]),
        )
    };

    let message = format!(
        "{} of {} connections currently active ({:.0}%)",
        active_connections,
        max_connections,
        ratio * 100.0
    );

    let details = if include_diagnostics {
        to_detail_map(json!({
            "active_connections": active_connections,
            "max_connections": max_connections,
            "utilization_ratio": ratio
        }))
    } else {
        None
    };

    Ok(HealthCheckResult {
        check: "Connection saturation".into(),
        status,
        message: Some(message),
        recommendations,
        details,
    })
}

async fn autovacuum_health(
    client: &Client,
    include_diagnostics: bool,
) -> ApiResult<HealthCheckResult> {
    let autovacuum_enabled: bool = client
        .query_one(
            "SELECT setting = 'on' FROM pg_settings WHERE name = 'autovacuum'",
            &[],
        )
        .await?
        .get(0);

    let (status, recommendations, message) = if autovacuum_enabled {
        (
            HealthCheckStatus::Pass,
            None,
            "Autovacuum is enabled.".to_string(),
        )
    } else {
        (
            HealthCheckStatus::Fail,
            Some(vec![
                "Enable autovacuum to keep table statistics up to date automatically.".to_string(),
            ]),
            "Autovacuum is disabled.".to_string(),
        )
    };

    let details = if include_diagnostics {
        to_detail_map(json!({ "autovacuum_enabled": autovacuum_enabled }))
    } else {
        None
    };

    Ok(HealthCheckResult {
        check: "Autovacuum configuration".into(),
        status,
        message: Some(message),
        recommendations,
        details,
    })
}

async fn deadlock_health(
    client: &Client,
    include_diagnostics: bool,
) -> ApiResult<HealthCheckResult> {
    let total_deadlocks: i64 = client
        .query_one(
            "SELECT COALESCE(SUM(deadlocks), 0) FROM pg_stat_database",
            &[],
        )
        .await?
        .get(0);

    let (status, recommendations) = if total_deadlocks == 0 {
        (HealthCheckStatus::Pass, None)
    } else if total_deadlocks < 3 {
        (
            HealthCheckStatus::Warn,
            Some(vec![
                "Investigate the workload for conflicting transactions causing deadlocks."
                    .to_string(),
            ]),
        )
    } else {
        (
            HealthCheckStatus::Fail,
            Some(vec![
                "High deadlock count detected; review transaction isolation and explicit locking."
                    .to_string(),
            ]),
        )
    };

    let message = if total_deadlocks == 0 {
        "No deadlocks recorded since last statistics reset.".to_string()
    } else {
        format!("{total_deadlocks} deadlock(s) recorded since last statistics reset.")
    };

    let details = if include_diagnostics {
        to_detail_map(json!({ "deadlocks": total_deadlocks }))
    } else {
        None
    };

    Ok(HealthCheckResult {
        check: "Deadlock history".into(),
        status,
        message: Some(message),
        recommendations,
        details,
    })
}

async fn long_running_queries_health(
    client: &Client,
    include_diagnostics: bool,
) -> ApiResult<HealthCheckResult> {
    let rows = client
        .query(
            "SELECT pid,
                    datname,
                    EXTRACT(EPOCH FROM (now() - query_start))::bigint AS duration_seconds,
                    LEFT(query, 200) AS sample
             FROM pg_stat_activity
             WHERE state <> 'idle'
               AND query_start IS NOT NULL
               AND now() - query_start > interval '5 minutes'
             ORDER BY duration_seconds DESC
             LIMIT 5",
            &[],
        )
        .await?;

    let count = rows.len() as i64;

    let (status, recommendations) = if count == 0 {
        (HealthCheckStatus::Pass, None)
    } else if count <= 2 {
        (
            HealthCheckStatus::Warn,
            Some(vec![
                "Investigate long-running queries; consider adding indexes or breaking work into smaller batches."
                    .to_string(),
            ]),
        )
    } else {
        (
            HealthCheckStatus::Fail,
            Some(vec![
                "Multiple long-running queries detected; examine blocking or missing indexes."
                    .to_string(),
            ]),
        )
    };

    let message = if count == 0 {
        "No queries running longer than 5 minutes.".to_string()
    } else {
        format!("{count} long running querie(s) detected (>{} minutes).", 5)
    };

    let details = if include_diagnostics {
        let samples = rows
            .into_iter()
            .map(|row| {
                json!({
                    "pid": row.get::<_, i32>("pid"),
                    "database": row.get::<_, String>("datname"),
                    "duration_seconds": row.get::<_, i64>("duration_seconds"),
                    "query": row.get::<_, String>("sample"),
                })
            })
            .collect::<Vec<_>>();
        to_detail_map(json!({ "samples": samples }))
    } else {
        None
    };

    Ok(HealthCheckResult {
        check: "Long running queries".into(),
        status,
        message: Some(message),
        recommendations,
        details,
    })
}
