use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Default)]
pub struct CredentialSet {
    values: BTreeMap<String, Credential>,
}

impl fmt::Debug for CredentialSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialSet")
            .field("keys", &self.values.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl CredentialSet {
    pub fn insert(&mut self, key: impl Into<String>, credential: Credential) {
        self.values.insert(key.into(), credential);
    }

    pub fn get(&self, key: &str) -> Option<&Credential> {
        self.values.get(key)
    }
}

#[derive(Clone)]
pub enum Credential {
    Header { name: String, value: String },
    Bearer { token: String },
}

impl fmt::Debug for Credential {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Header { name, .. } => formatter
                .debug_struct("Header")
                .field("name", name)
                .field("value", &"[REDACTED]")
                .finish(),
            Self::Bearer { .. } => formatter.write_str("Bearer([REDACTED])"),
        }
    }
}
