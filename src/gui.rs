use bevy::prelude::{
    AssetServer, Commands, Entity, Name, NextState, Query, Res, ResMut, Resource, State, States,
    With,
};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align2, Ui};
use bevy_egui::{egui, EguiContexts};

use crate::components::{
    BuyOrders, Gate, Inventory, SelectableEntity, SellOrders, TradeOrder, Velocity,
};
use crate::entity_selection::Selected;
use crate::game_data::GameData;
use crate::production::{ProductionComponent, ShipyardComponent};
use crate::session_data::SessionData;
use crate::ship_ai::TaskInsideQueue;
use crate::ship_ai::TaskQueue;
use crate::utils::ExchangeWareData;
use crate::utils::SimulationTime;
use crate::SpriteHandles;

#[derive(Default)]
struct SelectableCount {
    pub gates: u32,
    pub stations: u32,
    pub ships: u32,
}

impl SelectableCount {
    pub fn total(&self) -> u32 {
        self.stations + self.ships + self.gates
    }

    pub fn add(mut self, selectable_entity: &SelectableEntity) -> Self {
        match selectable_entity {
            SelectableEntity::Gate => self.gates += 1,
            SelectableEntity::Station => self.stations += 1,
            SelectableEntity::Ship => self.ships += 1,
        }
        self
    }
}

#[derive(Resource)]
pub struct UiIcons {
    pub gate: SizedTexture,
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
            SelectableEntity::Gate => self.gate,
            SelectableEntity::Station => self.station,
            SelectableEntity::Ship => self.ship,
        }
    }

    pub fn get_task(&self, task: &TaskInsideQueue) -> SizedTexture {
        match task {
            TaskInsideQueue::UseGate { .. } => self.move_to,
            TaskInsideQueue::MoveToEntity { .. } => self.move_to,
            TaskInsideQueue::ExchangeWares { data, .. } => match data {
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
        gate: SizedTexture::new(contexts.add_image(sprites.gate.clone()), ICON_SIZE),
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
                if counts.gates > 0 {
                    ui.image(images.gate);
                    ui.label(format!("x {}", counts.gates));
                }
            });
        });
}

#[allow(clippy::type_complexity)]
pub fn list_selection_details(
    game_data: Res<GameData>,
    session_data: Res<SessionData>,
    mut context: EguiContexts,
    simulation_time: Res<SimulationTime>,
    images: Res<UiIcons>,
    selected: Query<
        (
            Entity,
            &SelectableEntity,
            &Name,
            Option<&Inventory>,
            Option<&Velocity>,
            Option<&TaskQueue>,
            Option<&BuyOrders>,
            Option<&SellOrders>,
            Option<&ProductionComponent>,
            Option<&ShipyardComponent>,
            Option<&Gate>,
        ),
        With<Selected>,
    >,
    names: Query<&Name>,
) {
    let counts = selected
        .iter()
        .fold(SelectableCount::default(), |acc, x| acc.add(x.1));

    if counts.total() == 0 {
        return;
    }

    if counts.total() == 1 {
        let now = simulation_time.now();
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
                    inventory,
                    velocity,
                    task_queue,
                    buy_orders,
                    sell_orders,
                    production_module,
                    shipyard,
                    _,
                ) = selected.single();
                draw_ship_summary_row(
                    &images, ui, selectable, name, inventory, velocity, task_queue,
                );

                if let Some(inventory) = inventory {
                    ui.heading("Inventory");
                    let inventory = inventory.inventory();
                    if inventory.is_empty() {
                        ui.label("Empty");
                    } else {
                        for (item_id, amount) in inventory {
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
                }

                if let Some(production) = production_module {
                    ui.heading("Production");
                    for (id, module) in &production.modules {
                        let definition = game_data.production_modules.get(id).unwrap();
                        ui.label(format!("  {}x {}", module.amount, definition.name));
                        let recipe = game_data.item_recipes.get(&module.recipe).unwrap();
                        ui.label(format!("    Active Recipe: {}", recipe.name));
                        if let Some(finished_at) = module.current_run_finished_at {
                            ui.label(format!(
                                "      Done in {}",
                                now.remaining_time(finished_at).as_secs() + 1
                            ));
                        } else {
                            ui.label("    (Inactive)");
                        }
                    }
                }

                if let Some(buy_orders) = buy_orders {
                    ui.heading("Buy Orders");
                    for (item_id, data) in buy_orders.orders() {
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
                    for (item_id, data) in sell_orders.orders() {
                        ui.label(format!(
                            "Selling {}x{} for {}C",
                            data.amount,
                            game_data.items.get(item_id).unwrap().name,
                            data.price
                        ));
                    }
                }

                if let Some(shipyard) = shipyard {
                    ui.heading("Ship Construction");
                    for (id, module) in &shipyard.modules {
                        let definition = game_data.shipyard_modules.get(id).unwrap();
                        ui.label(format!("{}x {}", module.amount, definition.name));

                        for order in &module.active {
                            let definition = session_data
                                .ship_configurations
                                .get(&order.ship_config)
                                .unwrap();

                            ui.label(format!(
                                "  - {} | {}",
                                definition.name,
                                now.remaining_time(order.finished_at).as_secs() + 1
                            ));
                        }
                    }
                    if !shipyard.queue.is_empty() {
                        ui.label(format!("Queue: {} Ships", shipyard.queue.len()));
                    }
                }

                if let Some(task_queue) = task_queue {
                    ui.heading("Tasks");

                    if task_queue.is_empty() {
                        ui.image(images.idle);
                        ui.label("Idle");
                    } else {
                        for task in &task_queue.queue {
                            ui.horizontal(|ui| {
                                ui.image(images.get_task(task));
                                ui.label(match task {
                                    TaskInsideQueue::UseGate { exit_sector, .. } => {
                                        format!(
                                            "Using gate to {}",
                                            names.get(exit_sector.get()).unwrap()
                                        )
                                    }
                                    TaskInsideQueue::MoveToEntity { target } => {
                                        format!("Move to {}", names.get(*target).unwrap())
                                    }
                                    TaskInsideQueue::ExchangeWares { data, .. } => match data {
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
            for (_, selectable, name, storage, velocity, task_queue, _, _, _, _, _) in
                selected.iter()
            {
                draw_ship_summary_row(&images, ui, selectable, name, storage, velocity, task_queue);
            }
        });
}

fn draw_ship_summary_row(
    images: &UiIcons,
    ui: &mut Ui,
    selectable: &SelectableEntity,
    name: &Name,
    inventory: Option<&Inventory>,
    velocity: Option<&Velocity>,
    task_queue: Option<&TaskQueue>,
) {
    ui.horizontal(|ui| {
        ui.image(images.get_selectable(selectable));
        ui.label(format!("{}", name));

        if let Some(task_queue) = task_queue {
            if let Some(task) = task_queue.queue.front() {
                match task {
                    TaskInsideQueue::MoveToEntity { .. } => {
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

        if let Some(inventory) = inventory {
            ui.label(format!("{:.0}%", inventory.ratio() * 100.0));
        }

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
