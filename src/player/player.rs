use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {
    pub max_health: u32,

    pub size: f32,

    pub fire_rate_ps: f32,
    pub damage_shot: u32,
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            max_health: 100,
            fire_rate_ps: 1.0,
            damage_shot: 10,
            speed: 500.,
            size: 20.,
        }
    }
}

pub type ApplyOnPlayer = fn(&mut Player) -> ();

impl Player {
    pub fn apply(&mut self, apply: ApplyOnPlayer) {
        apply(self);
    }
}
