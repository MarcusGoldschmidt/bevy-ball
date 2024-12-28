use crate::enemy::Enemy;
use crate::phase::{PhaseState, SpawnEnemyEvent};
use crate::player::player::Player;
use crate::shared::InfoText;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{
    default, shape, Color, ColorMaterial, Commands, EventReader, EventWriter, Mesh, Query, ResMut,
    TextBundle, TextStyle, Transform, Window, With,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use rand::random;
use std::iter;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_enemy_event_writter: EventWriter<SpawnEnemyEvent>,
) {
    commands.insert_resource(PhaseState::default());

    // Text
    commands.spawn((
        InfoText,
        TextBundle::from_section(
            "DEBUG",
            TextStyle {
                font: Default::default(),
                font_size: 24.,
                color: Color::WHITE,
            },
        ),
    ));

    const SIZE: f32 = 50.;

    let window = window_query.get_single().unwrap();

    let transform_center = Transform::from_xyz(window.width() / 2., window.height() / 2., 1.);

    commands.spawn((
        Player {
            size: SIZE,
            speed: 500.,
            ..Player::default()
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: transform_center,
            ..default()
        },
    ));

    let events = iter::repeat(SpawnEnemyEvent).take(5);

    spawn_enemy_event_writter.send_batch(events);
}

pub fn spawn_enemy_listener(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_enemy_event: EventReader<SpawnEnemyEvent>,
    player_query: Query<(&Player, &Transform), With<Player>>,
    mut state: ResMut<PhaseState>,
) {
    let window = window_query.get_single().unwrap();

    for _ in spawn_enemy_event.read() {
        if let Ok((player, transform)) = player_query.get_single() {
            use rand::Rng;

            const SIZE: f32 = 20.;

            let enemy_position = loop {
                let random_x = random::<f32>() * window.width();
                let random_y = random::<f32>() * window.height();

                let enemy_position = Vec3::new(random_x, random_y, 0.);

                if enemy_position.distance(transform.translation) > player.size * 10. {
                    break enemy_position;
                }
            };
            let direction = (transform.translation - enemy_position).normalize();

            let speed = rand::thread_rng().gen_range(150..350) as f32;

            commands.spawn((
                Enemy {
                    size: SIZE,
                    speed,
                    direction: direction.truncate(),
                },
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(SIZE).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(enemy_position),
                    ..default()
                },
            ));

            state.last_enemy_spawn_in_seconds = 0.;
        }
    }
}
