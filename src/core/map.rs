use super::*;
use super::common::*;

/// タイル(1マス)
pub struct Tile {
    terrain: Box<dyn terrain::Terrain>,
}
impl Default for Tile {
    fn default() -> Self {
        Self {
            terrain: Box::new(terrain::DefaultTerrain()),
        }
    }
}
impl Tile {
    pub fn terrain(&self) -> &dyn terrain::Terrain {
        &*(self.terrain)
    }
    pub fn terrain_mut(&mut self) -> &mut dyn terrain::Terrain {
        &mut *(self.terrain)
    }
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
        let mut map = Self {
            tiles: Vec::new(),
            start_pos: Position{x:0, y:0}
        };
        for i in 0..y_size {
            // 行追加
            map.tiles.push(Vec::new());
            for j in 0.. x_size {
                // 列追加
                map.tiles[i].push(Default::default());
            }
        };
        return map;
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
