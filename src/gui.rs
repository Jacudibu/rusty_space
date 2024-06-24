use crate::components::{SelectableEntity, ShipTask, Storage, TaskQueue, Velocity};
use crate::entity_selection::Selected;
use crate::SpriteHandles;
use bevy::prelude::{
    Commands, Entity, Name, NextState, Query, Res, ResMut, Resource, State, States, With,
};
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

    pub fn add(mut self, selectable_entity: &SelectableEntity) -> Self {
        match selectable_entity {
            SelectableEntity::Station => self.stations += 1,
            SelectableEntity::Ship => self.ships += 1,
        }
        self
    }
}

#[derive(Resource)]
pub struct UiImages {
    pub ship: SizedTexture,
    pub station: SizedTexture,
}

impl UiImages {
    pub fn get(&self, selectable: &SelectableEntity) -> SizedTexture {
        match selectable {
            SelectableEntity::Station => self.station,
            SelectableEntity::Ship => self.ship,
        }
    }
}

pub fn initialize(mut commands: Commands, mut contexts: EguiContexts, sprites: Res<SpriteHandles>) {
    let images = UiImages {
        ship: SizedTexture::new(contexts.add_image(sprites.ship.clone()), [16.0, 16.0]),
        station: SizedTexture::new(contexts.add_image(sprites.station.clone()), [16.0, 16.0]),
    };

    commands.insert_resource(images);
}

pub fn list_selection_icons_and_counts(
    mut context: EguiContexts,
    images: Res<UiImages>,
    selected: Query<&SelectableEntity, With<Selected>>,
) {
    let counts = selected
        .iter()
        .fold(SelectableCount::default(), |acc, x| acc.add(x));

    if counts.total() == 0 {
        return;
    }

    egui::Window::new("Selection Icons and Counts")
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

#[allow(clippy::type_complexity)]
pub fn list_selection_details(
    mut context: EguiContexts,
    images: Res<UiImages>,
    selected: Query<
        (
            Entity,
            &SelectableEntity,
            &Name,
            &Storage,
            Option<&Velocity>,
            Option<&TaskQueue>,
        ),
        With<Selected>,
    >,
) {
    let counts = selected
        .iter()
        .fold(SelectableCount::default(), |acc, x| acc.add(x.1));

    if counts.total() == 0 {
        return;
    }

    egui::Window::new("Selection Details")
        .anchor(Align2::LEFT_CENTER, egui::Vec2::ZERO)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .show(context.ctx_mut(), |ui| {
            for (_, selectable, name, storage, velocity, task_queue) in selected.iter() {
                ui.horizontal(|ui| {
                    ui.image(images.get(selectable));
                    ui.label(format!("{}", name));
                    ui.label(format!("{:.0}%", storage.ratio() * 100.0));

                    if let Some(velocity) = velocity {
                        ui.label(format!("{:.0}u/s", velocity.forward));
                    }

                    if let Some(task_queue) = task_queue {
                        if let Some(task) = task_queue.queue.front() {
                            ui.label(match task {
                                ShipTask::DoNothing => "Idle",
                                ShipTask::MoveTo(_) => "Move",
                                ShipTask::ExchangeWares(_, _) => "Trade",
                            });
                        }
                    }
                });
            }
        });
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MouseCursorOverUiState {
    #[default]
    NotOverUI,
    OverUI,
}

pub fn detect_mouse_cursor_over_ui(
    mut egui: EguiContexts,
    current_mouse_state: Res<State<MouseCursorOverUiState>>,
    mut next_state: ResMut<NextState<MouseCursorOverUiState>>,
) {
    if egui.ctx_mut().is_pointer_over_area() {
        if current_mouse_state.get() != &MouseCursorOverUiState::OverUI {
            next_state.set(MouseCursorOverUiState::OverUI);
        }
    } else if current_mouse_state.get() != &MouseCursorOverUiState::NotOverUI {
        next_state.set(MouseCursorOverUiState::NotOverUI);
    }
}
