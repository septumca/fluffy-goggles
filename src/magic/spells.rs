use serde::Deserialize;

use crate::{Actor};


#[derive(Clone, Debug, Deserialize)]
pub enum SpellTarget {
    Caster,
    Enemy,
}

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum SpellKind {
    Firebolt,
    EarthShield,
    Shield,
    MagicMissile,
}

pub struct PreparedRunes {
    runes: Vec<u8>
}

#[derive(Debug, Deserialize)]
pub struct SpellData {
    rune_indexes: Vec<u8>,
    kind: SpellKind,
    target: SpellTarget,
}

impl PreparedRunes {
    pub fn new() -> Self {
        Self {
            runes: vec![]
        }
    }

    pub fn prepare_rune(&mut self, rune: u8) {
        self.runes.push(rune);
    }

    pub fn remove_rune(&mut self) {
        self.runes.pop();
    }

    pub fn get_act_rune_level(&self) -> usize {
        self.runes.len()
    }

    fn get_power(&self) -> u8 {
        self.runes.get(0).and_then(|&v| Some(v+1)).unwrap_or(0)
    }

    pub fn get_spell<'a>(&self, spells: &'a [SpellData], caster: &'a mut Actor, enemy: &'a mut Actor) -> Option<(&'a SpellKind, u8, &'a mut Actor)> {
        if self.runes.len() <= 1 {
            return None;
        }
        let Some(spell_data) = spells
            .iter()
            .find(|s| s.rune_indexes == &self.runes[1..]) else
        {
            return None;
        };

        let target = match spell_data.target {
            SpellTarget::Caster => caster,
            SpellTarget::Enemy => enemy,
        };

        Some((
            &spell_data.kind,
            self.get_power(),
            target
        ))
    }
}
