use crate::debug::ShowInfoPlugin;
use crate::enemy::EnemyPlugin;
use crate::phase::systems::{setup, spawn_enemy_listener};
use crate::phase::PhaseStatus::Running;
use crate::player::PlayerPlugin;
use bevy::app::{App, Plugin, PostStartup, Startup, Update};
use bevy::prelude::{Component, Event, Resource};

mod systems;

#[derive(PartialEq)]
pub enum PhaseStatus {
    Running,
    Paused,
    GameOver,
}

#[derive(Resource)]
pub struct PhaseState {
    pub last_enemy_spawn_in_seconds: f32,
    pub start_at: f32,
    pub enemy_spawn_time: f32,
    pub status: PhaseStatus,
}

impl Default for PhaseState {
    fn default() -> Self {
        Self {
            start_at: 0.,
            enemy_spawn_time: 1.,
            last_enemy_spawn_in_seconds: 0.,
            status: Running,
        }
    }
}

impl PhaseState {
    pub fn score(&self, now: f32) -> f32 {
        now - self.start_at
    }
}

#[derive(Event, Clone)]
pub struct SpawnEnemyEvent;

#[derive(Component)]
pub struct GameOverText;

pub struct PhasePlugin;

impl Plugin for PhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemyEvent>()
            .add_plugins(ShowInfoPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, spawn_enemy_listener);
    }
}
