use serde::{Deserialize, Serialize};
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Licence {
    pub user_count: usize,
    #[serde(with = "time::serde::iso8601")]
    pub end_date: OffsetDateTime,
    pub signature: String,
}

impl Default for Licence {
    fn default() -> Self {
        Self {
            user_count: 2,
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
            match serde_json::from_str(&json) {
                Ok(licence) => licence,
                Err(err) => {
                    tracing::error!("Failed to parse licence from environment: {}", err);
                    Self::default()
                }
            }
        } else {
            Self::default()
        }
    }
}
