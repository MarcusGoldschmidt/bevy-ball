use bevy::app::{App, Plugin, Update};
use bevy::math::Vec2;
use bevy::prelude::Component;

mod systems;

use systems::*;

#[derive(Component)]
pub struct Enemy {
    pub size: f32,
    pub speed: f32,
    pub direction: Vec2,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy_over_time)
            .add_systems(Update, move_enemies)
            .add_systems(Update, follow_player_event_listener);
    }
}
