use crate::damage::{DamageType, Damage};


#[derive(Debug, PartialEq)]
pub enum Buff {
    Shield(f32),
    EarthShield(i8)
}

pub struct Attributes {
    pub max_health: i8,
    pub act_health: i8,
    pub max_energy: i8,
    pub act_energy: i8,
    pub energy_regeneration: i8,
}


pub struct Actor {
    pub attributes: Attributes,
    buffs: Vec<Buff>
}

impl Actor {
    pub fn new(health: i8, energy: i8, energy_regenration: i8) -> Self {
        Self {
            attributes: Attributes {
                max_health: health,
                act_health: health,
                max_energy: energy,
                act_energy: energy,
                energy_regeneration: energy_regenration
            },
            buffs: vec![]
        }
    }

    pub fn get_buffs(&self) -> &[Buff] {
        &self.buffs
    }

    pub fn add_buff(&mut self, active_spell: Buff) {
        if let Some(existing_index) = self.buffs
            .iter()
            .enumerate()
            .find_map(|(index, actor_active_spell)| {
                if std::mem::discriminant(&active_spell) == std::mem::discriminant(actor_active_spell) {
                    return Some(index);
                }
                None
            })
        {
            self.buffs.remove(existing_index);
        }
        self.buffs.push(active_spell);
    }

    pub fn process_damage(&mut self, mut damage: Damage) -> Damage {
        self.buffs.iter_mut().for_each(|active_spell| {
            match active_spell {
                Buff::EarthShield(dr) => {
                    let new_value = (damage.value - *dr).max(0);
                    *dr = (*dr - damage.value).max(0);
                    damage.value = new_value;
                },
                Buff::Shield(_) if damage.damage_type == DamageType::Magic => {
                    damage.value = 0;
                },
                _ => ()
            }
        });
        damage
    }

    pub fn take_damage(&mut self, damage: Damage) {
        let damage = self.process_damage(damage);
        self.attributes.act_health = (self.attributes.act_health - damage.value).max(0);
    }
}


#[cfg(test)]
mod actor {
    use crate::actor::{Actor, Buff};

    use super::*;

    #[test]
    fn more_same_active_abilities() {
        let mut a = Actor::new(10, 10, 10);

        a.add_buff(Buff::EarthShield(2));
        a.add_buff(Buff::EarthShield(1));
        assert_eq!(a.buffs.len(), 1);
        assert_eq!(a.buffs.get(0), Some(&Buff::EarthShield(1)));
    }
}