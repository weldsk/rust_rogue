use super::*;
use std::collections::HashMap;

/// データベース型
pub struct Database {
    pub terrain :HashMap<&'static str, terrain::Terrain>,
    pub item    :HashMap<&'static str, item::Item>,
    pub entity  :HashMap<&'static str, entity::Entity>,
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
