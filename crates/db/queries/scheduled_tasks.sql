--: ScheduledTask(project_id?, last_run_at?)
--: ScheduledTaskRun(started_at?, completed_at?, error?, conversation_id?)

--! create : ScheduledTask
INSERT INTO scheduled_tasks.tasks (
    user_id, team_id, project_id, model_id, name, prompt, cron, timezone, next_run_at
)
VALUES (current_app_user(), :team_id, :project_id, :model_id, :name, :prompt, :cron, :timezone, :next_run_at)
RETURNING id, user_id, team_id, project_id, model_id, name, prompt, cron, timezone, enabled,
    next_run_at, last_run_at, created_at, updated_at;

--! list : ScheduledTask
SELECT id, user_id, team_id, project_id, model_id, name, prompt, cron, timezone, enabled,
    next_run_at, last_run_at, created_at, updated_at
FROM scheduled_tasks.tasks
WHERE user_id = current_app_user()
  AND team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user())
ORDER BY next_run_at, id;

--! update : ScheduledTask
UPDATE scheduled_tasks.tasks
SET name = COALESCE(:name, name), prompt = COALESCE(:prompt, prompt),
    cron = COALESCE(:cron, cron), timezone = COALESCE(:timezone, timezone),
    enabled = COALESCE(:enabled, enabled), next_run_at = COALESCE(:next_run_at, next_run_at)
WHERE id = :task_id
  AND user_id = current_app_user()
  AND team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user())
RETURNING id, user_id, team_id, project_id, model_id, name, prompt, cron, timezone, enabled,
    next_run_at, last_run_at, created_at, updated_at;

--! delete
DELETE FROM scheduled_tasks.tasks
WHERE id = :task_id
  AND user_id = current_app_user()
  AND team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user());

--! due : ScheduledTask
SELECT id, user_id, team_id, project_id, model_id, name, prompt, cron, timezone, enabled,
    next_run_at, last_run_at, created_at, updated_at
FROM scheduled_tasks.tasks
WHERE enabled = TRUE AND next_run_at <= :now
ORDER BY next_run_at, id
LIMIT :limit;

--! claim_due : ScheduledTask
SELECT id, user_id, team_id, project_id, model_id, name, prompt, cron, timezone, enabled,
    next_run_at, last_run_at, created_at, updated_at
FROM scheduled_tasks.tasks
WHERE id = :task_id AND enabled = TRUE AND next_run_at <= :now
FOR UPDATE;

--! advance
UPDATE scheduled_tasks.tasks
SET next_run_at = :next_run_at, last_run_at = :scheduled_for
WHERE id = :task_id AND next_run_at = :scheduled_for AND enabled = TRUE;

--! create_worker_conversation
INSERT INTO llm.conversations (user_id, team_id, project_id)
VALUES (current_app_user(), :team_id, :project_id)
RETURNING id;

--! create_worker_conversation_without_project
INSERT INTO llm.conversations (user_id, team_id)
VALUES (current_app_user(), :team_id)
RETURNING id;

--! create_worker_chat
INSERT INTO llm.chats (conversation_id, model_id, content, role, status)
VALUES (:conversation_id, :model_id, encrypt_text(:content), 'User', 'Pending')
RETURNING id;

--! create_run : ScheduledTaskRun
INSERT INTO scheduled_tasks.runs (task_id, scheduled_for)
VALUES (:task_id, :scheduled_for)
ON CONFLICT (task_id, scheduled_for) DO NOTHING
RETURNING id, task_id, scheduled_for, started_at, completed_at, status, error,
    conversation_id, created_at;

--! mark_run_running
UPDATE scheduled_tasks.runs
SET status = 'running', started_at = NOW()
WHERE id = :run_id;

--! mark_run_completed
UPDATE scheduled_tasks.runs
SET status = 'completed', completed_at = NOW(), conversation_id = :conversation_id
WHERE id = :run_id;

--! mark_run_failed
UPDATE scheduled_tasks.runs
SET status = 'failed', completed_at = NOW(), error = :error
WHERE id = :run_id;

--! disable
UPDATE scheduled_tasks.tasks
SET enabled = FALSE
WHERE id = :task_id
  AND user_id = current_app_user()
  AND team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user());
