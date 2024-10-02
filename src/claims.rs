use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub subject: String,
    pub name: String,
    pub iat: usize,
}

impl Claims {
    pub fn new(subject: &str, name: &str) -> Self {
        let iat = Utc::now() + Duration::hours(24);
        Claims {
            subject: subject.to_owned(),
            name: name.to_owned(),
            iat: iat.timestamp() as usize,
        }
    }
}
