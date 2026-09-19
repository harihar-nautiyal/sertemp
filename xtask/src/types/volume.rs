use chrono::{DateTime, Utc};
use std::path::PathBuf;

pub struct Volume {
    pub name: String,
    pub path: PathBuf,
    pub created_at: DateTime<Utc>,
}
