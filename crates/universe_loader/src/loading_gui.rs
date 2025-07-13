use crate::{LoadingCounts, LoadingState};
use bevy::prelude::{Res, State};
use bevy_egui::egui::Align2;
use bevy_egui::{EguiContexts, egui};
use common::constants::BevyResult;
use persistence::data::{
    GatePairSaveData, SaveDataCollection, SectorSaveData, ShipSaveData, StationSaveData,
};

pub(crate) fn display_loading_information(
    mut context: EguiContexts,
    state: Res<State<LoadingState>>,
    counts: Res<LoadingCounts>,

    sectors: Res<SaveDataCollection<SectorSaveData>>,
    gates: Res<SaveDataCollection<GatePairSaveData>>,
    stations: Res<SaveDataCollection<StationSaveData>>,
    ships: Res<SaveDataCollection<ShipSaveData>>,
) -> BevyResult {
    let text = match state.get() {
        LoadingState::Initialize => "Initializing".to_string(),
        LoadingState::Sectors => get_text("Sectors", sectors.data.len(), counts.sector_count),
        LoadingState::Gates => get_text("Gates", gates.data.len() * 2, counts.gate_count * 2),
        LoadingState::Stations => get_text("Stations", stations.data.len(), counts.station_count),
        LoadingState::Ships => get_text("Ships", ships.data.len(), counts.ship_count),
        LoadingState::Done => "Done!".to_string(),
    };

    egui::Window::new("Loading Status")
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .show(context.ctx_mut()?, |ui| {
            ui.label(text);
        });

    Ok(())
}

fn get_text(what: &str, remaining: usize, total: usize) -> String {
    format!("Loading {what} ({} / {total})", total - remaining,)
}
