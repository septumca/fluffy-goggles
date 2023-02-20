use std::collections::BTreeMap;

use macroquad::{prelude::*};

use crate::Position;

pub const TOKEN_SPRITE_SIZE: f32 = 32.0;
pub const TOKEN_FONT_SIZE: f32 = 12.0;

pub type TokenContainer = BTreeMap<Token, u8>;

#[derive(Hash, Eq, PartialEq, Debug, Clone, PartialOrd, Ord)]
pub enum Token {
    Health,
    Damage,
    Unstability,
    WideOpen,
    Block,
    Dodge,
    Counter,
    Stamina
}

impl Token {
    pub fn draw(&self, position: Position, token_texture: &Texture2D) {
        draw_texture_ex(
            *token_texture,
            position.0,
            position.1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(TOKEN_SPRITE_SIZE, TOKEN_SPRITE_SIZE)),
                ..Default::default()
             }
        );
    }
}
