use crate::{token::{TokenWithValue, TokenContainer}};

pub type EffectsToPerform =  (Vec<Box<dyn Effect>>, Vec<Box<dyn Effect>>);

pub trait Effect {
    fn execute(&self, tokens: &mut TokenContainer);
}

pub enum GenericEffect {
    AddToken(TokenWithValue),
    RemoveToken(TokenWithValue)
}

impl Effect for GenericEffect {
    fn execute(&self, tokens: &mut TokenContainer) {
        match self {
            Self::AddToken(tv) => {
                if let Some(value) = tokens.get_mut(&tv.token()) {
                    *value = *value + tv.value();
                } else {
                    tokens.insert(tv.token().clone(), tv.value());
                }
            },
            Self::RemoveToken(tv) => {
                let Some(value) = tokens.get_mut(&tv.token()) else {
                    return;
                };
                if *value > tv.value() {
                    *value = *value - tv.value();
                } else {
                    let _ = tokens.remove(&tv.token());
                }
            }
        }
    }
}