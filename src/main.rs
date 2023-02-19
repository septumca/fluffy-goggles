use std::collections::HashMap;

use macroquad::prelude::*;

mod ui;

fn can_perform_skill(data: &SkillData, source: &Actor, target: &Actor) -> bool {
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

#[derive(Hash, Eq, PartialEq, Debug)]
enum Token {
    Unstability,
    WideOpen,
    Block,
    Dodge,
    Counter,
    Stamina
}

type TokenContainer = HashMap<Token, usize>;

struct DmgRange<T> {
    low: T,
    high: T,
}

impl<T> DmgRange<T> {
    fn new(low: T, high: T) -> Self {
        Self { low, high }
    }
}

struct Actor {
    name: String,
    health: isize,
    tokens: TokenContainer
}

trait Skill {
    fn perform(&self) -> Box<dyn Effect>;
}

trait Effect {
    fn execute(&self, source: &mut Actor, target: &mut Actor);
}

struct SloppyJabEffect {
    damage: usize,
}

struct RegainStanceEffect {}

impl Effect for SloppyJabEffect {
    fn execute(&self, source: &mut Actor, target: &mut Actor) {
        
    }
}

impl Effect for RegainStanceEffect {
    fn execute(&self, source: &mut Actor, target: &mut Actor) {
        
    }
}

struct SkillData {
    name: String,
    required_tokens_source: TokenContainer,
    required_tokens_target: TokenContainer,
}

impl SkillData {
    fn with_name(name: String) -> Self {
        Self { 
            name,
            required_tokens_source: HashMap::new(), 
            required_tokens_target: HashMap::new()
        }
    }
}

struct SloppyJab {
    damage: DmgRange<usize>,
    data: SkillData
}

impl SloppyJab {
    fn new() -> Self {
        Self {
            damage: DmgRange { low: 1, high: 4 }, 
            data: SkillData::with_name("Sloppy Jab".into())
        }
    }
}

impl Skill for SloppyJab {
    fn perform(&self) -> Box<dyn Effect> {
        Box::new(SloppyJabEffect { damage: 1 }) //generate random number here
    }
}

struct RegainStance {
    data: SkillData
}

impl RegainStance {
    fn new() -> Self {
        Self {
            data: SkillData::with_name("Regain Stance".into())
        }
    }
}

impl Skill for RegainStance {
    fn perform(&self) -> Box<dyn Effect> {
        Box::new(RegainStanceEffect {})
    }
}


#[macroquad::main("fluffy-goggles")]
async fn main() {
    loop {
        clear_background(GRAY);

        let mut a1 = Actor {
            name: "A1".into(),
            health: 5,
            tokens: HashMap::new(),
        };

        let mut a2 = Actor {
            name: "A2".into(),
            health: 5,
            tokens: HashMap::new(),
        };

        let sloppy_jab = SloppyJab::new();
        let regain_stance = RegainStance::new();

        let texture = Texture2D::from_file_with_format(
            include_bytes!("../assets/under-construction.png"),
            None,
        );
        texture.set_filter(FilterMode::Nearest);
        
        if ui::create_skill_button(texture, vec2(16., 16.), sloppy_jab.data.name.clone()) {
            if can_perform_skill(&sloppy_jab.data, &a1, &a2) {
                let effect = sloppy_jab.perform();
                effect.execute(&mut a1, &mut a2);
            }
        }

        if ui::create_skill_button(texture, vec2(96., 16.), regain_stance.data.name.clone()) {
            if can_perform_skill(&regain_stance.data, &a1, &a2) {
                let effect = regain_stance.perform();
                effect.execute(&mut a1, &mut a2);
            }
        }

        next_frame().await
    }
}