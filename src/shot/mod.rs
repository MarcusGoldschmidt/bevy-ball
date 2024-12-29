use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Instant;

#[derive(Component)]
#[require(Transform)]
pub struct Shooter {
    pub fire_rate: f32,
    pub damage: f32,
    pub direction: Vec2,

    pub last_shoot: Instant,

    pub should_shoot: bool,
}

#[derive(Component)]
#[require(Transform)]
pub struct Bullet {
    pub damage: f32,
    pub direction: Vec2,
    pub speed: f32,
    pub size: f32,
}

#[derive(Event)]
pub struct ShootEvent {
    pub damage: f32,
    pub direction: Vec2,
    pub position: Transform,
}

pub struct ShotPlugin;

impl Plugin for ShotPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>()
            .add_systems(Update, shoot_over_time)
            .add_systems(Update, shoot_event_listener)
            .add_systems(Update, move_bullet_and_despawn);
    }
}

pub fn shoot_over_time(
    mut shooter_query: Query<(&mut Shooter, &Transform)>,
    mut event_writer: EventWriter<ShootEvent>,
) {
    for (mut shooter, transform) in shooter_query.iter_mut() {
        if shooter.last_shoot.elapsed().as_secs_f32() < (1. / shooter.fire_rate) {
            return;
        }

        if shooter.should_shoot == false {
            return;
        }

        shooter.last_shoot = Instant::now();

        event_writer.send(ShootEvent {
            damage: shooter.damage,
            direction: shooter.direction,
            position: transform.clone(),
        });
    }
}

pub fn shoot_event_listener(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut event_reader: EventReader<ShootEvent>,
) {
    const BULLET_SIZE: f32 = 2.;

    for event in event_reader.read() {
        commands.spawn((
            Bullet {
                damage: event.damage,
                speed: 600.,
                size: BULLET_SIZE,
                direction: event.direction,
            },
            event.position,
            Mesh2d(meshes.add(Circle::new(BULLET_SIZE))),
            MeshMaterial2d(materials.add(Color::srgb(7.5, 7.5, 0.))),
        ));
    }
}

pub fn move_bullet_and_despawn(
    mut commands: Commands,
    mut bullet_query: Query<((&mut Bullet, Entity), &mut Transform)>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    for ((bullet, entity), mut transform) in bullet_query.iter_mut() {
        let x_min = 0. + bullet.size;
        let x_max = window.width() - bullet.size;

        let y_min = 0. + bullet.size;
        let y_max = window.height() - bullet.size;

        if (transform.translation.x < x_min || transform.translation.x > x_max)
            || (transform.translation.y < y_min || transform.translation.y > y_max)
        {
            commands.entity(entity).despawn_recursive();
            return;
        }

        let sum = bullet.direction * bullet.speed * time.delta_secs();

        transform.translation += Transform::from_xyz(sum.x, sum.y, 0.).translation;
    }
}
