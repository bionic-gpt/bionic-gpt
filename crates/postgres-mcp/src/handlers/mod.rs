mod analysis;
mod monitoring;
mod schemas;
mod sql;

use axum::{routing::get, routing::post, Router};

use crate::state::AppState;

pub fn v1_router() -> Router<AppState> {
    Router::new()
        .route("/schemas", get(schemas::list_schemas))
        .route("/schemas/:schema/objects", get(schemas::list_objects))
        .route(
            "/schemas/:schema/objects/:object",
            get(schemas::get_object_details),
        )
        .route("/sql/execute", post(sql::execute_sql))
        .route("/sql/explain", post(sql::explain_query))
        .route("/monitoring/top-queries", get(monitoring::top_queries))
        .route(
            "/analysis/workload/indexes",
            post(analysis::analyze_workload_indexes),
        )
        .route(
            "/analysis/query/indexes",
            post(analysis::analyze_query_indexes),
        )
        .route("/analysis/db-health", get(analysis::db_health))
}
