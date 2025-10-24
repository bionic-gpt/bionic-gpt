use axum::{
    extract::{Path, Query},
    Json,
};
use serde::Deserialize;
use std::collections::HashSet;

use crate::{
    auth::ConnectionString,
    db,
    error::{ApiError, ApiResult},
    extractors::parameters::deserialize_comma_separated,
    models::{
        GetObjectDetailsResponse, ListObjectsResponse, ListSchemasResponse, ObjectSummary,
        ObjectType, SchemaSummary,
    },
};

#[derive(Debug, Deserialize)]
pub struct ListSchemasQuery {
    #[serde(default)]
    pub include_system_schemas: bool,
}

pub async fn list_schemas(
    ConnectionString(conn): ConnectionString,
    Query(query): Query<ListSchemasQuery>,
) -> ApiResult<Json<ListSchemasResponse>> {
    let include_system = query.include_system_schemas;
    let client = db::connect(&conn).await?;

    let rows = client
        .query(
            r#"
            SELECT
                n.nspname AS schema_name,
                r.rolname AS owner,
                obj_description(n.oid, 'pg_namespace') AS comment
            FROM pg_namespace n
            LEFT JOIN pg_roles r ON r.oid = n.nspowner
            WHERE has_schema_privilege(n.oid, 'USAGE')
              AND (
                $1
                OR (
                    n.nspname NOT IN ('pg_catalog', 'information_schema')
                    AND n.nspname NOT LIKE 'pg_toast%'
                    AND n.nspname NOT LIKE 'pg_temp_%'
                )
              )
            ORDER BY n.nspname
            "#,
            &[&include_system],
        )
        .await?;

    let schemas = rows
        .into_iter()
        .map(|row| SchemaSummary {
            name: row.get::<_, String>("schema_name"),
            owner: row.get::<_, Option<String>>("owner"),
            comment: row.get::<_, Option<String>>("comment"),
        })
        .collect();

    Ok(Json(ListSchemasResponse { schemas }))
}

#[derive(Debug, Deserialize)]
pub struct ListObjectsPath {
    pub schema: String,
}

#[derive(Debug, Deserialize)]
pub struct ListObjectsQuery {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub types: Vec<ObjectType>,
}

pub async fn list_objects(
    ConnectionString(conn): ConnectionString,
    Path(path): Path<ListObjectsPath>,
    Query(query): Query<ListObjectsQuery>,
) -> ApiResult<Json<ListObjectsResponse>> {
    let client = db::connect(&conn).await?;
    let mut summaries = Vec::new();

    let requested: HashSet<ObjectType> = query.types.iter().copied().collect();
    let include_all = requested.is_empty();

    let relations = [
        (ObjectType::Table, "r"),
        (ObjectType::View, "v"),
        (ObjectType::MaterializedView, "m"),
        (ObjectType::Index, "I"),
        (ObjectType::Sequence, "S"),
    ];

    for (object_type, relkind) in relations {
        if include_all || requested.contains(&object_type) {
            let rows = client
                .query(
                    r#"
                    SELECT
                        c.relname AS name,
                        r.rolname AS owner,
                        obj_description(c.oid) AS comment
                    FROM pg_class c
                    JOIN pg_namespace n ON n.oid = c.relnamespace
                    LEFT JOIN pg_roles r ON r.oid = c.relowner
                    WHERE n.nspname = $1
                      AND c.relkind::text = $2
                    ORDER BY c.relname
                    "#,
                    &[&path.schema, &relkind],
                )
                .await?;

            summaries.extend(rows.into_iter().map(|row| ObjectSummary {
                schema: path.schema.clone(),
                name: row.get::<_, String>("name"),
                object_type,
                owner: row.get::<_, Option<String>>("owner"),
                comment: row.get::<_, Option<String>>("comment"),
            }));
        }
    }

    let routines = [(ObjectType::Function, "f"), (ObjectType::Procedure, "p")];

    for (object_type, prokind) in routines {
        if include_all || requested.contains(&object_type) {
            let rows = client
                .query(
                    r#"
                    SELECT
                        p.proname AS name,
                        pg_get_function_identity_arguments(p.oid) AS arguments,
                        r.rolname AS owner,
                        d.description AS comment
                    FROM pg_proc p
                    JOIN pg_namespace n ON n.oid = p.pronamespace
                    LEFT JOIN pg_roles r ON r.oid = p.proowner
                    LEFT JOIN pg_description d ON d.objoid = p.oid AND d.classoid = 'pg_proc'::regclass
                    WHERE n.nspname = $1
                      AND p.prokind::text = $2
                    ORDER BY p.proname
                    "#,
                    &[&path.schema, &prokind],
                )
                .await?;

            summaries.extend(rows.into_iter().map(|row| {
                let name: String = row.get("name");
                let arguments: String = row.get("arguments");
                let display_name = if arguments.is_empty() {
                    name
                } else {
                    format!("{name}({arguments})")
                };

                ObjectSummary {
                    schema: path.schema.clone(),
                    name: display_name,
                    object_type,
                    owner: row.get::<_, Option<String>>("owner"),
                    comment: row.get::<_, Option<String>>("comment"),
                }
            }));
        }
    }

    summaries.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(Json(ListObjectsResponse { objects: summaries }))
}

#[derive(Debug, Deserialize)]
pub struct GetObjectPath {
    pub schema: String,
    pub object: String,
}

#[derive(Debug, Deserialize)]
pub struct GetObjectQuery {
    #[serde(default)]
    pub include_stats: bool,
}

pub async fn get_object_details(
    ConnectionString(conn): ConnectionString,
    Path(path): Path<GetObjectPath>,
    Query(query): Query<GetObjectQuery>,
) -> ApiResult<Json<GetObjectDetailsResponse>> {
    let _ = (conn, &path.schema, &path.object, query.include_stats);
    Err(ApiError::internal("not implemented"))
}
