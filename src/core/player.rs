use super::*;
use super::common::*;
use super::entity::*;

/// Player
pub struct Player {
    /// 名前
    name: &'static str,
    /// hp
    hp: i32,
    /// attack
    attack: u32,
    /// defence
    deffence: u32,
    /// 装備武器
    equiped_weapon: Option<weapon::Weapon>,
    /// 装備防具
    equiped_armor: Option<armor::Armor>,
    /// 所持武器
    weapons: Vec<weapon::Weapon>,
    /// 所持防具
    armors: Vec<armor::Armor>,
    /// 所持アイテム
    items: Vec<item::Item>,
    /// 現在位置
    position: Position,
}

impl Entity for Player {
    fn name(&self) -> &'static str {
        self.name
    }
    fn hp(&self) -> i32 {
        self.hp
    }
    fn is_player(&self) -> bool {
        true
    }
    fn walk(&self, core: Core, dir: Direction) -> bool {
        core.map.tile(self.position).terrain().is_able_to_get_out();
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "player",
            hp: 10,
            attack: 1,
            deffence: 0,
            equiped_weapon: None,
            equiped_armor: None,
            weapons: Vec::new(),
            armors: Vec::new(),
            items: Vec::new(),
        }
    }
}