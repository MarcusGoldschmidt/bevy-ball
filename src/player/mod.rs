pub mod player;
mod systems;

use crate::player::systems::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Event)]
pub struct PlayerMovedEvent;

#[derive(Event)]
pub struct PlayerReceiveXpEvent {
    pub xp: u32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovedEvent>()
            .add_event::<PlayerReceiveXpEvent>()
            .add_systems(Update, player_movement)
            .add_systems(Update, receive_xp_listener)
            .add_systems(Update, enemy_hit_player);
    }
}
