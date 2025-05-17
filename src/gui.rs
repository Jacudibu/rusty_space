use bevy::app::App;
use bevy::ecs::query::QueryData;
use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::{
    AppExtStates, AssetServer, Commands, Entity, EventReader, IntoScheduleConfigs, Name, NextState,
    Plugin, PreUpdate, Query, Res, ResMut, Resource, Startup, State, With, on_event,
};
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Align2, Shadow, Ui};
use bevy_egui::{EguiContextPass, EguiContexts, EguiStartupSet, egui};
use common::components::interaction_queue::InteractionQueue;
use common::components::production_facility::ProductionFacility;
use common::components::ship_velocity::ShipVelocity;
use common::components::shipyard::Shipyard;
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{
    Asteroid, BuyOrders, ConstructionSite, ConstructionSiteStatus, Gate, InSector, Inventory,
    SelectableEntity, SellOrders, Ship, Station, TradeOrder,
};
use common::constants::BevyResult;
use common::game_data::{
    AsteroidDataId, AsteroidManifest, Constructable, ConstructableModuleId, GameData,
    IRON_ASTEROID_ID,
};
use common::session_data::ship_configs::ShipConfigurationAddedEvent;
use common::session_data::{
    SessionData, ShipConfigId, ShipConfiguration, ShipConfigurationManifest,
};
use common::simulation_time::SimulationTime;
use common::states::MouseCursorOverUiState;
use common::types::exchange_ware_data::ExchangeWareData;
use common::types::sprite_handles::SpriteHandles;
use entity_selection::components::IsEntitySelected;
use entity_selection::mouse_cursor::MouseCursor;

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
                EguiContextPass,
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
            SelectableEntity::Celestial => self.planets += 1,
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
    pub construct: SizedTexture,
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
            SelectableEntity::Celestial => self.planet,
            SelectableEntity::Ship(id) => self.ships[id],
            SelectableEntity::Star => self.star,
            SelectableEntity::Station => self.station,
        }
    }

    pub fn get_task_icon(&self, task: &TaskKind) -> SizedTexture {
        match task {
            TaskKind::UseGate { .. } => self.move_to,
            TaskKind::MoveToEntity { .. } => self.move_to,
            TaskKind::ExchangeWares { data } => match data.exchange_data {
                ExchangeWareData::Buy(_, _) => self.buy,
                ExchangeWareData::Sell(_, _) => self.sell,
            },
            TaskKind::MineAsteroid { .. } => *self.asteroids.get(&IRON_ASTEROID_ID).unwrap(),
            TaskKind::HarvestGas { .. } => self.planet,
            TaskKind::AwaitingSignal { .. } => self.awaiting_signal,
            TaskKind::RequestAccess { .. } => self.awaiting_signal,
            TaskKind::DockAtEntity { .. } => self.dock_at,
            TaskKind::Undock { .. } => self.undock,
            TaskKind::Construct { .. } => self.construct,
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
    let construct = asset_server.load("sprites/construction_site.png");

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
        construct: SizedTexture::new(contexts.add_image(construct), ICON_SIZE),
    };

    commands.insert_resource(icons);
}

pub fn draw_sector_info(mut context: EguiContexts, mouse_cursor: Res<MouseCursor>) {
    let Some(sector_pos) = &mouse_cursor.sector_space else {
        return;
    };

    egui::Window::new("Sector Info")
        .anchor(Align2::CENTER_TOP, egui::Vec2::ZERO)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .show(context.ctx_mut(), |ui| {
            ui.set_width(120.0);
            ui.vertical_centered(|ui| {
                ui.label(format!(
                    "Sector {}/{}",
                    sector_pos.coordinates.x, sector_pos.coordinates.y
                ));
                ui.label(format!(
                    "[x: {:>4.0} | y: {:>4.0}]",
                    sector_pos.sector_position.local_position.x,
                    sector_pos.sector_position.local_position.y,
                ))
            });
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
    selected: Query<&SelectableEntity, With<IsEntitySelected>>,
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

#[derive(QueryData)]
struct SelectableComponents {
    entity: Entity,
    selectable: &'static SelectableEntity,
    name: &'static Name,
    ship: Option<&'static Ship>,
    inventory: Option<&'static Inventory>,
    asteroid: Option<&'static Asteroid>,
    ship_velocity: Option<&'static ShipVelocity>,
    task_queue: Option<&'static TaskQueue>,
    buy_orders: Option<&'static BuyOrders>,
    sell_orders: Option<&'static SellOrders>,
    station: Option<&'static Station>,
    production: Option<&'static ProductionFacility>,
    shipyard: Option<&'static Shipyard>,
    gate: Option<&'static Gate>,
    in_sector: Option<&'static InSector>,
    interaction_queue: Option<&'static InteractionQueue>,
}

#[allow(clippy::too_many_arguments)]
fn list_selection_details(
    game_data: GameData,
    session_data: SessionData,
    mut context: EguiContexts,
    simulation_time: Res<SimulationTime>,
    images: Res<UiIcons>,
    gui_data: Res<GuiDataCache>,
    selected: Query<SelectableComponents, With<IsEntitySelected>>,
    buy_orders: Query<&BuyOrders>,
    sell_orders: Query<&SellOrders>,
    construction_sites: Query<&ConstructionSite>,
    names: Query<&Name>,
) -> BevyResult {
    let counts = selected.iter().fold(
        SelectableCount::new(&game_data.asteroids, &gui_data),
        |acc, x| acc.add(x.selectable),
    );

    if counts.total() == 0 {
        return Ok(());
    }

    if counts.total() == 1 {
        let now = simulation_time.now();
        egui::Window::new("Selection Details")
            .anchor(Align2::LEFT_CENTER, egui::Vec2::ZERO)
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .show(context.ctx_mut(), |ui| {
                let item = selected.single().unwrap();
                draw_summary_row(&images, ui, &item);

                if let Some(in_sector) = item.in_sector {
                    ui.label(format!(
                        "In sector {}",
                        names.get(in_sector.sector.into()).unwrap()
                    ));
                }

                if let Some(interaction_queue) = item.interaction_queue {
                    ui.label(format!(
                        "Interaction Queue at {}/{}",
                        interaction_queue.currently_interacting(),
                        interaction_queue.maximum_interactions()
                    ));
                }

                if let Some(inventory) = item.inventory {
                    ui.heading("Inventory");
                    let inventory = inventory.inventory();
                    if inventory.is_empty() {
                        ui.label("Empty");
                    } else {
                        for (item_id, amount) in inventory {
                            let item = game_data.items.get_by_ref(item_id).unwrap();
                            ui.label(format!(
                                "{} x {} (+{}, -{}) [{} max]",
                                item.name,
                                amount.current,
                                amount.planned_incoming,
                                amount.planned_selling,
                                amount.reserved()
                            ));
                        }
                    }
                }

                if let Some(asteroid) = item.asteroid {
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

                if let Some(production) = item.production {
                    ui.heading("Production");
                    for (id, module) in &production.modules {
                        let definition = game_data.production_modules.get_by_ref(id).unwrap();
                        ui.label(format!("  {}x {}", module.amount, definition.name));
                        for running in &module.running_recipes {
                            let recipe =
                                game_data.item_recipes.get_by_ref(&running.recipe).unwrap();
                            ui.label(format!(
                                "    -> {} in {}",
                                recipe.name,
                                now.remaining_time(running.finished_at).as_secs() + 1
                            ));
                        }
                        for queued in &module.queued_recipes {
                            let recipe = game_data.item_recipes.get_by_ref(&queued.recipe).unwrap();
                            ui.label(format!(
                                "    {} (Queued{})",
                                recipe.name,
                                if queued.is_repeating { " [R]" } else { "" }
                            ));
                        }
                    }
                }

                if let Some(buy_orders) = item.buy_orders {
                    list_buy_orders(&game_data, ui, buy_orders);
                }
                if let Some(sell_orders) = item.sell_orders {
                    list_sell_orders(&game_data, ui, sell_orders);
                }

                if let Some(shipyard) = item.shipyard {
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

                if let Some(ship) = item.ship {
                    draw_ship_config_stats(
                        ui,
                        session_data
                            .ship_configurations
                            .get_by_id(&ship.config_id())
                            .unwrap(),
                    )
                }

                if let Some(station) = item.station {
                    if let Some(construction_site_entity) = station.construction_site {
                        let construction_site = construction_sites
                            .get(construction_site_entity.into())
                            .unwrap();

                        let module = construction_site.build_order.first().unwrap();
                        let required_build_power = match module {
                            ConstructableModuleId::ProductionModule(id) => {
                                game_data
                                    .production_modules
                                    .get_by_ref(id)
                                    .unwrap()
                                    .get_constructable_data()
                                    .required_build_power
                            }
                            ConstructableModuleId::ShipyardModule(id) => {
                                game_data
                                    .shipyard_modules
                                    .get_by_ref(id)
                                    .unwrap()
                                    .get_constructable_data()
                                    .required_build_power
                            }
                        };

                        let current_build = construction_site.current_build_progress;

                        ui.horizontal(|ui| {
                            ui.image(images.construct);
                            egui::CollapsingHeader::new(format!(
                                "Construction Site ({current_build:.0} / {required_build_power})"
                            ))
                            .default_open(true)
                            .id_salt("construction_site")
                            .show(ui, |ui| {
                                match &construction_site.status {
                                    ConstructionSiteStatus::Ok => {}
                                    ConstructionSiteStatus::MissingMaterials(missing_materials) => {
                                        ui.label("Missing ingredients:");
                                        for x in missing_materials {
                                            ui.label(format!(
                                                "- {}",
                                                game_data.items.get_by_ref(x).unwrap().name
                                            ));
                                        }
                                    }
                                    ConstructionSiteStatus::MissingBuilders => {
                                        ui.label("No builders!");
                                    }
                                }

                                if let Ok(buy_orders) =
                                    buy_orders.get(construction_site_entity.into())
                                {
                                    list_buy_orders(&game_data, ui, buy_orders);
                                }
                                if let Ok(sell_orders) =
                                    sell_orders.get(construction_site_entity.into())
                                {
                                    list_sell_orders(&game_data, ui, sell_orders);
                                }
                            });
                        });
                    }
                }

                if let Some(task_queue) = item.task_queue {
                    ui.heading("Tasks");

                    match &task_queue.active_task {
                        None => {
                            ui.image(images.idle);
                            ui.label("Idle");
                        }
                        Some(task) => {
                            print_task_list_element(&game_data, &images, names, ui, task);
                            for task in &task_queue.queue {
                                print_task_list_element(&game_data, &images, names, ui, task);
                            }
                        }
                    }
                }
            });

        return Ok(());
    }

    egui::Window::new("Selection Details")
        .anchor(Align2::LEFT_CENTER, egui::Vec2::ZERO)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .vscroll(true)
        .show(context.ctx_mut(), |ui| {
            for item in selected.iter() {
                draw_summary_row(&images, ui, &item);
            }
        });

    Ok(())
}

fn print_task_list_element(
    game_data: &GameData,
    images: &Res<UiIcons>,
    names: Query<&Name>,
    ui: &mut Ui,
    task: &TaskKind,
) {
    ui.horizontal(|ui| {
        ui.image(images.get_task_icon(task));
        ui.label(match task {
            TaskKind::UseGate { data } => {
                format!(
                    "Using gate to {}",
                    names.get(data.exit_sector.into()).unwrap()
                )
            }
            TaskKind::MoveToEntity { data } => {
                format!("Move to {}", names.get(data.target.into()).unwrap())
            }
            TaskKind::DockAtEntity { data } => {
                format!("Dock at {}", names.get(data.target.into()).unwrap())
            }
            TaskKind::Undock { .. } => "Undock".to_string(),
            TaskKind::ExchangeWares { data } => match data.exchange_data {
                ExchangeWareData::Buy(item_id, amount) => {
                    format!(
                        "Buy {amount}x{}",
                        game_data.items.get_by_ref(&item_id).unwrap().name
                    )
                }
                ExchangeWareData::Sell(item_id, amount) => {
                    format!(
                        "Sell {amount}x{}",
                        game_data.items.get_by_ref(&item_id).unwrap().name
                    )
                }
            },
            TaskKind::MineAsteroid { data } => {
                format!("Mining {}", names.get(data.target.into()).unwrap())
            }
            TaskKind::HarvestGas { data } => {
                format!(
                    "Harvesting {} from {}",
                    game_data.items.get_by_ref(&data.gas).unwrap().name,
                    names.get(data.target.into()).unwrap()
                )
            }
            TaskKind::AwaitingSignal { data } => {
                format!(
                    "Awaiting Signal from {}",
                    names.get(data.from.into()).unwrap()
                )
            }
            TaskKind::RequestAccess { data } => {
                format!(
                    "Requesting Access to {}",
                    names.get(data.target.into()).unwrap()
                )
            }
            TaskKind::Construct { data } => {
                // Might be none during the frame where a construction site is finished
                if let Ok(name) = names.get(data.target.into()) {
                    format!("Constructing {}", name)
                } else {
                    "Finished Construction".into()
                }
            }
        });
    });
}

fn list_sell_orders(game_data: &GameData, ui: &mut Ui, sell_orders: &SellOrders) {
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

fn list_buy_orders(game_data: &GameData, ui: &mut Ui, buy_orders: &BuyOrders) {
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

fn draw_summary_row(images: &UiIcons, ui: &mut Ui, item: &SelectableComponentsItem) {
    ui.horizontal(|ui| {
        ui.image(images.get_selectable(item.selectable));
        ui.label(format!("{} ({})", item.name, item.entity));

        if let Some(task_queue) = item.task_queue {
            if let Some(task) = &task_queue.active_task {
                match task {
                    TaskKind::MoveToEntity { .. } => {
                        ui.image(images.get_task_icon(task));
                        if let Some(next_task) = task_queue.queue.front() {
                            ui.image(images.get_task_icon(next_task));
                        }
                    }
                    _ => {
                        ui.image(images.get_task_icon(task));
                    }
                }
            }
        }

        if let Some(inventory) = item.inventory {
            ui.label(format!("{:.0}%", inventory.ratio() * 100.0));
        }

        if let Some(velocity) = item.ship_velocity {
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
