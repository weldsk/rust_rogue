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
