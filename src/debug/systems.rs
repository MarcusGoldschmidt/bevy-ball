use crate::phase::PhaseState;
use crate::shared::InfoText;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{Query, Res, Text, Time, With};

pub fn show_info(
    time: Res<Time>,
    state: Res<PhaseState>,
    mut text_query: Query<&mut Text, With<InfoText>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let mut text_info = format!("Score: {:.0}\n", state.score(time.elapsed_seconds()),);

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            let add = format!("FPS: {}\n", format!("{value:.0}"));

            text_info.push_str(add.as_str());
        }
    }

    text_query.get_single_mut().unwrap().sections[0].value = text_info;
}
