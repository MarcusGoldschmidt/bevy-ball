use crate::player::PlayerReceiveXpEvent;
use crate::timefade::{MoveAndFade, TimeFadePlugin};
use crate::utils::random_direction;
use bevy::app::{App, Plugin, Update};
use bevy::color::Color;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::log::warn;
use bevy::prelude::*;
use rand::{random, Rng};

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

    pub xp_on_death: u32,
}

impl Health2d {
    pub fn full_health(v: f32) -> Self {
        Self {
            max_health: v,
            health: v,
            ..Self::default()
        }
    }
}

impl Default for Health2d {
    fn default() -> Self {
        Self {
            max_health: 5.,
            health: 5.,
            xp_on_death: 1,
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>()
            .add_event::<PlayerReceiveXpEvent>()
            .add_plugins(TimeFadePlugin)
            .add_systems(Update, death_check)
            .add_systems(Update, damage_listener)
            .add_systems(Update, death_check_listener);
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
    mut xp_writer: EventWriter<PlayerReceiveXpEvent>,
    health_query: Query<(&Health2d, &Transform)>,
) {
    for event in event_reader.read() {
        if let Some(entity) = commands.get_entity(event.entity) {
            if let Ok((health, transform)) = health_query.get(event.entity) {
                entity.try_despawn_recursive();

                xp_writer.send(PlayerReceiveXpEvent {
                    xp: health.xp_on_death,
                });

                (0..rand::thread_rng().gen_range(7..20)).for_each(|_| {
                    commands.spawn((
                        MoveAndFade {
                            speed: rand::thread_rng().gen_range(350..450) as f32,
                            direction: random_direction(),
                            deceleration: rand::thread_rng().gen_range(5..10) as f32,
                            timer: Timer::from_seconds(random::<f32>() + 0.3, TimerMode::Once),
                        },
                        transform.clone(),
                        Mesh2d(meshes.add(Circle::new(1.))),
                        MeshMaterial2d(materials.add(Color::srgb(7.5, 7.5, 0.))),
                    ));
                });
            }
        }
    }
}
