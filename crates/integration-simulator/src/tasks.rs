use serde_json::Value;

pub const JORDAN_TASK: &str = "simple.email_sf_contact_phone_update";
pub const IMPORTANT_DRAFT_TASK: &str = "sales.create_important_draft";

const JORDAN_TASK_JSON: &str = include_str!("../tasks/simple.email_sf_contact_phone_update.json");
const IMPORTANT_DRAFT_TASK_JSON: &str = include_str!("../tasks/sales.create_important_draft.json");

pub fn load(task: &str) -> Result<Value, String> {
    let source = match task {
        JORDAN_TASK => JORDAN_TASK_JSON,
        IMPORTANT_DRAFT_TASK => IMPORTANT_DRAFT_TASK_JSON,
        _ => return Err(format!("unknown AutomationBench task: {task}")),
    };

    serde_json::from_str(source).map_err(|error| format!("invalid fixture for {task}: {error}"))
}
