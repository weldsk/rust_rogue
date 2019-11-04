use crate::system::*;

pub fn game()
{
    let system = SYSTEM.clone();
    let mut system_mut = system.lock().unwrap();
    system_mut.terrain_db.insert("Default", terrain::Terrain::default());
}