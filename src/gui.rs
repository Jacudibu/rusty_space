use crate::components::{
    Asteroid, BuyOrders, Gate, InSector, InteractionQueue, Inventory, SelectableEntity, SellOrders,
    Ship, TradeOrder,
};
use crate::entity_selection::{MouseCursor, Selected};
use crate::game_data::{AsteroidDataId, AsteroidManifest, GameData, IRON_ASTEROID_ID};
use crate::map_layout::MapLayout;
use crate::session_data::ship_configs::ShipConfigurationAddedEvent;
use crate::session_data::{
    SessionData, ShipConfigId, ShipConfiguration, ShipConfigurationManifest,
};
use crate::simulation::physics::ShipVelocity;
use crate::simulation::prelude::SimulationTime;
use crate::simulation::production::{ProductionComponent, ShipyardComponent};
use crate::simulation::ship_ai::TaskInsideQueue;
use crate::simulation::ship_ai::TaskQueue;
use crate::utils::ExchangeWareData;
use crate::SpriteHandles;
use bevy::app::App;
use bevy::prelude::{
    on_event, AppExtStates, AssetServer, Commands, Entity, EventReader, IntoSystemConfigs, Name,
    NextState, Plugin, PreUpdate, Query, Res, ResMut, Resource, Startup, State, States, Update,
    With,
};
use bevy::utils::{HashMap, HashSet};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align2, Shadow, Ui};
use bevy_egui::{egui, EguiContexts, EguiStartupSet};

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MouseCursorOverUiState>()
            .insert_resource(GuiDataCache {
                ships_configs: Default::default(),
            })
            .add_systems(
                Startup,
                initialize
                    .after(EguiStartupSet::InitContexts)
                    .after(crate::initialize_data),
            )
            .add_systems(PreUpdate, detect_mouse_cursor_over_ui)
            .add_systems(
                Update,
                (
                    draw_sector_info,
                    list_selection_icons_and_counts,
                    list_selection_details,
                    on_ship_configuration_added.run_if(on_event::<ShipConfigurationAddedEvent>),
                ),
            );
    }
}

struct SelectableCount {
    pub asteroids: HashMap<AsteroidDataId, u32>,
    pub gates: u32,
    pub stations: u32,
    pub ships: HashMap<ShipConfigId, u32>,
    pub planets: u32,
    pub stars: u32,
}

impl SelectableCount {
    // TODO: manifest argument results should just be cached somewhere
    pub fn new(asteroid_manifest: &AsteroidManifest, gui_data: &GuiDataCache) -> Self {
        Self {
            asteroids: asteroid_manifest.iter().map(|(id, _)| (*id, 0)).collect(),
            gates: 0,
            ships: gui_data.ships_configs.iter().map(|id| (*id, 0)).collect(),
            stations: 0,
            planets: 0,
            stars: 0,
        }
    }

    pub fn total(&self) -> u32 {
        self.stations
            + self.ships.values().sum::<u32>()
            + self.gates
            + self.asteroids.values().sum::<u32>()
            + self.stars
            + self.planets
    }

    pub fn add(mut self, selectable_entity: &SelectableEntity) -> Self {
        match selectable_entity {
            SelectableEntity::Asteroid(id) => *self.asteroids.get_mut(id).unwrap() += 1,
            SelectableEntity::Gate => self.gates += 1,
            SelectableEntity::Planet => self.planets += 1,
            SelectableEntity::Ship(id) => *self.ships.get_mut(id).unwrap() += 1,
            SelectableEntity::Star => self.stars += 1,
            SelectableEntity::Station => self.stations += 1,
        }
        self
    }
}

#[derive(Resource)]
pub struct UiIcons {
    pub asteroids: HashMap<AsteroidDataId, SizedTexture>,
    pub gate: SizedTexture,
    pub planet: SizedTexture,
    pub ships: HashMap<ShipConfigId, SizedTexture>,
    pub star: SizedTexture,
    pub station: SizedTexture,

    pub awaiting_signal: SizedTexture,
    pub idle: SizedTexture,
    pub move_to: SizedTexture,
    pub buy: SizedTexture,
    pub sell: SizedTexture,
    pub dock_at: SizedTexture,
    pub undock: SizedTexture,
}

impl UiIcons {
    pub fn get_selectable(&self, selectable: &SelectableEntity) -> SizedTexture {
        match selectable {
            SelectableEntity::Asteroid(id) => self.asteroids[id],
            SelectableEntity::Gate => self.gate,
            SelectableEntity::Planet => self.planet,
            SelectableEntity::Ship(id) => self.ships[id],
            SelectableEntity::Star => self.star,
            SelectableEntity::Station => self.station,
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
            TaskInsideQueue::MineAsteroid { .. } => *self.asteroids.get(&IRON_ASTEROID_ID).unwrap(),
            TaskInsideQueue::HarvestGas { .. } => self.planet,
            TaskInsideQueue::AwaitingSignal => self.awaiting_signal,
            TaskInsideQueue::RequestAccess { .. } => self.awaiting_signal,
            TaskInsideQueue::DockAtEntity { .. } => self.dock_at,
            TaskInsideQueue::Undock { .. } => self.undock,
        }
    }
}

const ICON_SIZE: [f32; 2] = [16.0, 16.0];

pub fn initialize(
    mut commands: Commands,
    mut contexts: EguiContexts,
    sprites: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
    asteroid_manifest: Res<AsteroidManifest>,
) {
    contexts
        .ctx_mut()
        .style_mut(|style| style.visuals.window_shadow = Shadow::NONE);

    let awaiting_signal = asset_server.load("sprites/task_icons/awaiting_signal.png");
    let idle = asset_server.load("sprites/task_icons/idle.png");
    let move_to = asset_server.load("sprites/task_icons/move_to.png");
    let dock_at = asset_server.load("sprites/task_icons/move_to.png"); // TODO
    let undock = asset_server.load("sprites/task_icons/move_to.png"); // TODO
    let buy = asset_server.load("sprites/task_icons/buy.png");
    let sell = asset_server.load("sprites/task_icons/sell.png");

    let icons = UiIcons {
        asteroids: asteroid_manifest
            .iter()
            .map(|(id, data)| {
                (
                    *id,
                    SizedTexture::new(contexts.add_image(data.sprite.clone()), ICON_SIZE),
                )
            })
            .collect(),
        ships: HashMap::default(),
        gate: SizedTexture::new(contexts.add_image(sprites.gate.clone()), ICON_SIZE),
        planet: SizedTexture::new(contexts.add_image(sprites.planet.clone()), ICON_SIZE),
        star: SizedTexture::new(contexts.add_image(sprites.star.clone()), ICON_SIZE),
        station: SizedTexture::new(contexts.add_image(sprites.station.clone()), ICON_SIZE),
        awaiting_signal: SizedTexture::new(contexts.add_image(awaiting_signal), ICON_SIZE),
        idle: SizedTexture::new(contexts.add_image(idle), ICON_SIZE),
        move_to: SizedTexture::new(contexts.add_image(move_to), ICON_SIZE),
        dock_at: SizedTexture::new(contexts.add_image(dock_at), ICON_SIZE),
        undock: SizedTexture::new(contexts.add_image(undock), ICON_SIZE),
        buy: SizedTexture::new(contexts.add_image(buy), ICON_SIZE),
        sell: SizedTexture::new(contexts.add_image(sell), ICON_SIZE),
    };

    commands.insert_resource(icons);
}

pub fn draw_sector_info(
    mut context: EguiContexts,
    mouse_cursor: Res<MouseCursor>,
    map: Res<MapLayout>,
) {
    let Some(world_pos) = mouse_cursor.world_space else {
        return;
    };

    let coordinates = map.hex_layout.world_pos_to_hex(world_pos);

    egui::Window::new("Sector Info")
        .anchor(Align2::CENTER_TOP, egui::Vec2::ZERO)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .show(context.ctx_mut(), |ui| {
            ui.label(format!("Sector {}/{}", coordinates.x, coordinates.y));
        });
}

#[derive(Resource)]
pub struct GuiDataCache {
    ships_configs: HashSet<ShipConfigId>,
}

pub fn on_ship_configuration_added(
    mut events: EventReader<ShipConfigurationAddedEvent>,
    ship_configs: Res<ShipConfigurationManifest>,
    mut context: EguiContexts,
    mut images: ResMut<UiIcons>,
    mut gui_data: ResMut<GuiDataCache>,
) {
    for event in events.read() {
        gui_data.ships_configs.insert(event.id);
        let data = ship_configs.get_by_id(&event.id).unwrap();
        let image = SizedTexture::new(context.add_image(data.sprite.clone()), ICON_SIZE);
        images.ships.insert(event.id, image);
    }
}

pub fn list_selection_icons_and_counts(
    mut context: EguiContexts,
    images: Res<UiIcons>,
    selected: Query<&SelectableEntity, With<Selected>>,
    asteroid_manifest: Res<AsteroidManifest>,
    gui_data: Res<GuiDataCache>,
) {
    let counts = selected.iter().fold(
        SelectableCount::new(&asteroid_manifest, &gui_data),
        |acc, x| acc.add(x),
    );

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
                if counts.planets > 0 {
                    ui.image(images.planet);
                    ui.label(format!("x {}", counts.planets));
                }
                if counts.stars > 0 {
                    ui.image(images.star);
                    ui.label(format!("x {}", counts.stars));
                }
                if counts.gates > 0 {
                    ui.image(images.gate);
                    ui.label(format!("x {}", counts.gates));
                }
                for (id, count) in &counts.ships {
                    if count == &0 {
                        continue;
                    }

                    ui.image(images.ships[id]);
                    ui.label(format!("x {}", count));
                }
                for (id, count) in &counts.asteroids {
                    if count == &0 {
                        continue;
                    }

                    ui.image(images.asteroids[id]);
                    ui.label(format!("x {}", count));
                }
            });
        });
}

#[allow(clippy::type_complexity)]
pub fn list_selection_details(
    game_data: GameData,
    session_data: SessionData,
    mut context: EguiContexts,
    simulation_time: Res<SimulationTime>,
    images: Res<UiIcons>,
    gui_data: Res<GuiDataCache>,
    selected: Query<
        (
            Entity,
            &SelectableEntity,
            &Name,
            Option<&Ship>,
            Option<&Inventory>,
            Option<&Asteroid>,
            Option<&ShipVelocity>,
            Option<&TaskQueue>,
            Option<&BuyOrders>,
            Option<&SellOrders>,
            Option<&ProductionComponent>,
            Option<&ShipyardComponent>,
            Option<&Gate>,
            Option<&InSector>,
            Option<&InteractionQueue>,
        ),
        With<Selected>,
    >,
    names: Query<&Name>,
) {
    let counts = selected.iter().fold(
        SelectableCount::new(&game_data.asteroids, &gui_data),
        |acc, x| acc.add(x.1),
    );

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
                    ship,
                    inventory,
                    asteroid,
                    velocity,
                    task_queue,
                    buy_orders,
                    sell_orders,
                    production_module,
                    shipyard,
                    _,
                    in_sector,
                    interaction_queue,
                ) = selected.single();
                draw_ship_summary_row(
                    &images, ui, selectable, name, inventory, velocity, task_queue,
                );

                if let Some(in_sector) = in_sector {
                    ui.label(format!(
                        "In sector {}",
                        names.get(in_sector.sector.into()).unwrap()
                    ));
                }

                if let Some(interaction_queue) = interaction_queue {
                    ui.label(format!(
                        "Interaction Queue at {}/{}",
                        interaction_queue.currently_interacting(),
                        interaction_queue.maximum_interactions()
                    ));
                }

                if let Some(inventory) = inventory {
                    ui.heading("Inventory");
                    let inventory = inventory.inventory();
                    if inventory.is_empty() {
                        ui.label("Empty");
                    } else {
                        for (item_id, amount) in inventory {
                            let item = game_data.items.get_by_ref(item_id).unwrap();
                            ui.label(format!(
                                "{} x {} (+{}, -{}, +{}(Prod))",
                                item.name,
                                amount.current,
                                amount.planned_buying,
                                amount.planned_selling,
                                amount.planned_producing
                            ));
                        }
                    }
                }

                if let Some(asteroid) = asteroid {
                    ui.label(format!(
                        "Material: {}",
                        game_data
                            .items
                            .get_by_ref(&asteroid.ore_item_id)
                            .unwrap()
                            .name
                    ));
                    let reserved = asteroid.ore - asteroid.remaining_after_reservations;

                    ui.label(format!(
                        "Amount: {}{}",
                        asteroid.ore,
                        if reserved > 0 {
                            format!(" ({} Reserved)", reserved)
                        } else {
                            String::new()
                        }
                    ));
                }

                if let Some(production) = production_module {
                    ui.heading("Production");
                    for (id, module) in &production.modules {
                        let definition = game_data.production_modules.get_by_ref(id).unwrap();
                        ui.label(format!("  {}x {}", module.amount, definition.name));
                        let recipe = game_data.item_recipes.get_by_ref(&module.recipe).unwrap();
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
                            game_data.items.get_by_ref(item_id).unwrap().name,
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
                            game_data.items.get_by_ref(item_id).unwrap().name,
                            data.price
                        ));
                    }
                }

                if let Some(shipyard) = shipyard {
                    ui.heading("Ship Construction");
                    for (id, module) in &shipyard.modules {
                        let definition = game_data.shipyard_modules.get_by_ref(id).unwrap();
                        ui.label(format!("{}x {}", module.amount, definition.name));

                        for order in &module.active {
                            let definition = session_data
                                .ship_configurations
                                .get_by_id(&order.ship_config)
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

                if let Some(ship) = ship {
                    draw_ship_config_stats(
                        ui,
                        session_data
                            .ship_configurations
                            .get_by_id(&ship.config_id())
                            .unwrap(),
                    )
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
                                            names.get(exit_sector.into()).unwrap()
                                        )
                                    }
                                    TaskInsideQueue::MoveToEntity { target, .. } => {
                                        format!("Move to {}", names.get(target.into()).unwrap())
                                    }
                                    TaskInsideQueue::DockAtEntity { target, .. } => {
                                        format!("Dock at {}", names.get(target.into()).unwrap())
                                    }
                                    TaskInsideQueue::Undock => "Undock".to_string(),
                                    TaskInsideQueue::ExchangeWares { data, .. } => match data {
                                        ExchangeWareData::Buy(item_id, amount) => {
                                            format!(
                                                "Buy {amount}x{}",
                                                game_data.items.get_by_ref(item_id).unwrap().name
                                            )
                                        }
                                        ExchangeWareData::Sell(item_id, amount) => {
                                            format!(
                                                "Sell {amount}x{}",
                                                game_data.items.get_by_ref(item_id).unwrap().name
                                            )
                                        }
                                    },
                                    TaskInsideQueue::MineAsteroid { target, .. } => {
                                        format!("Mining {}", names.get(target.into()).unwrap())
                                    }
                                    TaskInsideQueue::HarvestGas { target } => {
                                        format!("Harvesting {}", names.get(target.into()).unwrap())
                                    }
                                    TaskInsideQueue::AwaitingSignal => {
                                        "Awaiting Signal".to_string()
                                    }
                                    TaskInsideQueue::RequestAccess { target } => {
                                        format!(
                                            "Requesting Access to {}",
                                            names.get(target.into()).unwrap()
                                        )
                                    }
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
        .vscroll(true)
        .show(context.ctx_mut(), |ui| {
            for (_, selectable, name, _, storage, _, velocity, task_queue, _, _, _, _, _, _, _) in
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
    velocity: Option<&ShipVelocity>,
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

fn draw_ship_config_stats(ui: &mut Ui, config: &ShipConfiguration) {
    ui.heading("Stats");
    ui.label(format!(
        "Ship Config: {} (v{})",
        config.name, config.id.version
    ));
    ui.label(format!(
        "Inventory Size: {}",
        config.computed_stats.inventory_size
    ));
    ui.label(format!(
        "Engine: Fw{}|Acc{}|Rot{}|RotAcc{}",
        config.computed_stats.engine.max_speed,
        config.computed_stats.engine.acceleration,
        config.computed_stats.engine.max_angular_speed,
        config.computed_stats.engine.angular_acceleration,
    ));

    if let Some(ore_miner) = config.computed_stats.asteroid_mining_amount {
        ui.label(format!("Ore Mining Strength: {}", ore_miner));
    }
    if let Some(gas_harvester) = config.computed_stats.gas_harvesting_amount {
        ui.label(format!("Gas Harvesting Strength: {}", gas_harvester));
    }
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
