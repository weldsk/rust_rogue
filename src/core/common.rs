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
    pub fn has_direction(&self, direction: Direction) -> bool {
        // 精密な方向
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
        // 全て
        if Self::All == *self {
            return true;
        }
        // 無し
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
    /// ホワイトリスト(有効化したい方向をtrueにする)
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
    /// ブラックリスト(無効化したい方向をfalseにする)
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
#[derive(PartialEq, Clone, Copy)]
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
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn move_to_dir(&self, dir: Direction, range: i32) -> Position {
        let mut x = self.x;
        let mut y = self.y;
        match dir {
            Direction::Up           => y -= range,
            Direction::Down         => y += range,
            Direction::Left         => x -= range,
            Direction::Right        => x += range,
            Direction::UpperLeft    => { x-= range; y -= range },
            Direction::UpperRight   => { x+= range; y -= range },
            Direction::LowerLeft    => { x-= range; y += range },
            Direction::LowerRight   => { x+= range; y += range },
            _ => (),
        }

        Position {
            x: x,
            y: y,
        }
    }
}