use crate::debug::systems::show_info;
use bevy::app::{App, Plugin, Update};

mod systems;

pub struct ShowInfoPlugin;

impl Plugin for ShowInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_info);
    }
}
