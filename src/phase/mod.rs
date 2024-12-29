use crate::debug::ShowInfoPlugin;
use crate::enemy::EnemyPlugin;
use crate::health::HealthPlugin;
use crate::phase::systems::{
    increase_spawn_rate_over_time, setup, spawn_enemy_listener, track_mouse_to_shoot,
};
use crate::phase::PhaseStatus::Running;
use crate::player::PlayerPlugin;
use crate::shot::ShotPlugin;
use bevy::app::{App, Last, Plugin, Startup, Update};
use bevy::color::LinearRgba;
use bevy::prelude::{Color, Component, Event, Resource};
use bevy::utils::Instant;

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
    pub start_at: Instant,

    pub base_spawn_time: f32,
    pub enemy_spawn_time: f32,
    pub status: PhaseStatus,
}

impl Default for PhaseState {
    fn default() -> Self {
        Self {
            start_at: Instant::now(),
            base_spawn_time: 4.,
            enemy_spawn_time: 4.,
            last_enemy_spawn_in_seconds: 0.,
            status: Running,
        }
    }
}

impl PhaseState {
    pub fn score(&self) -> f32 {
        self.start_at.elapsed().as_secs_f32()
    }
}

#[derive(PartialEq, Clone)]
pub enum EnemySpeed {
    RandomSlow,
    RandomNormal,
    RandomFast,
    Value(f32),
}

#[derive(Event, Clone)]
pub struct SpawnEnemyEvent {
    pub size: f32,
    pub speed: EnemySpeed,

    pub color: Color,
}

impl Default for SpawnEnemyEvent {
    fn default() -> Self {
        Self {
            size: 10.,
            speed: EnemySpeed::RandomNormal,
            color: Color::LinearRgba(LinearRgba::RED),
        }
    }
}

#[derive(Component)]
pub struct GameOverText;

pub struct PhasePlugin;

impl Plugin for PhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemyEvent>()
            .add_plugins(ShowInfoPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(HealthPlugin)
            .add_plugins(ShotPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, track_mouse_to_shoot)
            .add_systems(Update, spawn_enemy_listener)
            .add_systems(Last, increase_spawn_rate_over_time);
    }
}
