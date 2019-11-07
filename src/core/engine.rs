use super::*;

pub struct Engine {
    database: database::Database,
}

impl Engine {
    pub fn new(database: database::Database) -> Self {
        Self {
            database: database,
        }
    }
}
