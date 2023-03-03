use std::{fs::File};

use actor::Actor;
use magic::{spells::{SpellData, PreparedRunes}, cast_effects::CastEffects};
use serde::{Deserialize};

mod magic;
mod damage;
mod actor;


#[derive(Debug, Deserialize)]
struct RuneLevel {
    name: Vec<String>,
    description: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GameData {
    rune_levels: Vec<RuneLevel>,
    spell_data: Vec<SpellData>
}


impl GameData {
    fn load() -> Self {
        let input_path = format!("{}/assets/data.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(&input_path).expect("Failed opening file");
        ron::de::from_reader(f).expect("should be able to load game data")
    }

    fn get_rune_level(&self, level_index: usize) -> Option<&RuneLevel> {
        self.rune_levels.get(level_index)
    }

    fn get_rune_description(&self, level_index: usize, rune_index: u8) -> Option<String> {
        self.rune_levels.get(level_index).and_then(|rl| rl.description.get(rune_index as usize).cloned())
    }

    fn get_rune_name(&self, level_index: usize, rune_index: u8) -> Option<String> {
        self.rune_levels.get(level_index).and_then(|rl| rl.name.get(rune_index as usize).cloned())
    }
}

fn cast_spell(cast_effects: &mut CastEffects, spell_data: &[SpellData], prepared_runes: &PreparedRunes, caster: &mut Actor, enemy: &mut Actor) {
    let Some((kind, power, target)) = prepared_runes.get_spell(spell_data, caster, enemy) else {
        return;
    };

    cast_effects.apply_effect(kind, power, target);
}



fn main() {
    let mut rng = rand::thread_rng();

    let game_data = GameData::load();

    println!("Config: {:?}", &game_data);

    let mut caster = Actor::new(10, 10, 1);
    let mut enemy = Actor::new(10, 10, 1);

    let mut prepared_runes = PreparedRunes::new();
}
