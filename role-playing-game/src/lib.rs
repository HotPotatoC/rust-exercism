use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        Some(Player {
            health: 100,
            mana: if self.level < 10 { None } else { Some(100) },
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(m) => {
                if m < mana_cost {
                    return 0;
                }

                self.mana = Some(m - mana_cost);
                mana_cost * 2
            }
            None => {
                self.health -= min(self.health, mana_cost);

                0
            }
        }
    }
}
