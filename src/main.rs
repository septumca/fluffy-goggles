use std::{fs::File, collections::HashMap};

use serde::{Deserialize};


pub enum SpellEffect {
    Shield,
    EarthShield
}

#[derive(Hash, PartialEq, Eq)]
pub enum Attribute {
    Health(i8),
    Energy(i8),
    EnergyRegeneration(i8),
}

struct AttributeModifier {
    attribute: Attribute,
    value: i8,
    duration: f32,
}

struct Actor {
    attributes: HashMap<Attribute, i8>,
    attributes_modifiers: Vec<AttributeModifier>,
    modifiers: Vec<SpellEffect>
}

impl Actor {
    pub fn new(health: i8, energy: i8, energy_regenration: i8) -> Self {
        Self {
            attributes: HashMap::from([
                (Attribute::Health(health), health),
                (Attribute::Energy(energy), energy),
                (Attribute::EnergyRegeneration(energy_regenration), energy_regenration)
            ]),
            attributes_modifiers: vec![],
            modifiers: vec![]
        }
    }
}

struct PreparedRunes(Vec<u8>);

#[derive(Debug, Deserialize)]
struct RuneLevel {
    count: u8,
    name: Vec<String>,
    description: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SpellData {
    rune_indexes: Vec<u8>,
    kind: SpellKind,
}

impl SpellData {
    pub fn does_match_runes(&self, prepared_runes: &[u8]) -> bool {
        prepared_runes.len() == self.rune_indexes.len() &&
        self.rune_indexes
            .iter()
            .enumerate()
            .all(|(idx, value)| *value == prepared_runes[idx])
    }
}

#[derive(Debug, Deserialize)]
struct GameData {
    rune_levels: Vec<RuneLevel>,
    spells: Vec<SpellData>
}


impl GameData {
    pub fn get_matching_spell(&self, prepared_runes: &[u8]) -> Option<&SpellKind> {
        self.spells
            .iter()
            .find(|s| s.does_match_runes(prepared_runes))
            .and_then(|s| Some(&s.kind))
    }

    fn get_next_rune_level(&self, act_index: Option<usize>) -> Option<&RuneLevel> {
        match act_index {
            None => Some(&self.rune_levels[0]),
            Some(rl) if rl < self.rune_levels.len() - 1 => Some(&self.rune_levels[rl+1]),
            Some(rl) => None
        }
    }

    fn get_rune_description(&self, level_index: usize, rune_index: u8) -> Option<String> {
        self.rune_levels.get(level_index).and_then(|rl| rl.description.get(rune_index as usize).cloned())
    }

    fn get_rune_name(&self, level_index: usize, rune_index: u8) -> Option<String> {
        self.rune_levels.get(level_index).and_then(|rl| rl.name.get(rune_index as usize).cloned())
    }
}


#[derive(Debug, Deserialize)]
enum SpellKind {
    Fireball,
    EarthShield,
    Shield,
    MagicMissile,
}

impl SpellKind {
    pub fn apply(&self, power: u8, source: &mut Actor, target: &mut Actor) {
        todo!()
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let input_path = format!("{}/assets/data.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(&input_path).expect("Failed opening file");
    let game_data: GameData = ron::de::from_reader(f).expect("should be able to load game data");

    println!("Config: {:?}", &game_data);
}