use super::common::*;

/// Entity (Player,Mob)
pub trait Entity
{
    fn name(&self) -> &'static str;
    fn hp(&self) -> i32;
    fn attack(&self) -> i32;
    fn defence(&self) -> i32;
    fn is_player(&self) -> bool;
}
