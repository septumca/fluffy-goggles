use rand::Rng;

use crate::token::{TokenContainer, TokenSet, TokenType};


//TODO: for which actor? by ID or reference
#[derive(Clone, PartialEq, Debug)]
pub enum TokenChange {
    Add(TokenType, u8),
    Remove(TokenType, u8),
}


pub trait Actionable {
    fn execute<R: Rng>(
        &self, rng: &mut R, source: &TokenContainer, target: &TokenContainer
    ) -> Option<(Vec<TokenChange>, Vec<TokenChange>)>;
}

pub struct SingleEnemy {
    ignore: TokenSet,
    changes_target: Vec<TokenChange>,
    changes_source: Vec<TokenChange>,
}

impl SingleEnemy {
    pub fn new(
        ignore: TokenSet,
        changes_target: Vec<TokenChange>,
        changes_source: Vec<TokenChange>
    ) -> Self {
        Self {
            ignore,
            changes_target,
            changes_source
        }
    }
}

impl Actionable for SingleEnemy {
    fn execute<R: Rng>(
        &self, rng: &mut R, source: &TokenContainer, target: &TokenContainer
    ) -> Option<(Vec<TokenChange>, Vec<TokenChange>)> {
        let mut miss_chance: i8 = 0;
        if !self.ignore.contains(&TokenType::Dodge) &&
            target.get_token_count(&TokenType::Dodge) > 0
        {
            miss_chance = miss_chance + 50;
        }
        if !self.ignore.contains(&TokenType::Blind) &&
            source.get_token_count(&TokenType::Blind) > 0
        {
            miss_chance = miss_chance + 50;
        }
        let roll = rng.gen_range(1..=100);
        if roll <= miss_chance {
            return None; //MISS
        }

        Some((self.changes_source.clone(), self.changes_target.clone())) //HIT
    }
}

pub fn apply_token_changes(changes: &[TokenChange], tokens: &mut TokenContainer) {
    for change in changes {
        match change {
            TokenChange::Add(token_type, value) => tokens.add_token(&token_type, *value),
            TokenChange::Remove(token_type, value) => tokens.remove_token(&token_type, *value)
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::{StdRng}, SeedableRng};

    use super::*;

    fn get_generator_with_target(target_value: u8) -> Option<rand::prelude::StdRng> {
        for i in 0..10000 {
            let mut rng = StdRng::seed_from_u64(i);
            if rng.gen_range(1..=100) == target_value {
                return Some(StdRng::seed_from_u64(i));
            }
        }
        return None;
    }

    #[test]
    fn test_generator_generation() {
        for i in 1..=100 {
            let mut rng = get_generator_with_target(i).expect("should be able to return generator for number");
            let roll = rng.gen_range(1..=100);
            assert_eq!(i, roll);
        }
    }

    #[test]
    fn apply_some_token_changes() {
        let mut target = TokenContainer::new(&vec![(TokenType::Block, 3)]);
        target.add_token(&TokenType::Block, 2);
        let target_changes = vec![
            TokenChange::Add(TokenType::Blind, 1),
            TokenChange::Remove(TokenType::Block, 1),
            TokenChange::Remove(TokenType::Daze, 1),
            TokenChange::Add(TokenType::Strong, 2),
        ];
        apply_token_changes(&target_changes, &mut target);

        assert_eq!(target.get_token_count(&TokenType::Blind), 1);
        assert_eq!(target.get_token_count(&TokenType::Block), 1);
        assert_eq!(target.get_token_count(&TokenType::Daze), 0);
        assert_eq!(target.get_token_count(&TokenType::Strong), 1);
    }

    mod execute {
        use std::collections::HashSet;
        use super::*;

        #[test]
        fn miss_with_blind_and_dodge() {
            let mut rng = get_generator_with_target(100).expect("should be able to return generator for number");
            let action = SingleEnemy::new(HashSet::new(), vec![], vec![]);
            let mut source = TokenContainer::new(&vec![]);
            let mut target = TokenContainer::new(&vec![]);
            target.add_token(&TokenType::Dodge, 1);
            source.add_token(&TokenType::Blind, 1);
            let result = action.execute(&mut rng, &mut source, &mut target);

            assert_eq!(result, None);
        }

        #[test]
        fn miss_with_blind() {
            let mut rng = get_generator_with_target(49).expect("should be able to return generator for number");
            let action = SingleEnemy::new(HashSet::new(), vec![], vec![]);
            let mut source = TokenContainer::new(&vec![]);
            let mut target = TokenContainer::new(&vec![]);
            target.add_token(&TokenType::Dodge, 1);
            let result = action.execute(&mut rng, &mut source, &mut target);

            assert_eq!(result, None);
        }

        #[test]
        fn hit_with_dodge_when_ignored() {
            let mut rng = get_generator_with_target(1).expect("should be able to return generator for number");
            let action = SingleEnemy::new(HashSet::from([TokenType::Dodge]), vec![], vec![]);
            let mut source = TokenContainer::new(&vec![]);
            let mut target = TokenContainer::new(&vec![]);
            target.add_token(&TokenType::Dodge, 1);
            let result = action.execute(&mut rng, &mut source, &mut target);

            assert_eq!(result, Some((vec![], vec![])));
        }

        #[test]
        fn hit_with_blind() {
            let mut rng = get_generator_with_target(51).expect("should be able to return generator for number");
            let action = SingleEnemy::new(HashSet::new(), vec![], vec![]);
            let mut source = TokenContainer::new(&vec![]);
            let mut target = TokenContainer::new(&vec![]);
            source.add_token(&TokenType::Blind, 1);
            let result = action.execute(&mut rng, &mut source, &mut target);

            assert_eq!(result, Some((vec![], vec![])));
        }

        #[test]
        fn miss_with_dodge() {
            let mut rng = get_generator_with_target(49).expect("should be able to return generator for number");
            let action = SingleEnemy::new(HashSet::new(), vec![], vec![]);
            let mut source = TokenContainer::new(&vec![]);
            let mut target = TokenContainer::new(&vec![]);
            target.add_token(&TokenType::Dodge, 1);
            let result = action.execute(&mut rng, &mut source, &mut target);

            assert_eq!(result, None);
        }

        #[test]
        fn hit_with_dodge() {
            let mut rng = get_generator_with_target(51).expect("should be able to return generator for number");
            let action = SingleEnemy::new(HashSet::new(), vec![], vec![]);
            let mut source = TokenContainer::new(&vec![]);
            let mut target = TokenContainer::new(&vec![]);
            target.add_token(&TokenType::Dodge, 1);
            let result = action.execute(&mut rng, &mut source, &mut target);

            assert_eq!(result, Some((vec![], vec![])));
        }

        #[test]
        fn miss_with_dodge_roll_50() {
            let mut rng = get_generator_with_target(50).expect("should be able to return generator for number");
            let action = SingleEnemy::new(HashSet::new(), vec![], vec![]);
            let mut source = TokenContainer::new(&vec![]);
            let mut target = TokenContainer::new(&vec![]);
            target.add_token(&TokenType::Dodge, 1);
            let result = action.execute(&mut rng, &mut source, &mut target);

            assert_eq!(result, None);
        }

        #[test]
        fn hit_roll_1() {
            let mut rng = get_generator_with_target(1).expect("should be able to return generator for number");
            let action = SingleEnemy::new(HashSet::new(), vec![], vec![]);
            let mut source = TokenContainer::new(&vec![]);
            let mut target = TokenContainer::new(&vec![]);
            let result = action.execute(&mut rng, &mut source, &mut target);

            assert_eq!(result, Some((vec![], vec![])));
        }

        #[test]
        fn hit() {
            let mut rng = get_generator_with_target(98).expect("should be able to return generator for number");
            let action = SingleEnemy::new(HashSet::new(), vec![], vec![]);
            let mut source = TokenContainer::new(&vec![]);
            let mut target = TokenContainer::new(&vec![]);
            let result = action.execute(&mut rng, &mut source, &mut target);

            assert_eq!(result, Some((vec![], vec![])));
        }

        #[test]
        fn hit_with_changes() {
            let target_changes = vec![
                TokenChange::Add(TokenType::Blind, 1),
                TokenChange::Remove(TokenType::Block, 1),
            ];
            let source_changes = vec![
                TokenChange::Add(TokenType::Dodge, 1),
                TokenChange::Remove(TokenType::Block, 1),
            ];
            let mut rng = get_generator_with_target(98).expect("should be able to return generator for number");
            let action = SingleEnemy::new(
                HashSet::new(),
                target_changes.clone(),
                source_changes.clone()
            );
            let mut source = TokenContainer::new(&vec![]);
            let mut target = TokenContainer::new(&vec![]);
            let result = action.execute(&mut rng, &mut source, &mut target);

            assert_eq!(result, Some((source_changes, target_changes)));
        }
    }

}
