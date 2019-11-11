use super::common::*;

/** エンティティ(プレイヤー,Mob) */
pub struct Entity {
    pub name: &'static str,
    pub hp: i32,
    pub attack: i32,
    pub defence: i32,
    pub speed: i32,
}