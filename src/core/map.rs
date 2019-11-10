use super::*;
use super::common::*;

/// タイル(1マス)
pub struct Tile {
    pub terrain: terrain::Terrain,
}

/// マップ
pub struct Map {
    /// タイル
    tiles: Vec<Vec::<Tile>>,
    /// 開始位置
    start_pos: Position
}

impl Map {
    #[allow(unused_variables)]
    /// コンストラクタ
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self{
            tiles: Vec::new(),
            start_pos: Position{x:0, y:0}
        }
    }

    /// タイル(imutable)
    pub fn tile(&self, pos: Position) -> &Tile {
        &self.tiles[pos.y as usize][pos.x as usize]
    }
    /// タイル(mutable)
    pub fn tile_mut(&mut self, pos: Position) -> &mut Tile {
        &mut self.tiles[pos.y as usize][pos.x as usize]
    }
}
