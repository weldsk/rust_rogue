/** 方向(複数) */
#[derive(PartialEq)]
pub enum Directions {
    /// 無し
    None,
    /// 全て
    All,
    /// 精密な複数方向
    StrictDirections(StrictDirections)
}
impl Directions {
    pub fn has_direction(&self, direction: &Direction) -> bool {
        if let Self::StrictDirections(strict_dir) = self {
            match direction {
                Direction::Up           => return strict_dir.up,
                Direction::Down         => return strict_dir.down,
                Direction::Left         => return strict_dir.left,
                Direction::Right        => return strict_dir.right,
                Direction::UpperLeft    => return strict_dir.upper_left,
                Direction::UpperRight   => return strict_dir.upper_right,
                Direction::LowerLeft    => return strict_dir.lower_left,
                Direction::LowerRight   => return strict_dir.lower_right,
                Direction::Jump         => return strict_dir.jump,
            };
        }
        if Self::All == *self {
            return true;
        }
        return false;
    }
}
impl Default for StrictDirections {
    fn default() -> Self {
        Self {
            up: false,
            down: false, 
            left: false, 
            right: false, 
            upper_left: false,
            upper_right: false,
            lower_left: false,
            lower_right: false,
            jump: false,
        }
    }
}
/** 精密な複数方向 */
#[derive(PartialEq)]
pub struct StrictDirections {
    /// 上
    pub up: bool,
    /// 下
    pub down: bool, 
    /// 左
    pub left: bool, 
    /// 右
    pub right: bool, 
    /// 左上
    pub upper_left: bool,
    /// 右上
    pub upper_right: bool,
    /// 左下
    pub lower_left: bool,
    /// 右下
    pub lower_right: bool,
    /// 瞬間移動(方向性を持たない)
    pub jump: bool,
}
impl StrictDirections {
    /// ホワイトリスト
    pub fn white_list() -> Self {
        Self {
            up: false,
            down: false, 
            left: false, 
            right: false, 
            upper_left: false,
            upper_right: false,
            lower_left: false,
            lower_right: false,
            jump: false,
        }
    }
    /// ブラックリスト
    pub fn black_list() -> Self {
        Self {
            up: true,
            down: true, 
            left: true, 
            right: true, 
            upper_left: true,
            upper_right: true,
            lower_left: true,
            lower_right: true,
            jump: true,
        }
    }
}
/** 方向 */
#[derive(PartialEq)]
pub enum Direction {
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
impl Direction {
    /// 反転
    pub fn invert(&self) -> Self {
        match self {
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
pub struct Position {
    pub x: u32,
    pub y: u32,
}