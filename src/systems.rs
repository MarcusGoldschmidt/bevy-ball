use crate::phase::GameOverText;
use crate::shared::CameraMarker;
use crate::GameOver;
use bevy::app::AppExit;
use bevy::input::Input;
use bevy::prelude::{
    default, Camera2dBundle, Color, Commands, EventReader, EventWriter, KeyCode, PositionType,
    Query, Res, Style, TextAlignment, TextBundle, TextStyle, Transform, Val, Window, With,
};
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
        Camera2dBundle {
            transform: transform_center,
            ..default()
        },
    ));
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    for _ in game_over_event_reader.read() {
        let window = window_query.single();

        commands.spawn((
            GameOverText,
            TextBundle::from_section(
                "Game Over\n Press space to restart",
                TextStyle {
                    font: Default::default(),
                    font_size: 24.,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_background_color(Color::RED)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(window.height() / 2.),
                left: Val::Px(window.width() * 0.85 / 2.),
                ..default()
            }),
        ));
    }
}
