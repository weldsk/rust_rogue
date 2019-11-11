use super::*;
use std::collections::HashMap;

/// データベース型
pub struct Database {
    pub terrain :HashMap<String, terrain::Terrain>,
    pub item    :HashMap<String, item::Item>,
    pub entity  :HashMap<String, entity::Entity>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            terrain :HashMap::new(),
            item    :HashMap::new(),
            entity  :HashMap::new(),
        }
    }
}
