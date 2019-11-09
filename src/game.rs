mod map;
mod config;
use crate::*;

/// データベースへ読み込み
pub fn database(database: &mut core::database::Database) -> &mut core::database::Database
{
    // ロード
    return database;
}

/// マップ生成
pub fn generate_map(level: i32) -> core::map::Map
{
    map::generate(config::MAP_SIZE_X, config::MAP_SIZE_Y)
}