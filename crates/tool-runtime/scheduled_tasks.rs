use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use cron::Schedule;
use db::Pool;
use serde_json::{json, Map, Value};
use std::str::FromStr;

#[derive(Clone)]
pub struct Context {
    pub pool: Pool,
    pub sub: String,
    pub conversation_id: i64,
    pub prompt_id: i32,
    pub team_id: i32,
    pub project_id: Option<i32>,
}

#[derive(Clone, Copy)]
pub enum Operation {
    Create,
    List,
    Update,
    Delete,
}

pub fn next_run_at(
    expression: &str,
    timezone: &str,
    now: DateTime<Utc>,
) -> Result<DateTime<Utc>, String> {
    let schedule = parse_schedule(expression)?;
    let timezone =
        Tz::from_str(timezone).map_err(|_| format!("invalid IANA timezone: {timezone}"))?;
    schedule
        .after(&now.with_timezone(&timezone))
        .next()
        .map(|value| value.with_timezone(&Utc))
        .ok_or_else(|| "cron expression has no future occurrence".to_string())
}

fn parse_schedule(expression: &str) -> Result<Schedule, String> {
    let fields = expression.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 5 {
        return Err(
            "cron must use exactly five fields: minute hour day-of-month month day-of-week"
                .to_string(),
        );
    }
    Schedule::from_str(&format!("0 {expression}"))
        .map_err(|error| format!("invalid cron expression: {error}"))
}

pub async fn execute(
    context: &Context,
    operation: Operation,
    arguments: Value,
) -> Result<Value, String> {
    match operation {
        Operation::Create => create(context, arguments).await,
        Operation::List => list(context).await,
        Operation::Update => update(context, arguments).await,
        Operation::Delete => delete(context, arguments).await,
    }
}

async fn create(context: &Context, arguments: Value) -> Result<Value, String> {
    let object = object(arguments)?;
    let name = required_string(&object, "name")?;
    let prompt = required_string(&object, "prompt")?;
    let cron = required_string(&object, "cron")?;
    let timezone = required_string(&object, "timezone")?;
    let next_run_at = next_run_at(&cron, &timezone, Utc::now())?;
    let mut client = context.pool.get().await.map_err(|e| e.to_string())?;
    let tx = client.transaction().await.map_err(|e| e.to_string())?;
    db::authz::set_row_level_security_user_id(&tx, context.sub.clone())
        .await
        .map_err(|e| e.to_string())?;
    let row = tx.query_one(
        "INSERT INTO scheduled_tasks.tasks (user_id, team_id, project_id, prompt_id, name, prompt, cron, timezone, next_run_at) VALUES (current_app_user(), $1, $2, $8, $3, $4, $5, $6, $7) RETURNING id, name, prompt, cron, timezone, enabled, next_run_at",
        &[&context.team_id, &context.project_id, &name, &prompt, &cron, &timezone, &next_run_at, &context.prompt_id],
    ).await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(task_json(&row))
}

async fn list(context: &Context) -> Result<Value, String> {
    let mut client = context.pool.get().await.map_err(|e| e.to_string())?;
    let tx = client.transaction().await.map_err(|e| e.to_string())?;
    db::authz::set_row_level_security_user_id(&tx, context.sub.clone())
        .await
        .map_err(|e| e.to_string())?;
    let rows = tx.query("SELECT id, name, prompt, cron, timezone, enabled, next_run_at FROM scheduled_tasks.tasks WHERE user_id = current_app_user() AND team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()) ORDER BY next_run_at, id", &[]).await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(Value::Array(rows.iter().map(task_json).collect()))
}

async fn update(context: &Context, arguments: Value) -> Result<Value, String> {
    let object = object(arguments)?;
    let task_id = object
        .get("task_id")
        .and_then(Value::as_i64)
        .ok_or_else(|| "task_id must be an integer".to_string())?;
    let mut client = context.pool.get().await.map_err(|e| e.to_string())?;
    let tx = client.transaction().await.map_err(|e| e.to_string())?;
    db::authz::set_row_level_security_user_id(&tx, context.sub.clone())
        .await
        .map_err(|e| e.to_string())?;
    let current = tx.query_opt("SELECT name, prompt, cron, timezone, enabled FROM scheduled_tasks.tasks WHERE id = $1 AND user_id = current_app_user() AND team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user())", &[&task_id]).await.map_err(|e| e.to_string())?.ok_or_else(|| "scheduled task not found".to_string())?;
    let name = optional_string(&object, "name")?.unwrap_or_else(|| current.get(0));
    let prompt = optional_string(&object, "prompt")?.unwrap_or_else(|| current.get(1));
    let cron = optional_string(&object, "cron")?.unwrap_or_else(|| current.get(2));
    let timezone = optional_string(&object, "timezone")?.unwrap_or_else(|| current.get(3));
    let enabled = match object.get("enabled") {
        None | Some(Value::Null) => current.get(4),
        Some(Value::Bool(value)) => *value,
        Some(_) => return Err("enabled must be a boolean".to_string()),
    };
    let next = if object.contains_key("cron") || object.contains_key("timezone") {
        Some(next_run_at(&cron, &timezone, Utc::now())?)
    } else {
        None
    };
    let row = tx.query_one("UPDATE scheduled_tasks.tasks SET name = $1, prompt = $2, cron = $3, timezone = $4, enabled = $5, next_run_at = COALESCE($6, next_run_at) WHERE id = $7 RETURNING id, name, prompt, cron, timezone, enabled, next_run_at", &[&name, &prompt, &cron, &timezone, &enabled, &next, &task_id]).await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(task_json(&row))
}

async fn delete(context: &Context, arguments: Value) -> Result<Value, String> {
    let object = object(arguments)?;
    let task_id = object
        .get("task_id")
        .and_then(Value::as_i64)
        .ok_or_else(|| "task_id must be an integer".to_string())?;
    let mut client = context.pool.get().await.map_err(|e| e.to_string())?;
    let tx = client.transaction().await.map_err(|e| e.to_string())?;
    db::authz::set_row_level_security_user_id(&tx, context.sub.clone())
        .await
        .map_err(|e| e.to_string())?;
    let count = tx.execute("DELETE FROM scheduled_tasks.tasks WHERE id = $1 AND user_id = current_app_user() AND team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user())", &[&task_id]).await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    if count == 0 {
        return Err("scheduled task not found".to_string());
    }
    Ok(json!({"deleted": true, "task_id": task_id}))
}

fn object(value: Value) -> Result<Map<String, Value>, String> {
    value
        .as_object()
        .cloned()
        .ok_or_else(|| "arguments must be an object".to_string())
}
fn required_string(object: &Map<String, Value>, name: &str) -> Result<String, String> {
    optional_string(object, name)?.ok_or_else(|| format!("{name} is required"))
}
fn optional_string(object: &Map<String, Value>, name: &str) -> Result<Option<String>, String> {
    match object.get(name) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) if !value.trim().is_empty() => Ok(Some(value.clone())),
        Some(_) => Err(format!("{name} must be a non-empty string")),
    }
}
fn task_json(row: &tokio_postgres::Row) -> Value {
    json!({"id": row.get::<_, i64>(0), "name": row.get::<_, String>(1), "prompt": row.get::<_, String>(2), "cron": row.get::<_, String>(3), "timezone": row.get::<_, String>(4), "enabled": row.get::<_, bool>(5), "next_run_at": row.get::<_, DateTime<Utc>>(6).to_rfc3339()})
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn calculates_next_run_in_timezone() {
        let now = Utc.with_ymd_and_hms(2026, 8, 31, 6, 0, 0).unwrap();
        assert_eq!(
            next_run_at("0 8 * * *", "Europe/Berlin", now).unwrap(),
            Utc.with_ymd_and_hms(2026, 9, 1, 6, 0, 0).unwrap()
        );
    }

    #[test]
    fn rejects_invalid_schedule_and_timezone() {
        assert!(next_run_at("not a cron", "Europe/Berlin", Utc::now()).is_err());
        assert!(next_run_at("0 8 * * *", "Not/AZone", Utc::now()).is_err());
    }
}
