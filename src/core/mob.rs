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
    fn walk(&mut self, core: &mut Core, dir: Direction) -> bool {
        let current_terrain = core.map.tile(self.position).terrain_mut();
        let next_position = self.position.move_to_dir(dir);
        let next_terrain = core.map.tile(next_position).terrain_mut();

        if (*current_terrain).is_able_to_get_out(
            &core, &*self,
            self.position, next_position, dir) {
            return false;
        } else {
            return true;
        }
        
    }
}