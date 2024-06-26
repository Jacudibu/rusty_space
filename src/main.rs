use crate::data::GameData;
use crate::entity_selection::MouseInteractionGizmos;
use crate::mouse_cursor::MouseCursor;
use crate::production_manager::GlobalProductionState;
use crate::simulation_time::SimulationTime;
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::Vec3;
use bevy::prelude::{
    default, App, AppExtStates, AppGizmoBuilder, Camera2dBundle, Commands, First, Handle, Image,
    ImagePlugin, IntoSystemConfigs, PluginGroup, PreUpdate, Quat, Res, Resource, Startup,
    Transform, Update, Vec2, Window, WindowPlugin,
};
use bevy::render::camera::ScalingMode;
use bevy::sprite::SpriteBundle;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use components::*;
use data::*;

mod camera;
mod components;
mod data;
mod entity_selection;
mod gui;
mod mouse_cursor;
mod physics;
mod production_manager;
mod ship_ai;
mod simulation_time;
mod utils;

const SHIP_COUNT: i32 = 10;
pub const MOCK_INVENTORY_SIZE: u32 = 5000;

fn get_window_title() -> String {
    let config = if cfg!(debug_assertions) {
        "DEBUG"
    } else {
        "RELEASE"
    };

    format!("{SHIP_COUNT} ships [{config}]")
}

const SHIP_LAYER: f32 = 10.0;
const STATION_LAYER: f32 = 5.0;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: get_window_title(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(EguiPlugin)
    .insert_resource(GameData::mock_data())
    .insert_resource(MouseCursor::default())
    .insert_resource(SimulationTime::default())
    .insert_resource(GlobalProductionState::default())
    .init_gizmo_group::<MouseInteractionGizmos>()
    .init_state::<gui::MouseCursorOverUiState>()
    .add_event::<ship_ai::TaskFinishedEvent>()
    .add_event::<production_manager::ProductionStartedEvent>()
    .add_event::<production_manager::TestIfEntityCanStartProductionEvent>()
    .add_systems(Startup, (on_startup, gui::initialize.after(on_startup)))
    .add_systems(First, simulation_time::update.after(bevy::time::TimeSystem))
    .add_systems(
        PreUpdate,
        (
            entity_selection::update_cursor_position,
            gui::detect_mouse_cursor_over_ui,
        ),
    )
    .add_systems(
        Update,
        (
            gui::list_selection_icons_and_counts,
            gui::list_selection_details,
            camera::move_camera,
            camera::zoom_camera,
            entity_selection::process_mouse_clicks,
            entity_selection::update_mouse_interaction,
            entity_selection::draw_mouse_interactions,
            entity_selection::on_selection_changed
                .after(entity_selection::process_mouse_clicks)
                .after(entity_selection::update_mouse_interaction),
            production_manager::update,
            production_manager::on_production_started,
            production_manager::check_if_production_can_start_on_inventory_updates,
            ship_ai::handle_idle_ships,
            ship_ai::run_ship_tasks,
            ship_ai::complete_tasks.after(ship_ai::run_ship_tasks),
            physics::move_things.after(ship_ai::run_ship_tasks),
        ),
    );

    if SHIP_COUNT > 10000 {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(LogDiagnosticsPlugin::default());
    }

    app.run();
}

#[derive(Resource)]
pub struct SpriteHandles {
    station: Handle<Image>,
    station_selected: Handle<Image>,
    ship: Handle<Image>,
    ship_selected: Handle<Image>,
}

fn spawn_station(
    commands: &mut Commands,
    sprites: &SpriteHandles,
    name: &str,
    pos: Vec2,
    buys: &ItemDefinition,
    sells: &ItemDefinition,
    produces: RecipeId,
) {
    commands.spawn((
        Name::new(name.to_string()),
        SelectableEntity::Station,
        SpriteBundle {
            texture: sprites.station.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, STATION_LAYER),
            ..default()
        },
        Inventory::new_with_content(MOCK_INVENTORY_SIZE, vec![(sells.id, MOCK_INVENTORY_SIZE)]),
        BuyOrders::mock_buying_item(buys),
        SellOrders::mock_selling_item(sells),
        ProductionModule {
            recipe: Some(produces),
            current_run_finished_at: None,
        },
    ));
}

pub fn on_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_data: Res<GameData>,
) {
    let sprites = SpriteHandles {
        station: asset_server.load("station.png"),
        station_selected: asset_server.load("station_selected.png"),
        ship: asset_server.load("ship.png"),
        ship_selected: asset_server.load("ship_selected.png"),
    };

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(1.0);
    commands.spawn((Name::new("Camera"), camera::MainCamera, camera_bundle));

    spawn_station(
        &mut commands,
        &sprites,
        "Station A",
        Vec2::new(-200.0, -200.0),
        &game_data.items[&DEBUG_ITEM_ID_A],
        &game_data.items[&DEBUG_ITEM_ID_B],
        RECIPE_B_ID,
    );
    spawn_station(
        &mut commands,
        &sprites,
        "Station B",
        Vec2::new(200.0, -200.0),
        &game_data.items[&DEBUG_ITEM_ID_B],
        &game_data.items[&DEBUG_ITEM_ID_C],
        RECIPE_C_ID,
    );
    spawn_station(
        &mut commands,
        &sprites,
        "Station C",
        Vec2::new(0.0, 200.0),
        &game_data.items[&DEBUG_ITEM_ID_C],
        &game_data.items[&DEBUG_ITEM_ID_A],
        RECIPE_A_ID,
    );

    for i in 0..SHIP_COUNT {
        commands.spawn((
            Name::new(format!("Ship {i}")),
            SelectableEntity::Ship,
            ShipBehavior::AutoTrade(AutoTradeData {}),
            Engine { ..default() },
            Velocity {
                forward: (i % 100) as f32,
                angular: 0.0,
            },
            Inventory::new(100),
            SpriteBundle {
                texture: sprites.ship.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(
                        ((std::f32::consts::PI * 2.0) / SHIP_COUNT as f32) * i as f32,
                    ),
                    translation: Vec3::new(0.0, 0.0, SHIP_LAYER),
                    ..default()
                },
                ..default()
            },
        ));
    }

    commands.insert_resource(sprites);
}
