use crate::*;
use crate::game::*;

#[allow(unused_variables)]
fn is_allowed_get_in(entity: &entity::Entity, current_tile: &map::Tile, next_tile: &map::Tile, from_dir: &Direction) -> bool
{
    if next_tile.terrain.allow_from_dir.contains(&from_dir)
    {
        return true;
    }

    return false;
}

#[allow(unused_variables)]
fn is_allowed_get_out(entity: &entity::Entity, current_tile: &map::Tile, next_tile: &map::Tile, to_dir: &Direction) -> bool
{
    return true;
}

#[allow(unused_variables)]
fn get_in(entity: &mut entity::Entity, current_tile: &mut map::Tile, next_tile: &mut map::Tile, from_dir: &Direction)
{
}

#[allow(unused_variables)]
fn keep(entity: &mut entity::Entity, current_tile: &mut map::Tile)
{
}

#[allow(unused_variables)]
fn get_out(entity: &mut entity::Entity, current_tile: &mut map::Tile, next_tile: &mut map::Tile, to_dir: &Direction)
{
}

/// 地形(allowの項目に一つでも合致していれば侵入可能)
pub struct Terrain
{
    /// 侵入可能方向 
    allow_from_dir: Vec<Direction>,
    /// 侵入可能判定関数
    /// (1st_arg: 移動するエンティティ, 2st_arg:現在のタイル, 3nd_arg:移動先のタイル, 4rd_arg:侵入方向)
    /// -> bool: 侵入可能
    is_able_to_get_in: fn(&entity::Entity, &map::Tile, &map::Tile, &Direction)->bool,
    /// 侵出可能判定関数
    /// (1st_arg: 移動するエンティティ, 2st_arg:現在のタイル, 3nd_arg:移動先のタイル, 4rd_arg:侵出方向)
    /// -> bool: 侵出可能
    is_able_to_get_out: fn(&entity::Entity, &map::Tile, &map::Tile, &Direction)->bool,
    /// 侵入イベント(侵入前に実行される)
    /// (1st_arg: 移動するエンティティ, 2st_arg:現在のタイル, 3nd_arg:移動先のタイル, 4rd_arg:侵入方向)
    get_in: fn(&mut entity::Entity, &mut map::Tile, &mut map::Tile, &Direction),
    /// 待機時(侵入後動かなかった場合)イベント
    /// (1st_arg: 移動するエンティティ, 2st_arg:現在のタイル)
    keep: fn(&mut entity::Entity, &mut map::Tile),
    /// 侵出イベント(侵出前に実行される)
    /// (1st_arg: 移動するエンティティ, 2st_arg:現在のタイル, 3nd_arg:移動先のタイル, 4rd_arg:侵出方向)
    get_out: fn(&mut entity::Entity, &mut map::Tile, &mut map::Tile, &Direction),
}
impl Default for Terrain
{
    fn default() -> Self
    {
        Self {
            allow_from_dir: Direction::all(),
            is_able_to_get_in: is_allowed_get_in,
            is_able_to_get_out: is_allowed_get_out,
            get_in: get_in,
            keep: keep,
            get_out: get_out,
        }
    }
}