use super::*;
use super::common::*;

/// タイル(1マス)
pub struct Tile {
    pub terrain: terrain::Terrain,
}

/// マップ
pub struct Map (Vec<Vec::<Tile>>);

impl Map {
    #[allow(unused_variables)]
    /// コンストラクタ
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self(Vec::new())
    }

    /// タイルゲッタ
    pub fn get_tile(&mut self, pos: Position) -> &mut Tile {
        &mut self.0[pos.y as usize][pos.x as usize]
    }
}
