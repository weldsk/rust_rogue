/** 方向 */
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

pub fn in_event_default(current_tile:Tile, next_tile:Tile, from_dir: Direction)
{
}

#[allow(unused_variables)]
pub fn on_event_default(current_tile:Tile)
{
}

pub fn out_event_default(current_tile:Tile, next_tile:Tile, to_dir: Direction)
{
}

/// 地形(allowの項目に一つでも合致していれば侵入可能)
pub struct Terrain
{
    /// 侵入可能方向 
    allow_from_dir: Vec<Direction>,
    /// 侵入イベント(侵入前に実行される)
    /// 1st_arg:現在のタイル, 2nd_arg:移動先のタイル, 3rd_arg:移動先方向
    in_envet: fn(Tile, Tile, Direction),
    /// 待機時(侵入後動かなかった場合)イベント
    /// arg:現在のタイル
    keep_event: fn(Tile),
    /// 侵出イベント(侵出前に実行される)
    /// 1st_arg:現在のタイル, 2nd_arg:移動先のタイル, 3rd_arg:移動先方向
    out_event: fn(Tile, Tile),
}

/// タイル(1マス)
pub struct Tile
{
    terrain: Terrain,
}

/// マップ
pub struct Map (Vec<Vec::<Tile>>);

#[test]
fn test()
{
    Direction::all();
}