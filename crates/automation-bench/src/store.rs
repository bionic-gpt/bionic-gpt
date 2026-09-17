use chrono::DateTime;
use serde_json::{json, Value};
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
}

#[derive(Clone)]
pub struct GmailState {
    pub messages: BTreeMap<String, Value>,
    pub drafts: BTreeMap<String, Value>,
    pub labels: BTreeMap<String, Value>,
    pub threads: BTreeMap<String, Value>,
    pub next_id: u64,
}

#[derive(Clone)]
pub struct SalesforceState {
    pub records: BTreeMap<String, BTreeMap<String, Value>>,
    pub next_id: u64,
}

impl Default for WorldState {
    fn default() -> Self {
        let fixture = tasks::load(tasks::JORDAN_TASK).expect("canonical task fixture is valid");
        Self::from_fixture(&fixture).expect("canonical task fixture has valid state")
    }
}

impl Default for GmailState {
    fn default() -> Self {
        Self {
            messages: BTreeMap::new(),
            drafts: BTreeMap::new(),
            labels: BTreeMap::new(),
            threads: BTreeMap::new(),
            next_id: 1,
        }
    }
}

impl Default for SalesforceState {
    fn default() -> Self {
        Self {
            records: BTreeMap::new(),
            next_id: 1,
        }
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
        let assertion = fixture
            .pointer("/info/assertions/0")
            .ok_or("task has no assertion")?;
        let record_id = assertion["record_id"]
            .as_str()
            .ok_or("assertion has no record_id")?;
        let field = assertion["field"]
            .as_str()
            .ok_or("assertion has no field")?;
        let expected = assertion["value"]
            .as_str()
            .ok_or("assertion has no expected value")?;
        let api_field = match field {
            "phone" => "Phone",
            other => other,
        };
        let actual = self
            .read()
            .await
            .salesforce
            .records
            .get("Contact")
            .and_then(|records| records.get(record_id))
            .and_then(|record| record.get(api_field))
            .cloned()
            .unwrap_or(Value::Null);
        let passed = actual.as_str() == Some(expected);

        Ok(json!({
            "task": task,
            "passed": passed,
            "assertions": [{
                "type": assertion["type"],
                "collection": assertion["collection"],
                "record_id": record_id,
                "field": field,
                "expected": expected,
                "actual": actual,
                "passed": passed
            }]
        }))
    }
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
        let gmail_state = initial_state
            .get("gmail")
            .ok_or("task fixture has no Gmail state")?;
        let salesforce_state = initial_state
            .get("salesforce")
            .ok_or("task fixture has no Salesforce state")?;

        let mut gmail = GmailState::default();
        for message in gmail_state
            .get("messages")
            .and_then(Value::as_array)
            .ok_or("Gmail state has no messages")?
        {
            let api_message = gmail_message(message)?;
            let id = api_message["id"]
                .as_str()
                .ok_or("Gmail message has no id")?
                .to_string();
            gmail.messages.insert(id, api_message);
        }

        let mut salesforce = SalesforceState::default();
        for contact in salesforce_state
            .get("contacts")
            .and_then(Value::as_array)
            .ok_or("Salesforce state has no contacts")?
        {
            let record = salesforce_contact(contact)?;
            let id = record["Id"]
                .as_str()
                .ok_or("Salesforce contact has no id")?
                .to_string();
            salesforce.insert("Contact", &id, record);
        }

        sync_gmail_threads(&mut gmail);
        Ok(Self { gmail, salesforce })
    }
}

fn gmail_message(message: &Value) -> Result<Value, String> {
    let string = |name: &str| {
        message
            .get(name)
            .and_then(Value::as_str)
            .ok_or_else(|| format!("Gmail message has no {name}"))
    };
    let recipients = message
        .get("to")
        .and_then(Value::as_array)
        .ok_or("Gmail message has no recipients")?
        .iter()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    let body = string("body_plain")?;
    let date = DateTime::parse_from_rfc3339(string("date")?)
        .map_err(|error| format!("invalid Gmail date: {error}"))?
        .timestamp_millis();

    Ok(json!({
        "id": string("id")?,
        "threadId": string("thread_id")?,
        "labelIds": message.get("label_ids").cloned().unwrap_or_else(|| json!([])),
        "snippet": body,
        "internalDate": date,
        "payload": {
            "headers": [
                {"name": "From", "value": string("from_")?},
                {"name": "To", "value": recipients},
                {"name": "Subject", "value": string("subject")?}
            ],
            "body": {"data": body}
        }
    }))
}

fn salesforce_contact(contact: &Value) -> Result<Value, String> {
    let fields = contact
        .as_object()
        .ok_or("Salesforce contact is not an object")?;
    let mut record = serde_json::Map::new();
    for (name, value) in fields {
        let api_name = match name.as_str() {
            "id" => "Id",
            "first_name" => "FirstName",
            "last_name" => "LastName",
            "email" => "Email",
            "phone" => "Phone",
            "title" => "Title",
            "account_id" => "AccountId",
            other => other,
        };
        record.insert(api_name.to_string(), value.clone());
    }
    let id = record
        .get("Id")
        .and_then(Value::as_str)
        .ok_or("Salesforce contact has no id")?;
    record.insert(
        "attributes".to_string(),
        json!({"type":"Contact","url":format!("/services/data/v61.0/sobjects/Contact/{id}")}),
    );
    Ok(Value::Object(record))
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
