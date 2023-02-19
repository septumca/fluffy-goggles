use macroquad::prelude::*;
use macroquad::ui::{root_ui,widgets};

pub fn create_skill_button(texture: Texture2D, position: Vec2, label: String) -> bool {
    widgets::Label::new(label)
        .position(position + vec2(0., 64.))
        .ui(&mut root_ui());
    widgets::Button::new(texture)
        .position(position)
        .size(vec2(64., 64.))
        .ui(&mut root_ui())
}