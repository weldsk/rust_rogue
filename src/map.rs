pub enum Direction
{
    Up,
    Down,
    Right,
    Left,
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

impl Direction
{
    fn AllDirection() -> Vec<Direction>
    {
        vec![Direction::Up,Direction::Down,Direction::Right,Direction::Left,Direction::UpperLeft,Direction::UpperRight,Direction::LowerLeft,Direction::LowerRight]
    }
}

pub struct TerrainType
{
    allow_from_dir: Vec<Direction>,
    allow_to_dir: Vec<Direction>,
}

pub struct Terrain
{
    terrain_type: TerrainType,
}

pub struct Map (Vec<Vec::<Terrain>>);