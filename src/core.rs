#![allow(dead_code)]
pub mod common;
pub mod database;
pub mod engine;
pub mod terrain;
pub mod map;
pub mod entity;
pub mod player;
pub mod mob;
pub mod item;
pub mod weapon;
pub mod armor;

pub struct Core {
    pub database: &'static database::Database,
    pub map: map::Map,
    pub player: player::Player,
}

pub enum CoreInput {
    /// 上
    Up,
    /// 下
    Down,
    /// 左
    Left,
    /// 右
    Right,
    /// 左上
    UpperLeft,
    /// 右上
    UpperRight,
    /// 左下
    LowerLeft,
    /// 右下
    LowerRight,
    /// 終了
    Quit,
}

pub enum CoreOutput {
    /// 入力
    Input,
    /// 待機(ミリ秒)
    Sleep(i32),
    /// 画面更新
    Update,
}

impl Core {
    pub fn new(db: &'static database::Database, player: player::Player) -> Self {
        Self {
            database: db,
            map: map::Map::new(1,1),
            player: player,
        }
    }
    pub fn next(input: Option<CoreInput>) -> CoreOutput {
        return CoreOutput::Update;
    }
}
