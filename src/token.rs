use std::{collections::{BTreeMap, HashSet}};

const DEFAULT_TOKEN_LIMIT: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenType {
    Dodge,
    Blind,
    Block,
    Vulnurable,
    Daze,
    Stun,
    Weak,
    Strong,
    Stamina,
    Health,
    ExtraHealth,
}

pub type TokenSet = HashSet<TokenType>;
type TokensMap = BTreeMap<TokenType, u8>;

pub struct TokenContainer {
    tokens: TokensMap,
    limits: TokensMap,
}

fn vec_to_btreemap<K: Clone + Ord, V: Copy>(elems: &[(K, V)]) -> BTreeMap<K, V> {
    elems.into_iter().map(|(t, v)| (t.clone(), *v)).collect::<BTreeMap<K, V>>()
}

impl TokenContainer {
    pub fn new(limits: &[(TokenType, u8)]) -> Self {
        Self {
            tokens: BTreeMap::new(),
            limits: vec_to_btreemap(limits),
        }
    }

    pub fn set_token_limit(&mut self, token_type: &TokenType, limit: u8) {
        self.limits.insert(token_type.clone(), limit);
        let Some(value) = self.tokens.get_mut(token_type) else {
            return;
        };
        if *value > limit {
            *value = limit;
        }
    }

    pub fn get_token_count(&self, token_type: &TokenType) -> u8 {
        self.tokens
            .get(token_type)
            .and_then(|v| Some(*v))
            .unwrap_or(0)
    }

    pub fn add_token(&mut self, token_type: &TokenType, value: u8) {
        let token_limit = *self.limits.get(token_type).unwrap_or(&DEFAULT_TOKEN_LIMIT);
        self.tokens
            .entry(token_type.clone())
            .and_modify(|v| { *v = (*v + value).min(token_limit) })
            .or_insert(value.min(token_limit));
    }

    pub fn remove_token(&mut self, token_type: &TokenType, value: u8) {
        let Some(v) = self.tokens.get_mut(token_type) else {
            return;
        };
        if *v > value {
            *v = *v - value;
        } else {
            self.tokens.remove(token_type);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_btreemap_from_vec() {
        let btm = vec_to_btreemap(&[
            (TokenType::Vulnurable, 2),
            (TokenType::Block, 1)
        ]);

        assert_eq!(btm.get(&TokenType::Block), Some(&1));
        assert_eq!(btm.get(&TokenType::Vulnurable), Some(&2));
        assert_eq!(btm.get(&TokenType::Daze), None);
    }

    fn get_container() -> TokenContainer {
        TokenContainer::new(&[
            (TokenType::Vulnurable, 2),
            (TokenType::Dodge, 1)
        ])
    }

    #[test]
    fn get_non_existing_token() {
        let tc = get_container();
        assert_eq!(tc.get_token_count(&TokenType::Block), 0);
    }

    mod add {
        use super::*;

        #[test]
        fn existing_token() {
            let mut tc = get_container();
            tc.add_token(&TokenType::Weak, 1);
            assert_eq!(tc.get_token_count(&TokenType::Weak), 1);
        }

        #[test]
        fn more_tokens_than_default_limit() {
            let mut tc = get_container();
            tc.add_token(&TokenType::Weak, 3);
            assert_eq!(tc.get_token_count(&TokenType::Weak), 1);
        }

        #[test]
        fn more_tokens_than_limit() {
            let mut tc = get_container();
            tc.add_token(&TokenType::Vulnurable, 5);
            assert_eq!(tc.get_token_count(&TokenType::Vulnurable), 2);
        }

        #[test]
        fn more_tokens_than_set_existing_limit() {
            let mut tc = get_container();
            tc.set_token_limit(&TokenType::Vulnurable, 3);
            tc.add_token(&TokenType::Vulnurable, 5);
            assert_eq!(tc.get_token_count(&TokenType::Vulnurable), 3);
        }

        #[test]
        fn more_tokens_than_newly_set_limit() {
            let mut tc = get_container();
            tc.set_token_limit(&TokenType::Daze, 2);
            tc.add_token(&TokenType::Daze, 5);
            assert_eq!(tc.get_token_count(&TokenType::Daze), 2);
        }

        #[test]
        fn decrease_limit_with_more_tokens() {
            let mut tc = get_container();
            tc.set_token_limit(&TokenType::Daze, 4);
            tc.add_token(&TokenType::Daze, 5);
            tc.set_token_limit(&TokenType::Daze, 2);
            assert_eq!(tc.get_token_count(&TokenType::Daze), 2);
        }
    }

    mod remove {
        use super::*;

        #[test]
        fn simple() {
            let mut tc = get_container();
            tc.set_token_limit(&TokenType::Daze, 4);
            tc.add_token(&TokenType::Daze, 3);
            tc.remove_token(&TokenType::Daze, 2);
            assert_eq!(tc.get_token_count(&TokenType::Daze), 1);
        }

        #[test]
        fn more_than_value() {
            let mut tc = get_container();
            tc.set_token_limit(&TokenType::Daze, 4);
            tc.add_token(&TokenType::Daze, 3);
            tc.remove_token(&TokenType::Daze, 5);
            assert_eq!(tc.get_token_count(&TokenType::Daze), 0);
        }
    }
}
