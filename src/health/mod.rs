use crate::utils::random_direction;
use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::log::warn;
use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;

#[derive(Event)]
pub struct DeathEvent {
    entity: Entity,
}

#[derive(Event)]
pub struct DamageEvent {
    pub entity: Entity,
    pub damage: f32,
}

#[derive(Component)]
pub struct Health2d {
    pub max_health: f32,
    pub health: f32,
}

#[derive(Component)]
struct ExplosionParticle {
    speed: f32,
    direction: Vec2,
    deceleration: f32,
    timer: Timer,
}

impl Health2d {
    pub fn full_health(v: f32) -> Self {
        Self {
            max_health: v,
            health: v,
        }
    }
}

impl Default for Health2d {
    fn default() -> Self {
        Self {
            max_health: 5.,
            health: 5.,
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>()
            .add_systems(Update, death_check)
            .add_systems(Update, damage_listener)
            .add_systems(Update, death_check_listener)
            .add_systems(Update, move_death_particles);
    }
}

pub fn damage_listener(
    mut event_reader: EventReader<DamageEvent>,
    mut query: Query<&mut Health2d>,
) {
    for event in event_reader.read() {
        if let Ok(mut entity) = query.get_mut(event.entity) {
            entity.health = entity.health - event.damage;
        } else {
            warn!("Entity not found for damage");
        }
    }
}

pub fn death_check(
    mut event_writer: EventWriter<DeathEvent>,
    health_query: Query<(Entity, &Health2d)>,
) {
    for (entity, health) in health_query.iter() {
        if health.health <= 0. {
            event_writer.send(DeathEvent {
                entity: entity.clone(),
            });
        }
    }
}

pub fn death_check_listener(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut event_reader: EventReader<DeathEvent>,
    health_query: Query<(&Health2d, &Transform)>,
) {
    for event in event_reader.read() {
        if let Some(entity) = commands.get_entity(event.entity) {
            if let Ok((_, transform)) = health_query.get(event.entity) {
                entity.try_despawn_recursive();

                (0..13).for_each(|_| {
                    commands.spawn((
                        ExplosionParticle {
                            speed: 400.,
                            direction: random_direction(),
                            deceleration: 10.,
                            timer: Timer::from_seconds(0.75, TimerMode::Once),
                        },
                        transform.clone(),
                        Mesh2d(meshes.add(Circle::new(2.))),
                        MeshMaterial2d(materials.add(Color::srgb(7.5, 7.5, 0.))),
                    ));
                });
            }
        }
    }
}

pub fn move_death_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut death_query: Query<(
        Entity,
        &mut ExplosionParticle,
        &mut Transform,
        &mut MeshMaterial2d<ColorMaterial>,
    )>,
) {
    for (entity, mut particle, mut transform, mut material) in death_query.iter_mut() {
        particle.timer.tick(time.delta());

        let sum = particle.direction * particle.speed * time.delta_secs();

        particle.speed -= (particle.deceleration * particle.timer.fraction_remaining());

        if let Some(mut color_material) = materials.get_mut(material.id()) {
            color_material.alpha_mode = AlphaMode2d::Blend;

            color_material
                .color
                .set_alpha(particle.timer.fraction_remaining());
        }

        if particle.timer.finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            transform.translation += Transform::from_xyz(sum.x, sum.y, 0.).translation;
        }
    }
}
