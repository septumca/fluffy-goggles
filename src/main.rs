use std::{collections::{BTreeMap}};

use action::{SloppyJab, RegainStance, Action};
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

type Position = (f32, f32);


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
    let mut a1 = Actor::new("A1".into(), 15, 10, 2);
    let mut a2 = Actor::new("A2".into(), 15, 10, 2);
    let sloppy_jab = SloppyJab::new();
    let regain_stance = RegainStance::new();

    loop {
        clear_background(GRAY);

        if ui::create_skill_button(under_construction_texture, vec2(16., 16.), sloppy_jab.get_name()) {
            if sloppy_jab.can_perform(&a1, &a2) {
                let (source_effects, target_effects) = sloppy_jab.perform();
                for e in source_effects {
                    e.execute(&mut a1);
                }
                for e in target_effects {
                    e.execute(&mut a2);
                }
            }
        }

        if ui::create_skill_button(under_construction_texture, vec2(96., 16.), regain_stance.get_name()) {
            if regain_stance.can_perform(&a1, &a2) {
                let (source_effects, target_effects) = regain_stance.perform();
                for e in source_effects {
                    e.execute(&mut a1);
                }
                for e in target_effects {
                    e.execute(&mut a2);
                }
            }
        }

        a1.draw(&player_texture, (150., 200.), &under_construction_texture);
        a2.draw(&enemy_texture, (450., 200.), &under_construction_texture);

        next_frame().await
    }
}