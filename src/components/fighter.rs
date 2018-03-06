#[derive(Debug, Default)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defence: i32,
    pub power: i32
}

impl Fighter {
    pub fn new(max_hp: i32, defence: i32, power: i32) -> Fighter {
        let mut fighter = Fighter::default();
        fighter.max_hp = max_hp;
        fighter.hp = max_hp;
        fighter.defence = defence;
        fighter.power = power;

        fighter
    }
}
