use super::*;

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