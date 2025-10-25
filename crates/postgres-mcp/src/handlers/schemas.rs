use axum::{
    extract::{Path, Query},
    Json,
};
use serde::Deserialize;
use std::collections::HashSet;
use tokio_postgres::Client;

use crate::{
    auth::ConnectionString,
    db,
    error::{ApiError, ApiResult},
    extractors::parameters::deserialize_comma_separated,
    models::{
        ColumnDetail, DatabaseObject, GetObjectDetailsResponse, IndexDetail, ListObjectsResponse,
        ListSchemasResponse, ObjectStatistics, ObjectSummary, ObjectType, SchemaSummary,
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

async fn fetch_relation_details(
    client: &Client,
    schema: &str,
    name: &str,
    include_stats: bool,
) -> ApiResult<Option<DatabaseObject>> {
    let row = match client
        .query_opt(
            "SELECT c.oid,
                    c.relkind::text AS relkind,
                    c.relname,
                    r.rolname AS owner,
                    obj_description(c.oid, 'pg_class') AS comment
             FROM pg_class c
             JOIN pg_namespace n ON n.oid = c.relnamespace
             LEFT JOIN pg_roles r ON r.oid = c.relowner
             WHERE n.nspname = $1
               AND c.relname = $2",
            &[&schema, &name],
        )
        .await?
    {
        Some(row) => row,
        None => return Ok(None),
    };

    let oid: u32 = row.get("oid");
    let relkind: String = row.get("relkind");
    let object_type = match relkind_to_object_type(&relkind) {
        Some(value) => value,
        None => return Ok(None),
    };

    let owner: Option<String> = row.get("owner");
    let comment: Option<String> = row.get("comment");
    let definition = fetch_object_definition(client, oid, &object_type).await?;

    let columns = match object_type {
        ObjectType::Table | ObjectType::View | ObjectType::MaterializedView => {
            Some(fetch_columns(client, oid).await?)
        }
        _ => None,
    };

    let indexes = match object_type {
        ObjectType::Table | ObjectType::MaterializedView => Some(fetch_indexes(client, oid).await?),
        _ => None,
    };

    let stats = if matches!(
        object_type,
        ObjectType::Table | ObjectType::MaterializedView
    ) && include_stats
    {
        fetch_object_stats(client, oid).await?
    } else {
        None
    };

    Ok(Some(DatabaseObject {
        schema: schema.to_string(),
        name: name.to_string(),
        object_type,
        owner,
        comment,
        definition,
        columns,
        indexes,
        dependencies: None,
        stats,
    }))
}

async fn fetch_routine_details(
    client: &Client,
    schema: &str,
    display_name: &str,
) -> ApiResult<Option<DatabaseObject>> {
    let (base_name, signature) = parse_routine_identifier(display_name);

    let rows = client
        .query(
            "SELECT p.oid,
                    p.prokind,
                    pg_get_function_identity_arguments(p.oid) AS identity_arguments,
                    pg_get_functiondef(p.oid) AS definition,
                    r.rolname AS owner,
                    obj_description(p.oid, 'pg_proc') AS comment
             FROM pg_proc p
             JOIN pg_namespace n ON n.oid = p.pronamespace
             LEFT JOIN pg_roles r ON r.oid = p.proowner
             WHERE n.nspname = $1
               AND p.proname = $2",
            &[&schema, &base_name],
        )
        .await?;

    if rows.is_empty() {
        return Ok(None);
    }

    let mut selected: Option<(tokio_postgres::Row, String)> = None;

    for row in rows {
        let identity: String = row.get("identity_arguments");
        if let Some(sig) = &signature {
            if identity == *sig {
                selected = Some((row, identity));
                break;
            }
        } else if identity.is_empty() {
            selected = Some((row, identity));
            break;
        } else if selected.is_none() {
            selected = Some((row, identity));
        }
    }

    let (row, identity_arguments) = match selected {
        Some(result) => result,
        None => return Ok(None),
    };

    let prokind: String = row.get("prokind");
    let object_type = match prokind.as_str() {
        "f" => ObjectType::Function,
        "p" => ObjectType::Procedure,
        _ => return Ok(None),
    };

    let owner: Option<String> = row.get("owner");
    let comment: Option<String> = row.get("comment");
    let definition: String = row.get("definition");

    let name = if identity_arguments.is_empty() {
        base_name
    } else {
        format!("{base_name}({identity_arguments})")
    };

    Ok(Some(DatabaseObject {
        schema: schema.to_string(),
        name,
        object_type,
        owner,
        comment,
        definition: Some(definition),
        columns: None,
        indexes: None,
        dependencies: None,
        stats: None,
    }))
}

fn parse_routine_identifier(object: &str) -> (String, Option<String>) {
    if let Some(idx) = object.find('(') {
        if object.ends_with(')') && idx < object.len() - 1 {
            let name = object[..idx].to_string();
            let signature = object[idx + 1..object.len() - 1].to_string();
            return (name, Some(signature));
        }
    }

    (object.to_string(), None)
}

fn relkind_to_object_type(relkind: &str) -> Option<ObjectType> {
    match relkind {
        "r" => Some(ObjectType::Table),
        "v" => Some(ObjectType::View),
        "m" => Some(ObjectType::MaterializedView),
        "S" | "s" => Some(ObjectType::Sequence),
        "i" => Some(ObjectType::Index),
        _ => None,
    }
}

async fn fetch_object_definition(
    client: &Client,
    oid: u32,
    object_type: &ObjectType,
) -> ApiResult<Option<String>> {
    match object_type {
        ObjectType::View | ObjectType::MaterializedView => {
            let definition: String = client
                .query_one("SELECT pg_get_viewdef($1::regclass, true)", &[&oid])
                .await?
                .get(0);
            Ok(Some(definition))
        }
        ObjectType::Index => {
            let definition: String = client
                .query_one("SELECT pg_get_indexdef($1::regclass)", &[&oid])
                .await?
                .get(0);
            Ok(Some(definition))
        }
        ObjectType::Sequence => match client
            .query_opt("SELECT pg_get_sequencedef($1::regclass)", &[&oid])
            .await
        {
            Ok(Some(row)) => Ok(Some(row.get(0))),
            Ok(None) => Ok(None),
            Err(_) => Ok(None),
        },
        _ => Ok(None),
    }
}

async fn fetch_columns(client: &Client, oid: u32) -> ApiResult<Vec<ColumnDetail>> {
    let rows = client
        .query(
            "SELECT a.attname AS name,
                    pg_catalog.format_type(a.atttypid, a.atttypmod) AS data_type,
                    NOT a.attnotnull AS nullable,
                    pg_get_expr(d.adbin, d.adrelid) AS default_value,
                    col_description(a.attrelid, a.attnum) AS comment
             FROM pg_attribute a
             LEFT JOIN pg_attrdef d ON d.adrelid = a.attrelid AND d.adnum = a.attnum
             WHERE a.attrelid = $1
               AND a.attnum > 0
               AND NOT a.attisdropped
             ORDER BY a.attnum",
            &[&oid],
        )
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| ColumnDetail {
            name: row.get("name"),
            data_type: row.get("data_type"),
            nullable: row.get("nullable"),
            default: row.get("default_value"),
            comment: row.get("comment"),
        })
        .collect())
}

async fn fetch_indexes(client: &Client, oid: u32) -> ApiResult<Vec<IndexDetail>> {
    let rows = client
        .query(
            "SELECT ic.relname AS name,
                    pg_get_indexdef(ix.indexrelid) AS definition,
                    ix.indisunique,
                    array_remove(array(
                        SELECT pg_get_indexdef(ix.indexrelid, k + 1, true)
                        FROM generate_subscripts(ix.indkey, 1) AS k
                    ), NULL) AS columns,
                    obj_description(ix.indexrelid, 'pg_class') AS comment
             FROM pg_index ix
             JOIN pg_class ic ON ic.oid = ix.indexrelid
             WHERE ix.indrelid = $1
             ORDER BY ic.relname",
            &[&oid],
        )
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let columns: Option<Vec<String>> = row.get("columns");
            IndexDetail {
                name: row.get("name"),
                definition: row.get("definition"),
                is_unique: row.get("indisunique"),
                columns: columns.filter(|cols| !cols.is_empty()),
                comment: row.get("comment"),
            }
        })
        .collect())
}

async fn fetch_object_stats(client: &Client, oid: u32) -> ApiResult<Option<ObjectStatistics>> {
    let row = match client
        .query_opt(
            "SELECT s.last_analyze,
                    s.n_live_tup,
                    c.relpages,
                    ts.n_live_tup AS toast_rows,
                    t.relpages AS toast_pages,
                    s.seq_scan,
                    s.idx_scan
             FROM pg_class c
             LEFT JOIN pg_stat_all_tables s ON s.relid = c.oid
             LEFT JOIN pg_class t ON t.oid = c.reltoastrelid
             LEFT JOIN pg_stat_all_tables ts ON ts.relid = c.reltoastrelid
             WHERE c.oid = $1",
            &[&oid],
        )
        .await?
    {
        Some(row) => row,
        None => return Ok(None),
    };

    let stats = ObjectStatistics {
        last_analyzed_at: row.get("last_analyze"),
        rows: row.get("n_live_tup"),
        rel_pages: row
            .get::<_, Option<i32>>("relpages")
            .map(|value| value as i64),
        toast_rows: row.get("toast_rows"),
        toast_pages: row
            .get::<_, Option<i32>>("toast_pages")
            .map(|value| value as i64),
        sequential_scans: row.get("seq_scan"),
        index_scans: row.get("idx_scan"),
    };

    if stats.last_analyzed_at.is_some()
        || stats.rows.is_some()
        || stats.rel_pages.is_some()
        || stats.toast_rows.is_some()
        || stats.toast_pages.is_some()
        || stats.sequential_scans.is_some()
        || stats.index_scans.is_some()
    {
        Ok(Some(stats))
    } else {
        Ok(None)
    }
}

pub async fn get_object_details(
    ConnectionString(conn): ConnectionString,
    Path(path): Path<GetObjectPath>,
    Query(query): Query<GetObjectQuery>,
) -> ApiResult<Json<GetObjectDetailsResponse>> {
    let client = db::connect(&conn).await?;

    if let Some(object) =
        fetch_relation_details(&client, &path.schema, &path.object, query.include_stats).await?
    {
        return Ok(Json(GetObjectDetailsResponse { object }));
    }

    if let Some(object) = fetch_routine_details(&client, &path.schema, &path.object).await? {
        return Ok(Json(GetObjectDetailsResponse { object }));
    }

    Err(ApiError::not_found(format!(
        "object `{}` not found in schema `{}`",
        path.object, path.schema
    )))
}
