pub mod player;
mod systems;

use crate::player::systems::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Event)]
pub struct PlayerMovedEvent;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovedEvent>()
            .add_systems(Update, player_movement)
            .add_systems(Update, enemy_hit_player);
    }
}
