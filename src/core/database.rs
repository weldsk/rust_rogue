use super::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;

/// データベース型
pub struct Database {
    pub terrain: Arc<RwLock<HashMap<String, terrain::Terrain>>>,
}

lazy_static! {
    pub static ref DATABASE: Database = Database::new();
}

impl Database {
    fn new() -> Self {
        Self {
            terrain: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}