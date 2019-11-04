/** 方向 */
#[derive(PartialEq)]
pub enum Direction
{
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
    /// 瞬間移動(方向性を持たない)
    Jump,
}

impl Direction
{
    /// 全て
    pub fn all() -> Vec<Direction>
    {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpperLeft,
            Direction::UpperRight,
            Direction::LowerLeft,
            Direction::LowerRight
            ]
    }

    /// 無し
    pub fn none() -> Vec<Direction>
    {
        Vec::new() as Vec<Direction>
    }

    pub fn invert(&self) -> Self
    {
        match self
        {
            Direction::Up           => Direction::Down,
            Direction::Down         => Direction::Up,
            Direction::Left         => Direction::Right,
            Direction::Right        => Direction::Left,
            Direction::UpperLeft    => Direction::LowerRight,
            Direction::UpperRight   => Direction::LowerLeft,
            Direction::LowerLeft    => Direction::UpperRight,
            Direction::LowerRight   => Direction::UpperLeft,
            Direction::Jump         => Direction::Jump,
        }
    }
}

#[derive(PartialEq,Copy,Clone)]
pub struct Position
{
    pub x: u32,
    pub y: u32,
}
