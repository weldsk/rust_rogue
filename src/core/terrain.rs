use super::*;
use super::common::*;
use super::database::Database;
use super::entity::Entity;
use super::map::Map;

#[allow(unused_variables)]
fn is_allowed_get_in(db: &Database, entity: &impl Entity, map: &Map,
                     current_pos: Position, next_pos: Position, from_dir: Direction)
                     -> bool {
    //let next_terrain = &db.terrain.unwrap().get
    //if next_terrain.allow_from_dir.has_direction(from_dir) {
    //    return true;
    //}

    return false;
}

#[allow(unused_variables)]
fn is_allowed_get_out(db: &Database, entity: &impl Entity, map: &Map,
                      current_pos: Position, next_pos: Position, to_dir: Direction)
                      -> bool {
    return true;
}

#[allow(unused_variables)]
fn get_in(db: &Database, entity: &mut impl Entity, map: &mut Map,
          current_pos: &mut Position, next_pos: &mut Position, from_dir: Direction) {
}

#[allow(unused_variables)]
fn keep(db: &Database, entity: &mut impl Entity, map: &mut Map,
        current_pos: &mut Position) {
}

#[allow(unused_variables)]
fn get_out(db: &Database, entity: &mut impl Entity, map: &mut Map,
           current_pos: &mut Position, next_pos: &mut Position, to_dir: Direction) {
}

pub trait Terrain {
    /// 侵入可能方向 
    fn allow_from_dir(&self) -> Direction;
    /// 侵入可能判定関数
    /// (移動するエンティティ, マップ, 現在の位置, 移動先の位置, 侵入方向)
    /// -> bool: 侵入可能
    
    fn is_able_to_get_in (&self, db: &Database, entity: &mut dyn Entity, map: &Map,
        current_pos: Position, next_pos: Position, from_dir: Direction)->bool;

    /// 侵出可能判定関数
    /// (移動するエンティティ, マップ, 現在の位置, 移動先の位置, 侵出方向)
    /// -> bool: 侵出可能
    fn is_able_to_get_out(&self, db: &Database, entity: &mut dyn Entity, map: &Map, current_pos: Position, next_pos: Position, to_dir: Direction)->bool;

    /// 侵入イベント(侵入前に実行される)
    /// (移動するエンティティ, マップ, 現在の位置, 移動先の位置, 侵入方向)
    fn get_in(&self, db: &Database, entity: &mut dyn Entity, map: &mut Map, current_pos: Position, next_pos: Position, from_dir: Direction);

    /// 待機時(侵入後動かなかった場合)イベント
    /// (移動するエンティティ, マップ, 現在の位置)
    fn keep(&self, db: &Database, entity: &mut dyn Entity, map: &mut Map, current_pos: &mut Position);

    /// 侵出イベント(侵出前に実行される)
    /// (移動するエンティティ, マップ, 現在の位置, 移動先の位置, 侵出方向)
    fn get_out(&self, db: &Database, entity: &mut dyn Entity, map: &mut Map, current_pos: Position, next_pos: Position, to_dir: Direction);

}