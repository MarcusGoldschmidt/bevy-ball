mod debug;
mod enemy;
mod health;
mod phase;
mod player;
mod shared;
mod shot;
mod systems;
mod utils;
mod timefade;

use crate::phase::PhasePlugin;
use crate::systems::{exit_game, handle_game_over, on_window_resized, spawn_camera};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::WindowMode;

#[derive(Event)]
struct RestartEvent;

#[derive(Event)]
pub struct GameOver;

fn main() {
    App::new()
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..default()
            }),
            ..default()
        }))
        .add_event::<RestartEvent>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, on_window_resized)
        .add_plugins(PhasePlugin)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}

#[derive(PartialEq)]
enum GameStatus {
    Running,
    GameOver,
}

#[derive(Resource)]
struct GameState {
    status: GameStatus,
}
