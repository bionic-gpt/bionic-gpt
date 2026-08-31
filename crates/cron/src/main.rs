use agent_runtime::Jwt;
use chrono::Utc;
use db::queries::scheduled_tasks;
use db::{Pool, ScheduledTask};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

const DEFAULT_POLL_SECONDS: u64 = 15;
const DEFAULT_WORKERS: usize = 4;
const DEFAULT_MAX_TURNS: usize = 20;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("cron=info")),
        )
        .init();

    let database_url = std::env::var("APP_DATABASE_URL")?;
    let pool = db::create_pool(&database_url);
    let poll_seconds = env_usize("SCHEDULED_TASK_POLL_SECONDS", DEFAULT_POLL_SECONDS as usize);
    let workers = env_usize("SCHEDULED_TASK_WORKERS", DEFAULT_WORKERS);
    let max_turns = env_usize("SCHEDULED_TASK_MAX_TURNS", DEFAULT_MAX_TURNS);

    tracing::info!(
        poll_seconds,
        workers,
        max_turns,
        "scheduled task worker started"
    );
    run(
        pool,
        Duration::from_secs(poll_seconds as u64),
        workers,
        max_turns,
    )
    .await;
    Ok(())
}

async fn run(pool: Pool, poll_interval: Duration, workers: usize, max_turns: usize) {
    let semaphore = Arc::new(Semaphore::new(workers.max(1)));
    let mut interval = tokio::time::interval(poll_interval);

    loop {
        tokio::select! {
            _ = interval.tick() => {
                dispatch_due_tasks(&pool, Arc::clone(&semaphore), max_turns).await;
            }
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("scheduled task worker stopping");
                break;
            }
        }
    }
}

async fn dispatch_due_tasks(pool: &Pool, semaphore: Arc<Semaphore>, max_turns: usize) {
    let now = Utc::now().fixed_offset();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(error) => {
            tracing::error!(%error, "failed to acquire database connection");
            return;
        }
    };
    let due = match scheduled_tasks::due()
        .bind(&client, &now, &100_i64)
        .all()
        .await
    {
        Ok(tasks) => tasks,
        Err(error) => {
            tracing::error!(%error, "failed to load due scheduled tasks");
            return;
        }
    };
    drop(client);

    for task in due {
        let permit = match Arc::clone(&semaphore).try_acquire_owned() {
            Ok(permit) => permit,
            Err(_) => break,
        };
        let Some((task, run_id)) = claim_task(pool, task).await else {
            drop(permit);
            continue;
        };
        let pool = pool.clone();
        tokio::spawn(async move {
            let _permit = permit;
            execute_task(&pool, task, run_id, max_turns).await;
        });
    }
}

async fn claim_task(pool: &Pool, candidate: ScheduledTask) -> Option<(ScheduledTask, i64)> {
    let mut client = pool.get().await.ok()?;
    let tx = client.transaction().await.ok()?;
    let now = Utc::now().fixed_offset();
    let task = scheduled_tasks::claim_due()
        .bind(&tx, &candidate.id, &now)
        .one()
        .await
        .ok()?;
    let run = scheduled_tasks::create_run()
        .bind(&tx, &task.id, &task.next_run_at)
        .opt()
        .await
        .ok()??;
    let next_run = match tool_runtime::scheduled_tasks::next_run_at(
        &task.cron,
        &task.timezone,
        task.next_run_at.with_timezone(&Utc),
    ) {
        Ok(next_run) => next_run.fixed_offset(),
        Err(error) => {
            tracing::error!(task_id = task.id, %error, "failed to advance scheduled task");
            return None;
        }
    };
    if scheduled_tasks::advance()
        .bind(&tx, &next_run, &task.next_run_at, &task.id)
        .await
        .ok()?
        != 1
    {
        return None;
    }
    tx.commit().await.ok()?;
    Some((task, run.id))
}

async fn execute_task(pool: &Pool, task: ScheduledTask, run_id: i64, max_turns: usize) {
    let result = execute_task_inner(pool, &task, run_id, max_turns).await;
    if let Err(error) = result {
        tracing::error!(task_id = task.id, run_id, %error, "scheduled task failed");
        let _ = update_run_failed(pool, &task, run_id, &error).await;
    }
}

async fn execute_task_inner(
    pool: &Pool,
    task: &ScheduledTask,
    run_id: i64,
    max_turns: usize,
) -> Result<(), String> {
    let mut client = pool.get().await.map_err(|error| error.to_string())?;
    let owner = client
        .query_one(
            "SELECT openid_sub, email FROM iam.users WHERE id = $1",
            &[&task.user_id],
        )
        .await
        .map_err(|error| error.to_string())?;
    let sub: String = owner.get(0);
    let email: String = owner.get(1);
    let tx = client
        .transaction()
        .await
        .map_err(|error| error.to_string())?;
    db::authz::set_row_level_security_user_id(&tx, sub.clone())
        .await
        .map_err(|error| error.to_string())?;
    scheduled_tasks::mark_run_running()
        .bind(&tx, &run_id)
        .await
        .map_err(|error| error.to_string())?;
    let conversation_id = if let Some(project_id) = task.project_id {
        scheduled_tasks::create_worker_conversation()
            .bind(&tx, &task.team_id, &project_id)
            .one()
            .await
            .map_err(|error| error.to_string())?
    } else {
        scheduled_tasks::create_worker_conversation_without_project()
            .bind(&tx, &task.team_id)
            .one()
            .await
            .map_err(|error| error.to_string())?
    };
    let chat_id = scheduled_tasks::create_worker_chat()
        .bind(&tx, &conversation_id, &task.prompt_id, &task.prompt)
        .one()
        .await
        .map_err(|error| error.to_string())?;
    tx.commit().await.map_err(|error| error.to_string())?;

    agent_runtime::ui_chat_orchestrator::run_scheduled_chat(
        pool.clone(),
        Jwt {
            sub,
            email,
            given_name: None,
            family_name: None,
        },
        chat_id,
        max_turns,
    )
    .await
    .map_err(|error| error.to_string())?;
    mark_run_completed(pool, run_id, conversation_id).await
}

async fn mark_run_completed(pool: &Pool, run_id: i64, conversation_id: i64) -> Result<(), String> {
    let client = pool.get().await.map_err(|error| error.to_string())?;
    scheduled_tasks::mark_run_completed()
        .bind(&client, &run_id, &conversation_id)
        .await
        .map_err(|error| error.to_string())?;
    Ok(())
}

async fn update_run_failed(
    pool: &Pool,
    _task: &ScheduledTask,
    run_id: i64,
    error: &str,
) -> Result<(), String> {
    let client = pool.get().await.map_err(|error| error.to_string())?;
    scheduled_tasks::mark_run_failed()
        .bind(&client, &error.to_string(), &run_id)
        .await
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value: &usize| *value > 0)
        .unwrap_or(default)
}
