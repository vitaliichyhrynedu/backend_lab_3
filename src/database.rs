use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

pub type Database<T> = Arc<RwLock<HashMap<Uuid, T>>>;

pub fn database<T>() -> Database<T> {
    Arc::new(RwLock::new(HashMap::new()))
}
