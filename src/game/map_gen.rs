use crate::system::*;
use crate::system::map::*;

pub fn generate(x_size: usize, y_size: usize) -> Map
{
    Map::new(x_size, y_size)
}