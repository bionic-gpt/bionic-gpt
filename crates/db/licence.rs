use base64::decode;
use ed25519_dalek::{pkcs8::DecodePublicKey, Signature, Verifier, VerifyingKey, SIGNATURE_LENGTH};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::{format_description::well_known::Rfc3339, Date, Month, OffsetDateTime, Time, UtcOffset};

const PUBLIC_KEY: &str = "\
-----BEGIN PUBLIC KEY-----\n\
MCowBQYDK2VwAyEAJguqlxohUamZpCPUGY8k5oBYlHSnCY66eTyothyPJM0=\n\
-----END PUBLIC KEY-----";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Licence {
    pub user_count: usize,
    pub hostname_url: String,
    #[serde(deserialize_with = "deserialize_rfc3339")]
    pub end_date: OffsetDateTime,
    pub signature: String,
}

fn deserialize_rfc3339<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)
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
                Ok(licence) => {
                    if !licence.verify() {
                        tracing::error!("Licence signature verification failed");
                        return Self::default();
                    }
                    tracing::info!("Successful signed licence verification");
                    licence
                }
                Err(err) => {
                    tracing::error!("Failed to parse licence from environment: {}", err);
                    Self::default()
                }
            }
        } else {
            tracing::error!("Licence not found");
            Self::default()
        }
    }

    pub fn verify(&self) -> bool {
        let Ok(public_key) = VerifyingKey::from_public_key_pem(PUBLIC_KEY) else {
            tracing::error!("Unable to read PUBLIC_KEY");
            return false;
        };

        let Ok(signature_bytes_vec) = decode(self.signature.trim()) else {
            tracing::error!("Unable to parse signature");
            return false;
        };

        let Ok(signature_bytes) = <[u8; SIGNATURE_LENGTH]>::try_from(signature_bytes_vec) else {
            tracing::error!("Invalid signature");
            return false;
        };

        let signature = Signature::from_bytes(&signature_bytes);

        let formatted_date = self.end_date.format(&Rfc3339).expect("valid date");

        let value = json!({
            "user_count": self.user_count,
            "hostname_url": self.hostname_url,
            "end_date": formatted_date,
        });

        tracing::debug!("{}", serde_json::to_string(&value).unwrap());

        let Ok(data) = serde_json::to_vec(&value) else {
            tracing::error!("Unable to parse JSON");
            return false;
        };

        public_key.verify(&data, &signature).is_ok()
    }
}
