pub mod container;
pub mod volume;

pub use container::*;
pub use volume::*;

use chrono::Utc;
use std::collections::HashMap;
use std::sync::LazyLock;

pub static DEFAULT_CONTAINER: LazyLock<Container> = LazyLock::new(|| {
    Container::builder()
        .name("web-app".to_string())
        .comp(true)
        .env(HashMap::from([(
            "RUST_LOG".to_string(),
            "info".to_string(),
        )]))
        .created_at(Utc::now())
        .volumes(vec![])
        .run("./run.sh".to_string())
        .build()
});
