use serde_json::{json, Value};
use std::collections::BTreeMap;
use tokio::sync::RwLock;

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
        Self::seeded()
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
    pub async fn reset(&self) {
        *self.state.write().await = WorldState::seeded();
    }

    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, WorldState> {
        self.state.read().await
    }

    pub async fn write(&self) -> tokio::sync::RwLockWriteGuard<'_, WorldState> {
        self.state.write().await
    }
}

impl WorldState {
    fn seeded() -> Self {
        let mut gmail = GmailState::default();
        gmail.labels.insert(
            "INBOX".to_string(),
            json!({"id":"INBOX","name":"INBOX","type":"system"}),
        );
        gmail.labels.insert(
            "SENT".to_string(),
            json!({"id":"SENT","name":"SENT","type":"system"}),
        );
        gmail.labels.insert(
            "DRAFT".to_string(),
            json!({"id":"DRAFT","name":"DRAFT","type":"system"}),
        );
        gmail.labels.insert(
            "TRASH".to_string(),
            json!({"id":"TRASH","name":"TRASH","type":"system"}),
        );

        let message = json!({
            "id": "msg-jordan-001",
            "threadId": "thread-jordan-001",
            "labelIds": ["INBOX"],
            "snippet": "Jordan Lee asked for an update on the contact record.",
            "internalDate": "1788432000000",
            "payload": {"headers": [
                {"name":"From","value":"jordan.lee@example.com"},
                {"name":"To","value":"alex@example.com"},
                {"name":"Subject","value":"Contact details update"}
            ], "body": {"data":"Jordan asked to update the phone number for the Salesforce contact."}}
        });
        gmail
            .messages
            .insert("msg-jordan-001".to_string(), message.clone());
        gmail.threads.insert(
            "thread-jordan-001".to_string(),
            json!({"id":"thread-jordan-001","snippet":"Jordan Lee asked for an update on the contact record.","messages":[message]}),
        );

        let mut salesforce = SalesforceState::default();
        salesforce.insert("Contact", "003JORDANLEE", json!({
            "attributes":{"type":"Contact","url":"/services/data/v61.0/sobjects/Contact/003JORDANLEE"},
            "Id":"003JORDANLEE","FirstName":"Jordan","LastName":"Lee",
            "Email":"jordan.lee@example.com","Phone":"+1 555 0100","Title":"Operations Director"
        }));
        salesforce.insert("Account", "001EXAMPLE", json!({
            "attributes":{"type":"Account","url":"/services/data/v61.0/sobjects/Account/001EXAMPLE"},
            "Id":"001EXAMPLE","Name":"Example Industries"
        }));

        Self { gmail, salesforce }
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
