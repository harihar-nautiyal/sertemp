use crate::types::volume::Volume;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Container {
    pub name: String,
    pub comp: bool,
    pub env: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub volumes: Vec<Volume>,
    pub run: String,
}
