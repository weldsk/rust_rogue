use super::*;
use std::collections::HashMap;

/// データベース型
pub struct Database {
    pub item    :HashMap<&'static str, item::Item>,
    pub entity  :HashMap<&'static str, mob::Mob>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            item    :HashMap::new(),
            entity  :HashMap::new(),
        }
    }
}
