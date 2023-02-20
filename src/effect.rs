use crate::{Actor, token::Token};

pub type EffectsToPerform =  (Vec<Box<dyn Effect>>, Vec<Box<dyn Effect>>);

pub trait Effect {
    fn execute(&self, actor: &mut Actor);
}

pub struct AddTokenEffect {
    token: Token,
    value: u8,
}

impl AddTokenEffect {
    pub fn new(token: Token, value: u8) -> Self {
        Self { token, value }
    }
}

impl Effect for AddTokenEffect {
    fn execute(&self, actor: &mut Actor) {
        if let Some(value) = actor.tokens.get_mut(&self.token) {
            *value = *value + self.value;
        } else {
            actor.tokens.insert(self.token.clone(), self.value);
        }
    }
}

pub struct RemoveTokenEffect {
    token: Token,
    value: u8,
}

impl RemoveTokenEffect {
    pub fn new(token: Token, value: u8) -> Self {
        Self { token, value }
    }
}


impl Effect for RemoveTokenEffect {
    fn execute(&self, actor: &mut Actor) {
        let Some(value) = actor.tokens.get_mut(&self.token) else {
            return;
        };
        if *value > self.value {
            *value = *value - self.value;
        } else {
            let _ = actor.tokens.remove(&self.token);
        }
    }
}