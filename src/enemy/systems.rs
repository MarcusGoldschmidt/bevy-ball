use crate::enemy::Enemy;
use crate::health::DamageEvent;
use crate::phase::{EnemySpeed, PhaseState, SpawnEnemyEvent};
use crate::player::player::Player;
use crate::player::PlayerMovedEvent;
use crate::quadtree::QuadTree;
use crate::shot::Bullet;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::log::warn;
use bevy::prelude::*;
use bevy::utils::default;
use rand::random;

pub fn move_enemies(
    mut quad_tree: ResMut<QuadTree<Entity, Enemy>>,
    mut enemy_query: Query<(Entity, &mut Enemy, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut enemy, mut transform) in enemy_query.iter_mut() {
        let sum = enemy.direction * enemy.speed * time.delta_secs();

        let previous_position = transform.translation.clone();

        transform.translation += Transform::from_xyz(sum.x, sum.y, 0.).translation;

        let _ = quad_tree.insert(entity, enemy.clone(), transform.translation.truncate());
    }
}

pub fn spawn_enemy_over_time(
    time: Res<Time>,
    mut state: ResMut<PhaseState>,
    mut spawn_enemy_event_writter: EventWriter<SpawnEnemyEvent>,
) {
    if state.last_enemy_spawn_in_seconds < state.enemy_spawn_time {
        state.last_enemy_spawn_in_seconds += time.delta_secs();
        return;
    };

    let x = random::<f64>() + (state.start_at.elapsed().as_secs_f64()).log10();

    let speed = match x {
        x if x < 0.3 => EnemySpeed::RandomSlow,
        x if x < 0.8 => EnemySpeed::RandomNormal,
        _ => EnemySpeed::RandomFast,
    };

    spawn_enemy_event_writter.send(SpawnEnemyEvent { speed, ..default() });
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

pub fn check_enemy_was_shoot(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Enemy, &Transform)>,
    mut bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut event_writer: EventWriter<DamageEvent>,
    mut quad_tree: ResMut<QuadTree<Entity, Enemy>>,
) {
    for (bullet_entity, bullet_transform, bullet) in bullet_query.iter_mut() {
        // Implement the quad tree check here for near quares for a circle object
        if let Some(a) = quad_tree.find(bullet_transform.translation.truncate()) {
            for (key, value) in a.into_iter() {
                if let Ok((enemy_entity, enemy, enemy_transform)) = enemy_query.get(*key) {
                    if enemy_transform
                        .translation
                        .distance(bullet_transform.translation)
                        < enemy.size
                    {
                        if let Some(e) = commands.get_entity(bullet_entity) {
                            e.try_despawn_recursive()
                        } else {
                            warn!("Entity not found for bullet");
                        }
                        event_writer.send(DamageEvent {
                            entity: enemy_entity,
                            damage: bullet.damage,
                        });
                    }
                }
            }
        }
    }
}
