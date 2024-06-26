use crate::components::{
    BuyOrders, ExchangeWareData, Inventory, ProductionModule, SelectableEntity, SellOrders,
    ShipTask, TaskQueue, Velocity,
};
use crate::data::GameData;
use crate::entity_selection::Selected;
use crate::simulation_time::SimulationTime;
use crate::SpriteHandles;
use bevy::prelude::{
    AssetServer, Commands, Entity, Name, NextState, Query, Res, ResMut, Resource, State, States,
    With,
};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align2, Ui};
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
pub struct UiIcons {
    pub ship: SizedTexture,
    pub station: SizedTexture,

    pub idle: SizedTexture,
    pub move_to: SizedTexture,
    pub buy: SizedTexture,
    pub sell: SizedTexture,
}

impl UiIcons {
    pub fn get_selectable(&self, selectable: &SelectableEntity) -> SizedTexture {
        match selectable {
            SelectableEntity::Station => self.station,
            SelectableEntity::Ship => self.ship,
        }
    }

    pub fn get_task(&self, task: &ShipTask) -> SizedTexture {
        match task {
            ShipTask::DoNothing => self.idle,
            ShipTask::MoveTo(_) => self.move_to,
            ShipTask::ExchangeWares(_, data) => match data {
                ExchangeWareData::Buy(_, _) => self.buy,
                ExchangeWareData::Sell(_, _) => self.sell,
            },
        }
    }
}

pub fn initialize(
    mut commands: Commands,
    mut contexts: EguiContexts,
    sprites: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    let idle = asset_server.load("ui_icons/idle.png");
    let move_to = asset_server.load("ui_icons/move_to.png");
    let buy = asset_server.load("ui_icons/buy.png");
    let sell = asset_server.load("ui_icons/sell.png");

    const ICON_SIZE: [f32; 2] = [16.0, 16.0];

    let icons = UiIcons {
        ship: SizedTexture::new(contexts.add_image(sprites.ship.clone()), ICON_SIZE),
        station: SizedTexture::new(contexts.add_image(sprites.station.clone()), ICON_SIZE),
        idle: SizedTexture::new(contexts.add_image(idle), ICON_SIZE),
        move_to: SizedTexture::new(contexts.add_image(move_to), ICON_SIZE),
        buy: SizedTexture::new(contexts.add_image(buy), ICON_SIZE),
        sell: SizedTexture::new(contexts.add_image(sell), ICON_SIZE),
    };

    commands.insert_resource(icons);
}

pub fn list_selection_icons_and_counts(
    mut context: EguiContexts,
    images: Res<UiIcons>,
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
    game_data: Res<GameData>,
    mut context: EguiContexts,
    simulation_time: Res<SimulationTime>,
    images: Res<UiIcons>,
    selected: Query<
        (
            Entity,
            &SelectableEntity,
            &Name,
            &Inventory,
            Option<&Velocity>,
            Option<&TaskQueue>,
            Option<&BuyOrders>,
            Option<&SellOrders>,
            Option<&ProductionModule>,
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

    if counts.total() == 1 {
        egui::Window::new("Selection Details")
            .anchor(Align2::LEFT_CENTER, egui::Vec2::ZERO)
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .show(context.ctx_mut(), |ui| {
                let (
                    _,
                    selectable,
                    name,
                    storage,
                    velocity,
                    task_queue,
                    buy_orders,
                    sell_orders,
                    production_module,
                ) = selected.single();
                draw_ship_summary_row(&images, ui, selectable, name, storage, velocity, task_queue);

                ui.heading("Inventory");
                let inventory = storage.inventory();
                if inventory.is_empty() {
                    ui.label("Empty");
                } else {
                    for (item_id, amount) in storage.inventory() {
                        let item = game_data.items.get(item_id).unwrap();
                        ui.label(format!(
                            "{} x {} (+{}, -{}, +{}(Prod))",
                            item.name,
                            amount.currently_available,
                            amount.planned_buying,
                            amount.planned_selling,
                            amount.planned_producing
                        ));
                    }
                }

                if let Some(production) = production_module {
                    ui.heading("Production");
                    let recipe = game_data.item_recipes.get(&production.recipe).unwrap();
                    ui.label(format!("Active Recipe: {}", recipe.name));
                    if let Some(finished_at) = production.current_run_finished_at {
                        ui.label(format!(
                            "Done in {}",
                            finished_at - simulation_time.seconds()
                        ));
                    } else {
                        ui.label("(Inactive)");
                    }
                }

                if let Some(buy_orders) = buy_orders {
                    ui.heading("Buy Orders");
                    for (item_id, data) in &buy_orders.orders {
                        ui.label(format!(
                            "Buying {}x{} for {}C",
                            data.amount,
                            game_data.items.get(item_id).unwrap().name,
                            data.price
                        ));
                    }
                }
                if let Some(sell_orders) = sell_orders {
                    ui.heading("Sell Orders");
                    for (item_id, data) in &sell_orders.orders {
                        ui.label(format!(
                            "Selling {}x{} for {}C",
                            data.amount,
                            game_data.items.get(item_id).unwrap().name,
                            data.price
                        ));
                    }
                }

                if let Some(task_queue) = task_queue {
                    ui.heading("Tasks");
                    for task in &task_queue.queue {
                        ui.horizontal(|ui| {
                            ui.image(images.get_task(task));
                            ui.label(match task {
                                ShipTask::DoNothing => "Idle".into(),
                                ShipTask::MoveTo(entity) => format!("Move to {entity}"),
                                ShipTask::ExchangeWares(_, data) => match data {
                                    ExchangeWareData::Buy(item_id, amount) => {
                                        format!(
                                            "Buy {amount}x{}",
                                            game_data.items.get(item_id).unwrap().name
                                        )
                                    }
                                    ExchangeWareData::Sell(item_id, amount) => {
                                        format!(
                                            "Sell {amount}x{}",
                                            game_data.items.get(item_id).unwrap().name
                                        )
                                    }
                                },
                            });
                        });
                    }
                }
            });

        return;
    }

    egui::Window::new("Selection Details")
        .anchor(Align2::LEFT_CENTER, egui::Vec2::ZERO)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .show(context.ctx_mut(), |ui| {
            for (_, selectable, name, storage, velocity, task_queue, _, _, _) in selected.iter() {
                draw_ship_summary_row(&images, ui, selectable, name, storage, velocity, task_queue);
            }
        });
}

fn draw_ship_summary_row(
    images: &UiIcons,
    ui: &mut Ui,
    selectable: &SelectableEntity,
    name: &Name,
    inventory: &Inventory,
    velocity: Option<&Velocity>,
    task_queue: Option<&TaskQueue>,
) {
    ui.horizontal(|ui| {
        ui.image(images.get_selectable(selectable));
        ui.label(format!("{}", name));

        if let Some(task_queue) = task_queue {
            if let Some(task) = task_queue.queue.front() {
                match task {
                    ShipTask::MoveTo(_) => {
                        ui.image(images.get_task(task));
                        if let Some(next_task) = task_queue.queue.get(1) {
                            ui.image(images.get_task(next_task));
                        }
                    }
                    _ => {
                        ui.image(images.get_task(task));
                    }
                }
            }
        }

        ui.label(format!("{:.0}%", inventory.ratio() * 100.0));

        if let Some(velocity) = velocity {
            ui.label(format!("{:.0}u/s", velocity.forward));
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
