use crate::{Actor, token::Token};

pub type EffectsToPerform =  (Vec<Box<dyn Effect>>, Vec<Box<dyn Effect>>);

pub trait Effect {
    fn execute(&self, actor: &mut Actor);
}

pub struct GenericEffectValue {
    token: Token,
    value: u8
}

impl GenericEffectValue {
    pub fn new(token: Token, value: u8) -> Self {
        Self { token, value }
    }
}

pub enum GenericEffect {
    AddToken(GenericEffectValue),
    RemoveToken(GenericEffectValue)
}

impl Effect for GenericEffect {
    fn execute(&self, actor: &mut Actor) {
        match self {
            Self::AddToken(tv) => {
                if let Some(value) = actor.tokens.get_mut(&tv.token) {
                    *value = *value + tv.value;
                } else {
                    actor.tokens.insert(tv.token.clone(), tv.value);
                }
            },
            Self::RemoveToken(tv) => {
                let Some(value) = actor.tokens.get_mut(&tv.token) else {
                    return;
                };
                if *value > tv.value {
                    *value = *value - tv.value;
                } else {
                    let _ = actor.tokens.remove(&tv.token);
                }
            }
        }
    }
}