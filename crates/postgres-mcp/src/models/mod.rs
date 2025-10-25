use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{collections::HashMap, str::FromStr};
use time::OffsetDateTime;

pub type SqlParameters = IndexMap<String, serde_json::Value>;
pub type SqlResultRow = Vec<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSchemasResponse {
    pub schemas: Vec<SchemaSummary>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaSummary {
    pub name: String,
    pub owner: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    Table,
    View,
    MaterializedView,
    Function,
    Procedure,
    Index,
    Sequence,
}

impl FromStr for ObjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "table" => Ok(Self::Table),
            "view" => Ok(Self::View),
            "materialized_view" => Ok(Self::MaterializedView),
            "function" => Ok(Self::Function),
            "procedure" => Ok(Self::Procedure),
            "index" => Ok(Self::Index),
            "sequence" => Ok(Self::Sequence),
            other => Err(format!("unsupported object type `{other}`")),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectSummary {
    pub schema: String,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: ObjectType,
    pub owner: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListObjectsResponse {
    pub objects: Vec<ObjectSummary>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDetail {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub default: Option<String>,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDetail {
    pub name: String,
    pub definition: String,
    pub is_unique: bool,
    pub columns: Option<Vec<String>>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectDependency {
    pub schema: String,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: ObjectType,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseObject {
    pub schema: String,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: ObjectType,
    pub owner: Option<String>,
    pub comment: Option<String>,
    pub definition: Option<String>,
    pub columns: Option<Vec<ColumnDetail>>,
    pub indexes: Option<Vec<IndexDetail>>,
    pub dependencies: Option<Vec<ObjectDependency>>,
    pub stats: Option<ObjectStatistics>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ObjectStatistics {
    pub last_analyzed_at: Option<OffsetDateTime>,
    pub rows: Option<i64>,
    pub rel_pages: Option<i64>,
    pub toast_rows: Option<i64>,
    pub toast_pages: Option<i64>,
    pub sequential_scans: Option<i64>,
    pub index_scans: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetObjectDetailsResponse {
    pub object: DatabaseObject,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteSqlRequest {
    pub statement: String,
    #[serde(default)]
    pub parameters: SqlParameters,
    #[serde(default = "default_max_rows")]
    pub max_rows: i32,
}

fn default_max_rows() -> i32 {
    500
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlResultColumn {
    pub name: String,
    pub data_type: String,
    pub table: Option<String>,
    pub nullable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteSqlResponse {
    pub columns: Vec<SqlResultColumn>,
    pub rows: Vec<SqlResultRow>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainQueryRequest {
    pub query: String,
    #[serde(default)]
    pub parameters: SqlParameters,
    #[serde(default)]
    pub analyze: bool,
    #[serde(default)]
    pub verbose: bool,
    #[serde(default = "default_true")]
    pub costs: bool,
    #[serde(default)]
    pub buffers: bool,
    #[serde(default)]
    pub settings: bool,
    #[serde(default = "default_explain_format")]
    pub format: ExplainFormat,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainFormat {
    Text,
    Json,
    Yaml,
    Xml,
}

fn default_explain_format() -> ExplainFormat {
    ExplainFormat::Json
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExplainPlan {
    Text(String),
    Map(HashMap<String, serde_json::Value>),
    Sequence(Vec<serde_json::Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainQueryResponse {
    pub format: ExplainFormat,
    pub plan: ExplainPlan,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopQuery {
    pub query: String,
    pub calls: i64,
    pub total_time_ms: f64,
    pub mean_time_ms: f64,
    pub rows: Option<i64>,
    pub shared_blks_hit: Option<i64>,
    pub shared_blks_read: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTopQueriesResponse {
    pub queries: Vec<TopQuery>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadQuery {
    pub query: String,
    #[serde(default)]
    pub parameters: SqlParameters,
    #[serde(default = "default_frequency")]
    pub frequency: i64,
}

fn default_frequency() -> i64 {
    1
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeWorkloadIndexesRequest {
    pub workload: Vec<WorkloadQuery>,
    #[serde(default = "default_max_recommendations")]
    pub max_recommendations: i32,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeQueryIndexesRequest {
    pub query: String,
    #[serde(default)]
    pub parameters: SqlParameters,
    #[serde(default = "default_max_recommendations")]
    pub max_recommendations: i32,
}

fn default_max_recommendations() -> i32 {
    5
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexRecommendation {
    pub statement: String,
    pub reason: String,
    pub estimated_improvement_percent: Option<f64>,
    #[serde(default)]
    pub confidence: Option<RecommendationConfidence>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationConfidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeIndexesResponse {
    pub recommendations: Vec<IndexRecommendation>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HealthCheckStatus {
    Pass,
    Warn,
    Fail,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub check: String,
    pub status: HealthCheckStatus,
    pub message: Option<String>,
    pub recommendations: Option<Vec<String>>,
    pub details: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeDbHealthResponse {
    pub checks: Vec<HealthCheckResult>,
}
