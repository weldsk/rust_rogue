use super::*;
use super::entity::*;
use super::common::*;
/// Mob
pub struct Mob {
    name: &'static str,
    hp: i32,
    attack: i32,
    defence: i32,
    speed: i32,
    /// 現在位置
    position: Position,
}

impl Entity for Mob {
    fn name(&self) -> &'static str {
        self.name
    }
    fn hp(&self) -> i32 {
        self.hp
    }
    fn is_player(&self) -> bool{
        false
    }
    fn action(&mut self, core: &mut Core) {
    }
}