use std::collections::HashMap;

use crate::{token::{TokenWithValue, Token, TokenContainer}, effect::{GenericEffect, Effect}};

pub type TriggerList = Vec<Box<dyn Triggerable>>;

#[derive(Hash, Eq, PartialEq, Debug, Clone, PartialOrd, Ord)]
pub enum TriggerCondition {
    RoundStart,
    RoundEnd,
    TurnStart,
    TurnEnd,
}

pub trait Triggerable {
    fn get_description(&self) -> String;
    fn is_alive(&self) -> bool;
    fn execute(&self, actor: &mut TokenContainer);
}

pub struct ApplyDamageTrigger {}

impl Triggerable for ApplyDamageTrigger {
    fn execute(&self, tokens: &mut TokenContainer) {
        let damage = tokens.get(&Token::Damage).unwrap_or(&0);
        let health = tokens.get(&Token::Health).unwrap_or(&0);

        if damage >= health {
            let remove_health_effect = GenericEffect::RemoveToken(TokenWithValue::new(Token::Health, *damage));
            let remove_damage_effect = GenericEffect::RemoveToken(TokenWithValue::new(Token::Damage, *damage));
            println!("Removing {damage} health");
            remove_health_effect.execute(tokens);
            remove_damage_effect.execute(tokens);
        }
    }

    fn get_description(&self) -> String {
        "Apply damage to health".into()
    }

    fn is_alive(&self) -> bool {
        true
    }
}


pub struct TriggerContainer {
    triggers: HashMap<TriggerCondition, TriggerList>
}

impl TriggerContainer {
    pub fn new() -> Self {
        Self { triggers: HashMap::new() }
    }

    pub fn add_trigger(&mut self, condition: TriggerCondition, trigger: Box<dyn Triggerable>) {
        self.triggers
            .entry(condition)
            .or_insert(vec![])
            .push(trigger);
    }

    pub fn execute_triggers(&self, condition: &TriggerCondition, tokens: &mut TokenContainer) {
        let Some(triggers) = self.triggers.get(&condition) else {
            return;
        };

        triggers
            .iter()
            .for_each(|t| {
                t.execute(tokens);
            });
    }

    pub fn clean(&mut self, condition: &TriggerCondition) {
        let Some(triggers) = self.triggers.get_mut(condition) else {
            return;
        };

        triggers.retain(|t| {
            t.is_alive()
        });

        if triggers.is_empty() {
            self.triggers.remove(condition);
        }
    }
}