use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct Container {
    pub name: String,
    pub comp: bool,
    pub env: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}
