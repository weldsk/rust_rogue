use super::*;
use super::common::*;
use super::database::Database;
use super::entity::Entity;
use super::map::Map;

/*
#[allow(unused_variables)]
fn is_allowed_get_in(self_terrain: &dyn terrain::Terrain, db: &Database, entity: &dyn Entity, map: &mut Map,
                     current_pos: Position, next_pos: Position, from_dir: Direction)
                     -> bool {
    //let next_terrain = *(&self, db.terrain.get(map.tile(next_pos).terrain_id).unwrap()) as dyn terrain::Terrain;
    let current_terrain: &dyn terrain::Terrain = &(**(db.terrain.get(map.tile(next_pos).terrain_id).unwrap()));
    let a = current_terrain.allow_from_dir().has_direction(from_dir);
    let next_terrain = self_terrain;
    //if next_terrain.allow_from_dir.has_direction(from_dir) {
    //    return true;
    //}

    return false;
}

#[allow(unused_variables)]
fn is_allowed_get_out(self_terrain: &dyn terrain::Terrain, db: &Database, entity: &dyn Entity, map: &Map,
                      current_pos: Position, next_pos: Position, to_dir: Direction)
                      -> bool {

    return true;
}

#[allow(unused_variables)]
fn get_in(self_terrain: &dyn terrain::Terrain, db: &Database, entity: &mut dyn Entity, map: &mut Map,
          current_pos: &mut Position, next_pos: &mut Position, from_dir: Direction) {
}

#[allow(unused_variables)]
fn keep(self_terrain: &dyn terrain::Terrain, db: &Database, entity: &mut dyn Entity, map: &mut Map,
        current_pos: &mut Position) {
}

#[allow(unused_variables)]
fn get_out(self_terrain: &dyn terrain::Terrain, db: &Database, entity: &mut dyn Entity, map: &mut Map,
           current_pos: &mut Position, next_pos: &mut Position, to_dir: Direction) {
}
*/

pub struct DefaultTerrain();
#[allow(unused_variables)]
impl Terrain for DefaultTerrain {
    fn allow_from_dir(&self) -> Directions {
        Directions::None
    }
    fn is_able_to_get_in (&self, core: &Core, entity: &dyn Entity, current_pos: Position, next_pos: Position, from_dir: Direction)->bool {
        self.allow_from_dir().has_direction(from_dir)
    }
    fn is_able_to_get_out (&self, core: &Core, entity: &dyn Entity, current_pos: Position, next_pos: Position, to_dir: Direction)->bool {
        self.allow_from_dir().has_direction(to_dir)
    }
    fn get_in(&mut self, db: &Database, entity: &mut dyn Entity, map: &mut Map, current_pos: Position, next_pos: Position, from_dir: Direction) {}
    fn keep(&mut self, db: &Database, entity: &mut dyn Entity, map: &mut Map, current_pos: &mut Position) {}
    fn get_out(&mut self, db: &Database, entity: &mut dyn Entity, map: &mut Map, current_pos: Position, next_pos: Position, to_dir: Direction) {}
}

pub trait Terrain {
    /// 侵入可能方向 
    fn allow_from_dir(&self) -> Directions;
    /// 侵入可能判定関数
    /// -> bool: 侵入可能
    
    fn is_able_to_get_in (&self, core: &Core, entity: &dyn Entity, current_pos: Position, next_pos: Position, from_dir: Direction)->bool;

    /// 侵出可能判定関数
    /// -> bool: 侵出可能
    fn is_able_to_get_out (&self, core: &Core, entity: &dyn Entity, current_pos: Position, next_pos: Position, to_dir: Direction)->bool;

    /// 侵入イベント(侵入前に実行される)
    fn get_in(&mut self, db: &Database, entity: &mut dyn Entity, map: &mut Map, current_pos: Position, next_pos: Position, from_dir: Direction);

    /// 待機時(侵入後動かなかった場合)イベント
    fn keep(&mut self, db: &Database, entity: &mut dyn Entity, map: &mut Map, current_pos: &mut Position);

    /// 侵出イベント(侵出前に実行される)
    fn get_out(&mut self, db: &Database, entity: &mut dyn Entity, map: &mut Map, current_pos: Position, next_pos: Position, to_dir: Direction);

}