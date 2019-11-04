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
//static mut SYSTEM: &'static Option<Rc<System>> = &None;
    pub static ref SYSTEM: Arc<Mutex<System>> = Arc::new(Mutex::new(System::new()));
}

pub struct System
{
    pub terrain_db: database::Database<terrain::Terrain>,
}

impl System
{
    pub fn new() -> Self
    {
        System
        {
            terrain_db: Database::new(),
        }
    }
}