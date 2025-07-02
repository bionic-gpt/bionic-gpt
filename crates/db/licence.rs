use base64::decode;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Licence {
    pub user_count: usize,
    pub hostname_url: String,
    #[serde(with = "time::serde::iso8601")]
    pub end_date: OffsetDateTime,
    pub signature: String,
}

impl Default for Licence {
    fn default() -> Self {
        Self {
            user_count: 2,
            hostname_url: String::new(),
            end_date: Date::from_calendar_date(9999, Month::December, 31)
                .expect("valid hardcoded date")
                .with_time(Time::MIDNIGHT)
                .assume_offset(UtcOffset::UTC),
            signature: String::new(),
        }
    }
}

impl Licence {
    pub fn from_env() -> Self {
        if let Ok(json) = std::env::var("LICENCE") {
            match serde_json::from_str::<Self>(&json) {
                Ok(mut licence) => {
                    if let Ok(public_key) = std::env::var("LICENCE_PUBLIC_KEY") {
                        if !licence.verify(&public_key) {
                            tracing::error!("Licence signature verification failed");
                            return Self::default();
                        }
                    }
                    licence
                }
                Err(err) => {
                    tracing::error!("Failed to parse licence from environment: {}", err);
                    Self::default()
                }
            }
        } else {
            Self::default()
        }
    }

    pub fn verify(&self, public_key_b64: &str) -> bool {
        let Ok(public_key_bytes) = decode(public_key_b64.trim()) else {
            return false;
        };
        let Ok(public_key) = PublicKey::from_bytes(&public_key_bytes) else {
            return false;
        };
        let Ok(signature_bytes) = decode(self.signature.trim()) else {
            return false;
        };
        let Ok(signature) = Signature::from_bytes(&signature_bytes) else {
            return false;
        };

        let value = json!({
            "user_count": self.user_count,
            "hostname_url": self.hostname_url,
            "end_date": self.end_date,
        });
        let data = serde_json::to_vec(&value).expect("serializable");
        public_key.verify(&data, &signature).is_ok()
    }
}
