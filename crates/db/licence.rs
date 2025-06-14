use serde::{Deserialize, Serialize};
use time::{Date, Month};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Licence {
    pub user_count: usize,
    pub end_date: Date,
    pub signature: String,
}

impl Default for Licence {
    fn default() -> Self {
        Self {
            user_count: 2,
            end_date: Date::from_calendar_date(9999, Month::December, 31).unwrap(),
            signature: String::new(),
        }
    }
}

impl Licence {
    pub fn from_env() -> Self {
        if let Ok(json) = std::env::var("LICENCE") {
            serde_json::from_str(&json).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}
