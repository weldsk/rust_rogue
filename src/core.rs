#![allow(dead_code)]
pub mod map;
pub mod entity;
pub mod common;
pub mod terrain;
pub mod database;
use database::*;
use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
//static mut SYSTEM: &'static Option<Rc<Core>> = &None;
    pub static ref SYSTEM: Arc<Mutex<Core>> = Arc::new(Mutex::new(Core::new()));
}

pub struct Core
{
    pub terrain_db: database::Database<terrain::Terrain>,
}

impl Core
{
    pub fn new() -> Self
    {
        Core
        {
            terrain_db: Database::new(),
        }
    }
}