use crate::phase::PhaseState;
use crate::shared::InfoText;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{Query, Res, Text, With};

pub fn show_info(
    state: Res<PhaseState>,
    mut text_query: Query<&mut Text, With<InfoText>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let mut text_info = format!("Score: {:.0}\n", state.score());

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            let add = format!("FPS: {}\n", format!("{value:.0}"));

            text_info.push_str(add.as_str());
        }
    }

    text_info.push_str(format!("Spawn Time: {:.1}\n", state.enemy_spawn_time).as_str());

    text_query.get_single_mut().unwrap().0 = text_info;
}
