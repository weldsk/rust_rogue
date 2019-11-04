#![allow(dead_code)]
pub mod map;
pub mod entity;
pub mod common;
pub mod terrain;
pub mod database;
use database::*;

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