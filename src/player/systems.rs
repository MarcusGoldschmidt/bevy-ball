use crate::enemy::Enemy;
use crate::phase::{PhaseState, PhaseStatus};
use crate::player::player::Player;
use crate::player::PlayerMovedEvent;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{EventWriter, KeyCode, Query, Res, ResMut, Transform, Window, With};
use bevy::time::Time;
use bevy::window::PrimaryWindow;

pub fn enemy_hit_player(
    mut player_query: Query<(&Player, &Transform)>,
    enemy_query: Query<(&Enemy, &Transform)>,
    state: ResMut<PhaseState>,
) {
    if state.status != PhaseStatus::Running {
        return;
    }

    if let Ok((player, player_transform)) = player_query.get_single_mut() {
        for (enemy, enemy_transform) in enemy_query.iter() {
            if player_transform
                .translation
                .distance(enemy_transform.translation)
                < enemy.size + player.size
            {
                // TODO: Adicionar dano
            }
        }
    }
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_moved_event: EventWriter<PlayerMovedEvent>,
) {
    if let Ok(query) = player_query.get_single_mut() {
        let (player, mut transform) = query;

        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            direction.y += 1.;
        }

        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.;
        }

        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            direction.x += 1.;
        }

        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let window = window_query.single();

        let x_min = 0. + player.size;
        let x_max = window.width() - player.size;

        let y_min = 0. + player.size;
        let y_max = window.height() - player.size;

        let new_position = transform.translation + direction * player.speed * time.delta_secs();

        if new_position.x < x_min || new_position.x > x_max {
            return;
        } else if new_position.y < y_min || new_position.y > y_max {
            return;
        }

        transform.translation = new_position;

        player_moved_event.send(PlayerMovedEvent);
    }
}