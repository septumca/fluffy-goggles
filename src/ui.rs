use macroquad::prelude::*;
use macroquad::ui::{root_ui,widgets};

use crate::Actor;
use crate::action::Action;

fn create_skill_button(texture: Texture2D, position: Vec2, label: String) -> bool {
    widgets::Label::new(label)
        .position(position + vec2(0., 64.))
        .ui(&mut root_ui());
    widgets::Button::new(texture)
        .position(position)
        .size(vec2(64., 64.))
        .ui(&mut root_ui())
}

pub fn action_clicked<'a>(
    texture: Texture2D,
    position: Vec2,
    action: Box<&'a dyn Action>,
    source: &Actor,
    target: &Actor
) -> Option<Box<&'a dyn Action>> {
    if create_skill_button(texture, position, action.get_name()) {
        if action.can_perform(&source, &target) {
            return Some(action);
        }
    }
    None
}
