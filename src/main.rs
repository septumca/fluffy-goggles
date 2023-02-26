use std::collections::HashSet;

use action::{SingleEnemy, TokenChange, Actionable, apply_token_changes};
use token::{TokenContainer, TokenType};

mod token;
mod action;

fn main() {
    let mut rng = rand::thread_rng();
    let mut tc_player = TokenContainer::new(&[
        (TokenType::Block, 2),
        (TokenType::Dodge, 2),
        (TokenType::Health, 10),
        (TokenType::Stamina, 10)
    ]);
    let mut tc_enemy = TokenContainer::new(&[
        (TokenType::Block, 1),
        (TokenType::Dodge, 1),
        (TokenType::Health, 5),
        (TokenType::Stamina, 10)
        ]);

    let attack_action = SingleEnemy::new(
        HashSet::new(),
        vec![
            TokenChange::Remove(TokenType::Block, 1),
            TokenChange::Remove(TokenType::Dodge, 1),
            TokenChange::Remove(TokenType::Health, 2),
        ],
        vec![
            TokenChange::Add(TokenType::Vulnurable, 1),
            TokenChange::Remove(TokenType::Stamina, 1)
        ]
    );

    let Some((
        changes_source,
        changes_target
    )) = attack_action.execute(&mut rng, &tc_player, &tc_enemy) else {
        return;
    };

    apply_token_changes(&changes_source, &mut tc_player);
    apply_token_changes(&changes_target, &mut tc_enemy);
}