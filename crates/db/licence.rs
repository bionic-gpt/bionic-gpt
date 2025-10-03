use base64::decode;
use ed25519_dalek::{pkcs8::DecodePublicKey, Signature, Verifier, VerifyingKey, SIGNATURE_LENGTH};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use time::{format_description::well_known::Rfc3339, Date, Month, OffsetDateTime, Time, UtcOffset};

const PUBLIC_KEY: &str = "\
-----BEGIN PUBLIC KEY-----\n\
MCowBQYDK2VwAyEAJguqlxohUamZpCPUGY8k5oBYlHSnCY66eTyothyPJM0=\n\
-----END PUBLIC KEY-----";

static LICENCE: OnceLock<Licence> = OnceLock::new();

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Licence {
    pub user_count: usize,
    pub hostname_url: String,
    #[serde(deserialize_with = "deserialize_rfc3339")]
    pub end_date: OffsetDateTime,
    pub signature: String,
    pub app_name: String,
    pub app_logo_svg: String,
    #[serde(default)]
    pub default_lang: String,
    #[serde(default)]
    pub redirect_url: Option<String>,
}

#[derive(Serialize)]
struct SignableLicence<'a> {
    user_count: usize,
    hostname_url: &'a str,
    end_date: &'a str,
    app_name: &'a str,
    app_logo_svg: &'a str,
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
            app_name: String::new(),
            app_logo_svg: String::new(),
            default_lang: String::new(),
            redirect_url: None,
        }
    }
}

impl Licence {
    /// Returns the global cached licence
    pub fn global() -> &'static Self {
        LICENCE.get_or_init(Self::from_env)
    }

    /// Tries to load, verify, and return the licence from environment
    fn from_env() -> Self {
        if let Ok(json) = std::env::var("LICENCE") {
            tracing::debug!("Licence: {}", json);
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
            tracing::error!("Invalid signature length");
            return false;
        };

        let signature = Signature::from_bytes(&signature_bytes);

        let formatted_date = self.end_date.format(&Rfc3339).expect("valid date");

        let signable = SignableLicence {
            user_count: self.user_count,
            hostname_url: &self.hostname_url,
            end_date: &formatted_date,
            app_name: &self.app_name,
            app_logo_svg: &self.app_logo_svg,
        };

        tracing::debug!(
            "Signed payload: {}",
            serde_json::to_string(&signable).expect("serializable payload"),
        );

        let Ok(data) = serde_json::to_vec(&signable) else {
            tracing::error!("Unable to convert JSON to bytes");
            return false;
        };

        public_key.verify(&data, &signature).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_payload_matches_js_signer() {
        let licence = Licence {
            user_count: 42,
            hostname_url: "https://example.com".to_string(),
            end_date: OffsetDateTime::parse("2028-12-31T00:00:00Z", &Rfc3339).unwrap(),
            signature: String::new(),
            app_name: "Deploy".to_string(),
            app_logo_svg: "logo".to_string(),
            default_lang: "en-US".to_string(),
            redirect_url: None,
        };

        let formatted_date = licence.end_date.format(&Rfc3339).expect("valid date");
        let signable = SignableLicence {
            user_count: licence.user_count,
            hostname_url: &licence.hostname_url,
            end_date: &formatted_date,
            app_name: &licence.app_name,
            app_logo_svg: &licence.app_logo_svg,
        };

        let payload = serde_json::to_string(&signable).unwrap();
        assert_eq!(
            payload,
            "{\"user_count\":42,\"hostname_url\":\"https://example.com\",\"end_date\":\"2028-12-31T00:00:00Z\",\"app_name\":\"Deploy\",\"app_logo_svg\":\"logo\"}"
        );
    }
}
