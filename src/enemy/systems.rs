use crate::enemy::Enemy;
use crate::phase::{PhaseState, SpawnEnemyEvent};
use crate::player::player::Player;
use crate::player::PlayerMovedEvent;
use bevy::prelude::{EventReader, EventWriter, Query, Res, ResMut, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;

pub fn move_enemies(
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

pub fn spawn_enemy_over_time(
    time: Res<Time>,
    mut state: ResMut<PhaseState>,
    mut spawn_enemy_event_writter: EventWriter<SpawnEnemyEvent>,
) {
    if state.last_enemy_spawn_in_seconds < state.enemy_spawn_time {
        state.last_enemy_spawn_in_seconds += time.delta_seconds();
        return;
    }

    spawn_enemy_event_writter.send(SpawnEnemyEvent);
}

pub fn follow_player_event_listener(
    mut events: EventReader<PlayerMovedEvent>,
    mut snake_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Enemy, &Transform), With<Enemy>>,
) {
    for _ in events.read() {
        if let Ok(snake_transform) = snake_query.get_single_mut() {
            for (mut enemy, enemy_transform) in enemy_query.iter_mut() {
                let direction =
                    (snake_transform.translation - enemy_transform.translation).normalize();

                enemy.direction = direction.truncate();
            }
        }
    }
}
