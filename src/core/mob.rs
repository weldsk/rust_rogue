use super::*;
use super::entity::*;
/// Mob
pub struct Mob {
    pub name: &'static str,
    pub hp: i32,
    pub attack: i32,
    pub defence: i32,
    pub speed: i32,
}

impl Entity for Mob {
    fn name(&self) -> &'static str {
        self.name
    }
    fn hp(&self) -> i32 {
        self.hp
    }
    fn attack(&self) -> i32 {
        self.attack
    }
    fn defence(&self) -> i32 {
        self.defence
    }
    fn is_player(&self) -> bool{
        false
    }
}