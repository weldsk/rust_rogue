use super::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// データベース型
pub struct Database {
    pub terrain: Arc<RwLock<HashMap<String, terrain::Terrain>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            terrain: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}