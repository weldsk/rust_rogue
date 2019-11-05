use crate::core::*;

pub fn game()
{
    let core = SYSTEM.clone();
    let mut core_mut = core.lock().unwrap();
    core_mut.terrain_db.insert("Default", terrain::Terrain::default());
}