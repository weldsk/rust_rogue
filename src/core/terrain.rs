use super::*;
use super::common::*;
use super::database::Database;
use super::entity::Entity;
use super::map::Map;

#[allow(unused_variables)]
fn is_allowed_get_in(db: &Database, entity: &Entity, map: &Map,
                     current_pos: Position, next_pos: Position, from_dir: Direction) -> bool {
    let next_terrain = &db.terrain[map.tile(next_pos).terrain_id];
    if next_terrain.allow_from_dir.has_direction(from_dir) {
        return true;
    }

    return false;
}

#[allow(unused_variables)]
fn is_allowed_get_out(db: &Database, entity: &Entity, map: &Map,
                      current_pos: Position, next_pos: Position, to_dir: Direction) -> bool {
    return true;
}

#[allow(unused_variables)]
fn get_in(db: &Database, entity: &mut Entity, map: &mut Map,
          current_pos: &mut Position, next_pos: &mut Position, from_dir: Direction) {
}

#[allow(unused_variables)]
fn keep(db: &Database, entity: &mut Entity, map: &mut Map,
        current_pos: &mut Position) {
}

#[allow(unused_variables)]
fn get_out(db: &Database, entity: &mut Entity, map: &mut Map,
           current_pos: &mut Position, next_pos: &mut Position, to_dir: Direction) {
}

/// 地形(allowの項目に一つでも合致していれば侵入可能)
pub struct Terrain {
    /// 侵入可能方向 
    allow_from_dir: Directions,

    /// 侵入可能判定関数
    /// (移動するエンティティ, マップ, 現在の位置, 移動先の位置, 侵入方向)
    /// -> bool: 侵入可能
    is_able_to_get_in:  fn(&Database, &Entity, &Map,  Position, Position, Direction)->bool,

    /// 侵出可能判定関数
    /// (移動するエンティティ, マップ, 現在の位置, 移動先の位置, 侵出方向)
    /// -> bool: 侵出可能
    is_able_to_get_out: fn(&Database, &Entity, &Map,  Position, Position, Direction)->bool,

    /// 侵入イベント(侵入前に実行される)
    /// (移動するエンティティ, マップ, 現在の位置, 移動先の位置, 侵入方向)
    get_in: fn(&Database, &mut Entity, &mut Map,  &mut Position, &mut Position, Direction),

    /// 待機時(侵入後動かなかった場合)イベント
    /// (移動するエンティティ, マップ, 現在の位置)
    keep:   fn(&Database, &mut Entity, &mut Map,  &mut Position),

    /// 侵出イベント(侵出前に実行される)
    /// (移動するエンティティ, マップ, 現在の位置, 移動先の位置, 侵出方向)
    get_out:fn(&Database, &mut Entity, &mut Map,  &mut Position, &mut Position, Direction),
}
impl Default for Terrain {
    fn default() -> Self {
        Self {
            allow_from_dir: Directions::All,
            is_able_to_get_in: is_allowed_get_in,
            is_able_to_get_out: is_allowed_get_out,
            get_in: get_in,
            keep: keep,
            get_out: get_out,
        }
    }
}