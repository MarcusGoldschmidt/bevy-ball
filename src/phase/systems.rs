use crate::enemy::Enemy;
use crate::health::Health2d;
use crate::phase::{EnemySpeed, PhaseState, SpawnEnemyEvent};
use crate::player::player::Player;
use crate::shared::InfoText;
use crate::shot::Shooter;
use bevy::asset::Assets;
use bevy::color::palettes::css::RED;
use bevy::color::palettes::tailwind::FUCHSIA_500;
use bevy::color::{Color, LinearRgba, Luminance};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    AmbientLight, Circle, ColorMaterial, Commands, EventReader, EventWriter, Mesh, Mesh2d,
    MeshMaterial2d, Query, ResMut, Text, Transform, Window, With,
};
use bevy::text::{TextColor, TextFont};
use bevy::window::PrimaryWindow;
use rand::random;
use std::iter;

pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_enemy_event_writter: EventWriter<SpawnEnemyEvent>,
) {
    commands.insert_resource(PhaseState::default());
    commands.insert_resource(AmbientLight {
        color: RED.into(),
        brightness: 1.02,
    });

    // Text
    commands.spawn((
        InfoText,
        Text::new("DEBUG"),
        TextFont {
            font_size: 24.,
            ..Default::default()
        },
        TextColor(Color::LinearRgba(LinearRgba::WHITE)),
    ));

    let window = window_query.get_single().unwrap();

    let transform_center = Transform::from_xyz(window.width() / 2., window.height() / 2., 1.);

    commands.spawn((
        Player {
            speed: 500.,
            ..Player::default()
        },
        Shooter {
            fire_rate: 1.,
            damage: 1.,
            direction: Vec2::new(0., 1.),
            last_shoot: std::time::Instant::now(),
        },
        Transform::from(transform_center),
        Mesh2d(meshes.add(Circle::new(Player::default().size))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::Srgba(FUCHSIA_500)))),
    ));

    let events = iter::repeat(SpawnEnemyEvent::default()).take(5);

    spawn_enemy_event_writter.send_batch(events);
}

pub fn spawn_enemy_listener(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut event_reader: EventReader<SpawnEnemyEvent>,
    player_query: Query<(&Player, &Transform), With<Player>>,
    mut state: ResMut<PhaseState>,
) {
    let window = window_query.get_single().unwrap();

    for event in event_reader.read() {
        if let Ok((player, transform)) = player_query.get_single() {
            use rand::Rng;

            let enemy_position = loop {
                let random_x = random::<f32>() * window.width();
                let random_y = random::<f32>() * window.height();

                let enemy_position = Vec3::new(random_x, random_y, 0.);

                if enemy_position.distance(transform.translation) > player.size * 10. {
                    break enemy_position;
                }
            };
            let direction = (transform.translation - enemy_position).normalize();

            let speed = match event.speed {
                EnemySpeed::RandomSlow => rand::thread_rng().gen_range(100..200) as f32,
                EnemySpeed::RandomNormal => rand::thread_rng().gen_range(200..300) as f32,
                EnemySpeed::RandomFast => rand::thread_rng().gen_range(300..500) as f32,
                EnemySpeed::Value(x) => x,
            };

            let color = match event.speed {
                EnemySpeed::RandomSlow => event.color.darker(0.2),
                EnemySpeed::RandomFast => event.color.lighter(0.2),
                _ => event.color,
            };

            // Add Heath bar with
            // https://bevy-cheatbook.github.io/fundamentals/hierarchy.html

            commands.spawn((
                Enemy {
                    size: event.size,
                    speed,
                    direction: direction.truncate(),
                },
                Health2d::full_health(1.),
                Transform::from_translation(enemy_position),
                Mesh2d(meshes.add(Circle::new(event.size))),
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
            ));

            state.last_enemy_spawn_in_seconds = 0.;
        }
    }
}

pub fn track_mouse_to_shoot(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&Player, &mut Shooter, &Transform)>,
) {
    // Games typically only have one window (the primary window)
    if let Ok(window) = q_windows.get_single() {
        if let Some(position) = window.cursor_position() {
            if let Ok((_player, mut shooter, player_position)) = player_query.get_single_mut() {
                // Y it's different because the window height is inverted
                let position = Vec2::new(position.x, window.height() - position.y);

                let direction = (position - player_position.translation.truncate()).normalize();

                shooter.direction = direction;
            }
        }
    }
}

pub fn increase_spawn_rate_over_time(mut state: ResMut<PhaseState>) {
    state.enemy_spawn_time = state.base_spawn_time - (state.start_at.elapsed().as_secs_f32() / 60.);
}
