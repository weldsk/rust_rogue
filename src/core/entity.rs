use super::common::*;
use super::*;

/// Entity (Player,Mob)
pub trait Entity
{
    fn name(&self) -> &'static str;
    fn hp(&self) -> i32;
    fn is_player(&self) -> bool;
    fn walk(&mut self, core: &mut Core, dir: Direction) -> bool;
}
