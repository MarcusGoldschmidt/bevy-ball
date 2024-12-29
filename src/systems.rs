use crate::phase::GameOverText;
use crate::shared::CameraMarker;
use crate::GameOver;
use bevy::app::AppExit;
use bevy::core_pipeline::bloom::{Bloom};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::text::{TextColor, TextFont};
use bevy::window::{PrimaryWindow, WindowResized};

pub fn on_window_resized(
    mut events: EventReader<WindowResized>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<CameraMarker>>,
) {
    let window = window_query.single();

    for _ in events.read() {
        for mut camera in camera_query.iter_mut() {
            camera.translation.x = window.width() / 2.;
            camera.translation.y = window.height() / 2.;
        }
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    let transform_center = Transform::from_xyz(window.width() / 2., window.height() / 2., 1.);

    commands.spawn((
        CameraMarker,
        Camera2d,
        Camera {
            hdr: true, // 1. HDR is required for bloom
            ..default()
        },
        Transform::from(transform_center),
        Tonemapping::TonyMcMapface,
        Bloom::default(),
    ));
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}

pub fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for _ in game_over_event_reader.read() {
        commands.spawn((
            GameOverText,
            Text::new("Game Over\n Press space to restart"),
            TextColor(Color::WHITE),
            TextFont {
                font_size: 24.,
                ..Default::default()
            },
        ));
    }
}
