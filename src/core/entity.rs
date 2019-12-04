use super::common::*;

/// Entity (Player,Mob)
pub trait Entity
{
    fn get_name(&self) -> &'static str;
    fn get_hp(&self) -> i32;
    fn is_player(&self) -> bool;
}
