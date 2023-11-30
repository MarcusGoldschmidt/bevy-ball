use std::ops::Add;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::{PrimaryWindow, WindowResized};
use rand::random;

const ENEMY_SPAWN_TIME: f32 = 1.;

#[derive(Event)]
struct RestartEvent;

fn main() {
    App::new()
        .add_plugins((FrameTimeDiagnosticsPlugin))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".to_string(),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_event::<RestartEvent>()
        .add_systems(Startup, setup)
        .add_systems(Main, keyboard_input)
        .add_systems(Update, spawn_enemy_over_time)
        .add_systems(Update, show_info)
        .add_systems(Update, move_enemies)
        .add_systems(Update, enemy_hit_player)
        .add_systems(PreUpdate, restart_event_listener)
        .add_systems(PreUpdate, on_window_resized)
        .run();
}

fn on_window_resized(
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.single();

    let transform_center = Transform::from_xyz(window.width() / 2., window.height() / 2., 1.);

    println!("window: {:?}", transform_center.translation);

    commands.spawn((
        CameraMarker,
        Camera2dBundle {
            transform: transform_center,
            ..default()
        })
    );

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
        ))
    );

    const SIZE: f32 = 50.;

    commands.spawn((
        Snake {
            size: SIZE,
            speed: 500.,
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: transform_center,
            ..default()
        },
    ));

    commands.insert_resource(GameState {
        last_enemy_spawn_in_seconds: 0.,
        start_at: 0.,
        status: GameStatus::Running,
    });

    for x in 0..5 {
        spawn_enemy(
            &mut commands,
            &mut meshes,
            &mut materials,
            &window,
            transform_center.translation,
            SIZE,
        );
    }
}

#[derive(PartialEq)]
enum GameStatus {
    Running,
    GameOver,
}

#[derive(Component)]
struct CameraMarker;

#[derive(Resource)]
struct GameState {
    last_enemy_spawn_in_seconds: f32,
    start_at: f32,
    status: GameStatus,
}

impl GameState {
    fn score(&self, now: f32) -> f32 {
        now - self.start_at
    }
}

#[derive(Component)]
struct InfoText;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct Enemy {
    size: f32,
    speed: f32,
    direction: Vec2,
}

#[derive(Component)]
struct Snake {
    size: f32,
    speed: f32,
}

fn show_info(
    time: Res<Time>,
    state: ResMut<GameState>,
    mut text_query: Query<&mut Text, With<InfoText>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let mut text_info = format!(
        "Score: {:.0}\n",
        state.score(time.elapsed_seconds()),
    );

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS)
    {
        if let Some(value) = fps.smoothed()
        {
            let add = format!(
                "FPS: {}\n", format!("{value:.0}")
            );

            text_info.push_str(add.as_str());
        }
    }

    text_query.get_single_mut().unwrap().sections[0].value = text_info;
}

fn spawn_enemy_over_time(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<GameState>,
    mut snake_query: Query<(&mut Snake, &mut Transform)>,
) {
    if state.status == GameStatus::GameOver {
        return;
    }

    if state.last_enemy_spawn_in_seconds < ENEMY_SPAWN_TIME {
        state.last_enemy_spawn_in_seconds += time.delta_seconds();
        return;
    }

    state.last_enemy_spawn_in_seconds = 0.;

    if let Ok((snake, transform)) = snake_query.get_single_mut() {
        spawn_enemy(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_query.single(),
            transform.translation,
            snake.size,
        )
    }
}

fn move_enemies(
    mut enemy_query: Query<(&mut Enemy, &mut Transform)>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    for (mut enemy, mut transform) in enemy_query.iter_mut() {
        let x_min = 0. + enemy.size;
        let x_max = window.width() - enemy.size;

        let y_min = 0. + enemy.size;
        let y_max = window.height() - enemy.size;

        if transform.translation.x < x_min || transform.translation.x > x_max {
            enemy.direction.x *= -1.;
        } else if transform.translation.y < y_min || transform.translation.y > y_max {
            enemy.direction.y *= -1.;
        }

        let sum = enemy.direction * enemy.speed * time.delta_seconds();

        transform.translation += Transform::from_xyz(sum.x, sum.y, 0.).translation;
    }
}

fn spawn_enemy(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    window: &Window,
    player_position: Vec3,
    player_size: f32,
) {
    use rand::Rng;

    const SIZE: f32 = 20.;

    let enemy_position = loop {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let enemy_position = Vec3::new(
            random_x,
            random_y,
            0.,
        );

        if enemy_position.distance(player_position) > player_size * 10. {
            break enemy_position;
        }
    };
    let direction = (player_position - enemy_position).normalize();

    let speed = rand::thread_rng().gen_range(150..350) as f32;

    commands.spawn((
        Enemy {
            size: SIZE,
            speed: speed,
            direction: direction.truncate(),
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(enemy_position),
            ..default()
        },
    ));
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut snake_query: Query<(&mut Snake, &mut Transform)>,
    time: Res<Time>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    state: Option<ResMut<GameState>>,
    mut restart_event_writer: EventWriter<RestartEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit);
    }

    if let Some(state) = state {
        if state.status == GameStatus::GameOver {
            if keys.just_pressed(KeyCode::Space) {
                restart_event_writer.send(RestartEvent);
            }
            return;
        }
    }

    if let Ok(query) = snake_query.get_single_mut() {
        let (snake, mut transform) = query;

        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
            direction.y += 1.;
        }

        if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
            direction.x -= 1.;
        }

        if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
            direction.x += 1.;
        }

        if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
            direction.y -= 1.;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let window = window_query.single();

        let x_min = 0. + snake.size;
        let x_max = window.width() - snake.size;

        let y_min = 0. + snake.size;
        let y_max = window.height() - snake.size;

        let new_position = (transform.translation + direction * snake.speed * time.delta_seconds());

        if new_position.x < x_min || new_position.x > x_max {
            return;
        } else if new_position.y < y_min || new_position.y > y_max {
            return;
        }

        transform.translation = new_position;
    }
}

fn enemy_hit_player(
    mut commands: Commands,
    mut snake_query: Query<(&Snake, &Transform)>,
    enemy_query: Query<(&Enemy, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<GameState>,
) {
    if state.status == GameStatus::GameOver {
        return;
    }

    if let Ok((snake, snake_transform)) = snake_query.get_single_mut() {
        for (enemy, enemy_transform) in enemy_query.iter() {
            if snake_transform.translation.distance(enemy_transform.translation) < enemy.size + snake.size {
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
                        }))
                );

                state.status = GameStatus::GameOver;
            }
        }
    }
}

fn restart_event_listener(
    mut commands: Commands,
    time: Res<Time>,
    mut events: EventReader<RestartEvent>,
    mut snake_query: Query<(&Snake, &mut Transform)>,
    enemy_query: Query<Entity, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<GameState>,
    text_query: Query<Entity, With<GameOverText>>,
) {
    for _ in events.read() {
        state.status = GameStatus::Running;
        state.start_at = time.elapsed_seconds();
        state.last_enemy_spawn_in_seconds = 0.;

        enemy_query.for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });

        let window = window_query.single();

        let (_, mut snake_transform) = snake_query.single_mut();

        snake_transform.translation.x = window.width() / 2.;
        snake_transform.translation.y = window.height() / 2.;

        text_query.for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });

        return;
    }
}