use std::collections::HashMap;

use crate::{actor::{Actor, Buff}, damage::{Damage, DamageType}};

use super::spells::SpellKind;


fn firebolt(power: u8, target: &mut Actor) {
    target.take_damage(Damage::new(power as i8 * 2, DamageType::Burn));
}

fn magic_missile(power: u8, target: &mut Actor) {
    let count = match power {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 4,
        4.. => 6
    };
    for _ in 0..count {
        target.take_damage(Damage::new(2, DamageType::Magic));
    }
}

fn shield(power: u8, target: &mut Actor) {
    target.add_buff(Buff::Shield(power as f32));
}

fn earth_shell(power: u8, target: &mut Actor) {
    target.add_buff(Buff::EarthShield(2 * power as i8));
}

pub struct CastEffects {
    effects: HashMap<SpellKind, Box<dyn FnMut(u8, &mut Actor)>>
}

fn capture_callback(callback: fn(u8, &mut Actor)) -> Box<dyn FnMut(u8, &mut Actor)> {
    Box::new(move |power: u8, target: &mut Actor| callback(power, target))
}

impl CastEffects {
    pub fn new() -> Self {
        let mut effects = HashMap::new();
        effects.insert(SpellKind::Firebolt, capture_callback(firebolt));
        effects.insert(SpellKind::MagicMissile, capture_callback(magic_missile));
        effects.insert(SpellKind::Shield, capture_callback(shield));
        effects.insert(SpellKind::EarthShield, capture_callback(earth_shell));
        Self { effects }
    }

    pub fn apply_effect(&mut self, kind: &SpellKind, power: u8, target: &mut Actor) {
        let Some(callback) = self.effects.get_mut(kind) else  {
            return;
        };

        callback(power, target);
    }
}
