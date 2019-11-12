use super::*;
use super::entity::*;
/// Mob
pub struct Mob {
    name: &'static str,
    hp: i32,
    attack: i32,
    defence: i32,
    speed: i32,
}

impl Entity for Mob {
    fn get_name(&self) -> &'static str {
        self.name
    }
    fn get_hp(&self) -> i32 {
        self.hp
    }
    fn attack(&mut self, entity: &mut impl Entity) -> i32 {
        self.attack
    }
    fn is_player(&self) -> bool{
        false
    }
}