#[derive(PartialEq, Debug)]
pub enum DamageType {
    Magic,
    Burn,
    Necrotic,
}

#[derive(Debug)]
pub struct Damage {
    pub value: i8,
    pub damage_type: DamageType
}

impl Damage {
    pub fn new(value: i8, damage_type: DamageType) -> Self {
        Self {
            value, damage_type
        }
    }
}