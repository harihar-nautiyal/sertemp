use chrono::{DateTime, Utc};
use std::path::PathBuf;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Volume {
    pub name: String,
    pub path: PathBuf,
    pub created_at: DateTime<Utc>,
}
