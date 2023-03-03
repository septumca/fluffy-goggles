use std::{fs::File};

use actor::Actor;
use damage::{Damage, DamageType};
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


#[cfg(test)]
mod tests {
    use super::*;

    mod all_together {
        use crate::actor::{Actor, Buff};

        use super::*;

        fn prepare_rune_indexes(runes: &[u8], game_data: &GameData) -> PreparedRunes {
            let mut prepared_runes = PreparedRunes::new();

            let power_level_runes = game_data.get_rune_level(prepared_runes.get_act_rune_level()).unwrap();
            assert_eq!(power_level_runes.name, game_data.rune_levels[0].name);
            assert_eq!(power_level_runes.description, game_data.rune_levels[0].description);

            let Some(&power_rune) = runes.get(0) else {
                return prepared_runes;
            };
            prepared_runes.prepare_rune(power_rune);
            let power_level_runes = game_data.get_rune_level(prepared_runes.get_act_rune_level()).unwrap();
            assert_eq!(power_level_runes.name, game_data.rune_levels[1].name);
            assert_eq!(power_level_runes.description, game_data.rune_levels[1].description);

            let Some(&element_rune) = runes.get(1) else {
                return prepared_runes;
            };
            prepared_runes.prepare_rune(element_rune);
            let power_level_runes = game_data.get_rune_level(prepared_runes.get_act_rune_level()).unwrap();
            assert_eq!(power_level_runes.name, game_data.rune_levels[2].name);
            assert_eq!(power_level_runes.description, game_data.rune_levels[2].description);

            let Some(&form_rune) = runes.get(2) else {
                return prepared_runes;
            };
            prepared_runes.prepare_rune(form_rune);
            let power_level_runes = game_data.get_rune_level(prepared_runes.get_act_rune_level());
            assert!(power_level_runes.is_none());

            prepared_runes
        }

        #[test]
        fn magic_missile_power_0() {
            let game_data = GameData::load();
            let mut cast_effects = CastEffects::new();
            let mut caster = Actor::new(10, 10, 1);
            let mut enemy = Actor::new(10, 10, 1);

            let prepared_runes = prepare_rune_indexes(&[0, 2, 0], &game_data);
            cast_spell(&mut cast_effects, &game_data.spell_data, &prepared_runes, &mut caster, &mut enemy);

            assert_eq!(enemy.attributes.act_health, 8);
        }

        #[test]
        fn magic_missile_power_1_and_earth_shield_0() {
            let game_data = GameData::load();
            let mut cast_effects = CastEffects::new();
            let mut caster = Actor::new(10, 10, 1);
            let mut enemy = Actor::new(10, 10, 1);

            let prepared_runes = prepare_rune_indexes(&[0, 1, 1], &game_data);
            cast_spell(&mut cast_effects, &game_data.spell_data, &prepared_runes, &mut enemy, &mut caster);

            let prepared_runes = prepare_rune_indexes(&[1, 2, 0], &game_data);
            cast_spell(&mut cast_effects, &game_data.spell_data, &prepared_runes, &mut caster, &mut enemy);

            assert_eq!(enemy.attributes.act_health, 8);
        }

        #[test]
        fn magic_missile_power_3_and_shield_0() {
            let game_data = GameData::load();
            let mut cast_effects = CastEffects::new();
            let mut caster = Actor::new(10, 10, 1);
            let mut enemy = Actor::new(10, 10, 1);

            let prepared_runes = prepare_rune_indexes(&[0, 2, 1], &game_data);
            cast_spell(&mut cast_effects, &game_data.spell_data, &prepared_runes, &mut enemy, &mut caster);

            let prepared_runes = prepare_rune_indexes(&[3, 2, 0], &game_data);
            cast_spell(&mut cast_effects, &game_data.spell_data, &prepared_runes, &mut caster, &mut enemy);

            assert_eq!(enemy.attributes.act_health, 10);
        }

        #[test]
        fn firebolt_power_1() {
            let game_data = GameData::load();
            let mut cast_effects = CastEffects::new();
            let mut caster = Actor::new(10, 10, 1);
            let mut enemy = Actor::new(10, 10, 1);

            let prepared_runes = prepare_rune_indexes(&[1, 0, 0], &game_data);
            cast_spell(&mut cast_effects, &game_data.spell_data, &prepared_runes, &mut caster, &mut enemy);

            assert_eq!(enemy.attributes.act_health, 6);
        }

        #[test]
        fn earth_shield_power_2() {
            let game_data = GameData::load();
            let mut cast_effects = CastEffects::new();
            let mut caster = Actor::new(10, 10, 1);
            let mut enemy = Actor::new(10, 10, 1);

            let prepared_runes = prepare_rune_indexes(&[2, 1, 1], &game_data);
            cast_spell(&mut cast_effects, &game_data.spell_data, &prepared_runes, &mut caster, &mut enemy);

            assert_eq!(caster.get_buffs().len(), 1);
            assert_eq!(caster.get_buffs().get(0), Some(&Buff::EarthShield(6)));
        }
    }
}