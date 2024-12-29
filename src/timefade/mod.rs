use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;

pub struct TimeFadePlugin;

impl Plugin for TimeFadePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_and_fade_particles);
    }
}

#[derive(Component)]
#[require(MeshMaterial2d<ColorMaterial>, Transform)]
pub struct MoveAndFade {
    pub speed: f32,
    pub direction: Vec2,
    pub deceleration: f32,
    pub timer: Timer,
}

pub fn move_and_fade_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut death_query: Query<(
        Entity,
        &mut MoveAndFade,
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
