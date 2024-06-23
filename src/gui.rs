use crate::components::SelectableEntity;
use crate::entity_selection::Selected;
use crate::SpriteHandles;
use bevy::prelude::{Commands, Query, Res, Resource, With};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts};

#[derive(Default)]
struct SelectableCount {
    pub stations: u32,
    pub ships: u32,
}

impl SelectableCount {
    pub fn total(&self) -> u32 {
        self.stations + self.ships
    }
}

#[derive(Resource)]
pub struct UiImages {
    pub ship: SizedTexture,
    pub station: SizedTexture,
}

pub fn initialize(mut commands: Commands, mut contexts: EguiContexts, sprites: Res<SpriteHandles>) {
    let images = UiImages {
        ship: SizedTexture::new(contexts.add_image(sprites.ship.clone()), [16.0, 16.0]),
        station: SizedTexture::new(contexts.add_image(sprites.station.clone()), [16.0, 16.0]),
    };

    commands.insert_resource(images);
}

pub fn list_selection(
    mut context: EguiContexts,
    images: Res<UiImages>,
    selected: Query<&SelectableEntity, With<Selected>>,
) {
    let counts = selected
        .iter()
        .fold(SelectableCount::default(), |mut acc, x| {
            match x {
                SelectableEntity::Station => acc.stations += 1,
                SelectableEntity::Ship => acc.ships += 1,
            }
            acc
        });

    if counts.total() == 0 {
        return;
    }

    egui::Window::new("Selection Overview")
        .anchor(Align2::CENTER_BOTTOM, egui::Vec2::ZERO)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .show(context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if counts.stations > 0 {
                    ui.image(images.station);
                    ui.label(format!("x {}", counts.stations));
                }
                if counts.ships > 0 {
                    ui.image(images.ship);
                    ui.label(format!("x {}", counts.ships));
                }
            });
        });
}
