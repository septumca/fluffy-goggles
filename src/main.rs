use std::{collections::{BTreeMap}};

use action::{SloppyJab, RegainStance, Action, Regeneration};
use macroquad::{prelude::*};
use token::{TokenContainer, Token, TOKEN_FONT_SIZE, TOKEN_SPRITE_SIZE};

mod ui;
mod action;
mod damage;
mod token;
mod effect;

const SRC_SPRITE_SIZE: f32 = 16.0;
const SPRITE_SIZE: f32 = 128.0;
const FONT_SIZE: f32 = 40.0;
const ACTIONS_PER_ROUND: u8 = 2;

type Position = (f32, f32);

enum Triggers {
    TokensAdded(u8),
    TokensRemoved(u8),
    RoundStart,
    RoundEnd,
    TurnStart,
    TurnEnd,
}

enum ModifierType {
    Buff,
    Debuff
}

pub struct Actor {
    name: String,
    action_per_round: u8,
    tokens: TokenContainer
}

impl Actor {
    fn new(name: String, health: u8, stamina: u8, action_per_round: u8) -> Self {
        Self {
            name,
            action_per_round,
            tokens: BTreeMap::from([
                (Token::Health, health),
                (Token::Stamina, stamina),
            ])
        }
    }

    fn draw(&self, texture: &Texture2D, position: Position, token_texture: &Texture2D) {
        draw_text(&self.name, position.0, position.1 - FONT_SIZE / 3., FONT_SIZE, WHITE);
        draw_texture_ex(
            *texture,
            position.0,
            position.1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(SPRITE_SIZE, SPRITE_SIZE)),
                ..Default::default()
             }
        );
        let mut offset = SPRITE_SIZE + 4.0;
        for (token, value) in &self.tokens {
            let x = position.0;
            let y = position.1 + offset;
            token.draw((x, y), token_texture);
            draw_text(&format!("{:?} x{}", token, value), x + TOKEN_SPRITE_SIZE + 2.0, y + TOKEN_SPRITE_SIZE - 4.0, FONT_SIZE, WHITE);
            offset += (TOKEN_FONT_SIZE * 2.5) + 4.0;
        }
    }
}


#[macroquad::main("fluffy-goggles")]
async fn main() {
    let sprite_sheet = Image::from_file_with_format(
        include_bytes!("../assets/colored-transparent_packed.png"),
        Some(ImageFormat::Png),
    );

    let under_construction_texture = Texture2D::from_image(&sprite_sheet.sub_image(Rect::new(35.0 * SRC_SPRITE_SIZE, 21.0 * SRC_SPRITE_SIZE, SRC_SPRITE_SIZE, SRC_SPRITE_SIZE)));
    let player_texture = Texture2D::from_image(&sprite_sheet.sub_image(Rect::new(30.0 * SRC_SPRITE_SIZE, 0.0 * SRC_SPRITE_SIZE, SRC_SPRITE_SIZE, SRC_SPRITE_SIZE)));
    let enemy_texture = Texture2D::from_image(&sprite_sheet.sub_image(Rect::new(26.0 * SRC_SPRITE_SIZE, 2.0 * SRC_SPRITE_SIZE, SRC_SPRITE_SIZE, SRC_SPRITE_SIZE)));
    under_construction_texture.set_filter(FilterMode::Nearest);
    player_texture.set_filter(FilterMode::Nearest);
    enemy_texture.set_filter(FilterMode::Nearest);
    let mut player_actor = Actor::new("A1".into(), 15, 10, ACTIONS_PER_ROUND);
    let mut enemy_actor = Actor::new("A2".into(), 15, 10, ACTIONS_PER_ROUND);
    let sloppy_jab = SloppyJab::new();
    let regain_stance = RegainStance::new();
    let regeneration = Regeneration::new();

    let mut is_player_turn = true;

    loop {
        clear_background(GRAY);

        let mut action: Option<Box<&dyn Action>> = None;

        let (mut source, mut target) = if is_player_turn {
            (&mut player_actor, &mut enemy_actor)
        } else {
            (&mut enemy_actor, &mut player_actor)
        };

        if let Some(performed_action) = ui::action_clicked(
            under_construction_texture,
            vec2(16., 16.),
            Box::new(&sloppy_jab),
            &source,
            &target
        ) {
            if is_player_turn {
                action = Some(performed_action);
            }
        }

        if let Some(performed_action) = ui::action_clicked(
            under_construction_texture,
            vec2(96., 16.),
            Box::new(&regain_stance),
            &source,
            &target
        ) {
            if is_player_turn {
                action = Some(performed_action);
            }
        }

        if !is_player_turn {
            println!("AI takes it's turn!");
            action = Some(Box::new(&regeneration)); //TODO: make and plug AI here
        }

        if let Some(action) = action {
            let (source_effects, target_effects) = action.perform();
            for e in source_effects {
                e.execute(&mut source);
            }
            for e in target_effects {
                e.execute(&mut target);
            }

            source.action_per_round = source.action_per_round - 1;
            if source.action_per_round == 0 {
                //round end here
                if !is_player_turn {
                    //turn ends here, AI goes always second
                }
                is_player_turn = !is_player_turn;
                source.action_per_round = ACTIONS_PER_ROUND;
            }
        }

        player_actor.draw(&player_texture, (150., 200.), &under_construction_texture);
        enemy_actor.draw(&enemy_texture, (450., 200.), &under_construction_texture);

        next_frame().await
    }
}