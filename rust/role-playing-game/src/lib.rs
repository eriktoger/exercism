pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health != 0 {
            return None;
        }
        let new_mana = if self.level >= 10 { Some(100) } else { None };
        return Some(Player {
            health: 100,
            mana: new_mana,
            level: self.level,
        });
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            let new_helth = 0.max(self.health as i32 - mana_cost as i32);
            self.health = new_helth as u32;
            return 0;
        }
        let mana = self.mana.unwrap();
        if mana >= mana_cost {
            self.mana = Some(mana - mana_cost);
            return mana_cost * 2;
        }
        return 0;
    }
}
