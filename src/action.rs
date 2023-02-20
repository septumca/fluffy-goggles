use std::collections::BTreeMap;

use crate::{effect::{EffectsToPerform, AddTokenEffect, RemoveTokenEffect}, token::{TokenContainer, Token}, damage::DmgRange, Actor};


fn can_perform_action(data: &ActionData, source: &Actor, target: &Actor) -> bool {
  data.required_tokens_source
      .iter()
      .all(|(key, val)| {
          source.tokens.get(key).unwrap_or(&0) >= val
      })
  ||
  data.required_tokens_target
      .iter()
      .all(|(key, val)| {
          target.tokens.get(key).unwrap_or(&0) >= val
      })
}

pub trait Action {
  fn can_perform(&self, source: &Actor, target: &Actor) -> bool;
  fn perform(&self) -> EffectsToPerform;
}

struct ActionData {
  name: String,
  required_tokens_source: TokenContainer,
  required_tokens_target: TokenContainer,
}

impl ActionData {
  fn with_name(name: String) -> Self {
      Self {
          name,
          required_tokens_source: BTreeMap::new(),
          required_tokens_target: BTreeMap::new()
      }
  }
}

pub struct SloppyJab {
  damage: DmgRange<u8>,
  data: ActionData
}

impl SloppyJab {
  pub fn new() -> Self {
      Self {
          damage: DmgRange::new(1, 4),
          data: ActionData::with_name("Sloppy Jab".into())
      }
  }

  pub fn get_name(&self) -> String {
    self.data.name.clone()
  }
}

impl Action for SloppyJab {
  fn can_perform(&self, source: &Actor, target: &Actor) -> bool {
    can_perform_action(&self.data, source, target)
  }
  fn perform(&self) -> EffectsToPerform {
      (
          vec![
              Box::new(AddTokenEffect::new(Token::Unstability, 1)),
              Box::new(RemoveTokenEffect::new(Token::Stamina, 1)),
          ],
          vec![
              Box::new(AddTokenEffect::new(Token::Damage, self.damage.generate())),
          ],
      )
  }
}

pub struct RegainStance {
  data: ActionData
}

impl RegainStance {
  pub fn new() -> Self {
      Self {
          data: ActionData::with_name("Regain Stance".into())
      }
  }

  pub fn get_name(&self) -> String {
    self.data.name.clone()
  }
}

impl Action for RegainStance {
  fn can_perform(&self, _source: &Actor, _target: &Actor) -> bool {
    true
  }
  fn perform(&self) -> EffectsToPerform {
      (
          vec![
              Box::new(RemoveTokenEffect::new(Token::Unstability, u8::MAX)),
          ],
          vec![],
      )
  }
}