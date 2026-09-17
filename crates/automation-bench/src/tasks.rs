use serde_json::Value;

pub const JORDAN_TASK: &str = "simple.email_sf_contact_phone_update";

const JORDAN_TASK_JSON: &str = include_str!("../tasks/simple.email_sf_contact_phone_update.json");

pub fn load(task: &str) -> Result<Value, String> {
    if task != JORDAN_TASK {
        return Err(format!("unknown AutomationBench task: {task}"));
    }

    serde_json::from_str(JORDAN_TASK_JSON)
        .map_err(|error| format!("invalid fixture for {JORDAN_TASK}: {error}"))
}
