use crate::entity_selection::MouseCursor;
use bevy::app::{App, Plugin};
use bevy::ecs::query::QueryFilter;
use bevy::input::ButtonInput;
use bevy::log::warn;
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    AppExtStates, AppGizmoBuilder, BevyError, Commands, Component, Entity, GizmoConfig,
    GizmoConfigGroup, GizmoLineConfig, GizmoLineStyle, Gizmos, IntoScheduleConfigs, Isometry2d,
    KeyCode, MouseButton, Name, NextState, OnEnter, OnExit, Or, Query, Reflect, Res, ResMut,
    Resource, State, States, Transform, Update, Vec2, Visibility, With, Without, in_state,
};
use bevy::sprite::Sprite;
use common::components::celestials::{Celestial, Planet, Star};
use common::components::{
    BuyOrderData, BuyOrders, ConstantOrbit, Gate, Sector, SectorWithCelestials, Station,
};
use common::game_data::{
    Constructable, ConstructableModuleId, ItemId, ItemManifest, ProductionModuleManifest,
    RecipeManifest, SILICA_PRODUCTION_MODULE_ID, ShipyardModuleManifest,
};
use common::types::entity_id_map::{ConstructionSiteIdMap, StationIdMap};
use common::types::map_layout::MapLayout;
use common::types::polar_coordinates::PolarCoordinates;
use common::types::price_setting::PriceSetting;
use common::types::sector_position::SectorPosition;
use common::types::sprite_handles::SpriteHandles;
use common::{constants, geometry};
use entity_spawners::spawn_station::{ConstructionSiteSpawnData, StationSpawnData, spawn_station};

/// Plugin for placing new Construction Sites.
pub struct ConstructionSitePlacementPlugin;
impl Plugin for ConstructionSitePlacementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ConstructionMode::Off);
        app.init_gizmo_group::<ConstructionSitePreviewGizmos>();
        app.insert_gizmo_config(
            DottedConstructionSitePreviewGizmos,
            GizmoConfig {
                line: GizmoLineConfig {
                    style: GizmoLineStyle::Dotted,
                    width: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        app.add_systems(
            OnEnter(ConstructionMode::On),
            spawn_construction_preview_entity,
        );
        app.add_systems(
            OnExit(ConstructionMode::On),
            despawn_construction_preview_entity,
        );
        app.add_systems(
            Update,
            (
                toggle_construction_mode,
                (
                    update_target_position,
                    update_preview_entity.after(update_target_position),
                    spawn_construction_site_on_mouse_click.after(update_target_position),
                )
                    .run_if(in_state(ConstructionMode::On)),
            ),
        );
    }
}

/// States indicating whether we are currently in Construction Mode.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstructionMode {
    Off,
    On,
}

/// Toggles the [ConstructionMode] between [ConstructionMode::On] and [ConstructionMode::Off] when pressing a button.
fn toggle_construction_mode(
    state: Res<State<ConstructionMode>>,
    mut next_state: ResMut<NextState<ConstructionMode>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::KeyV) {
        return;
    }

    match state.get() {
        ConstructionMode::Off => {
            next_state.set(ConstructionMode::On);
        }
        ConstructionMode::On => {
            next_state.set(ConstructionMode::Off);
        }
    }
}

/// Marker component for the construction site preview entity.
#[derive(Component)]
#[component(immutable)]
struct ConstructionSitePreviewEntity {}

/// [GizmoConfigGroup] for all gizmos related to Construction Site Previews.
#[derive(Default, Reflect, GizmoConfigGroup)]
struct ConstructionSitePreviewGizmos;

/// [GizmoConfigGroup] for all gizmos related to Construction Site Previews with dotted lines.
#[derive(Default, Reflect, GizmoConfigGroup)]
struct DottedConstructionSitePreviewGizmos;

fn spawn_construction_preview_entity(mut commands: Commands, sprites: Res<SpriteHandles>) {
    commands.spawn((
        Name::new("Construction Site Preview"),
        ConstructionSitePreviewEntity {},
        Sprite {
            image: sprites.station.clone(),
            color: constants::colors::INVALID_PREVIEW_COLOR,
            ..Default::default()
        },
    ));

    commands.insert_resource(PreviewTargetPosition::default());
}

/// Despawns all entities marked with a [ConstructionSitePreviewEntity]
fn despawn_construction_preview_entity(
    mut commands: Commands,
    query: Query<Entity, With<ConstructionSitePreviewEntity>>,
) {
    for x in query.iter() {
        commands.entity(x).despawn();
    }

    commands.remove_resource::<PreviewTargetPosition>()
}

enum PreviewGizmos {
    PolarRadius { radius: f32 },
}

#[derive(Resource)]
struct PreviewTargetPosition {
    pub world_pos: Option<Vec2>,
    pub sector_pos: Option<SectorPosition>,
    pub position_state: Result<(), PositionValidationError>,
    pub gizmos: Vec<PreviewGizmos>,
}

impl Default for PreviewTargetPosition {
    fn default() -> Self {
        PreviewTargetPosition {
            world_pos: None,
            sector_pos: None,
            position_state: Err(PositionValidationError::InvalidPosition),
            gizmos: Vec::default(),
        }
    }
}

/// Updates the color and position of all entities marked with a [ConstructionSitePreviewEntity].
#[allow(clippy::type_complexity)]
fn update_preview_entity(
    preview_target: Res<PreviewTargetPosition>,
    mut preview_query: Query<
        (&mut Transform, &mut Sprite, &mut Visibility),
        (
            With<ConstructionSitePreviewEntity>,
            Without<Station>,
            Without<Gate>,
            Without<Planet>,
            Without<Star>,
        ),
    >,
    mut gizmos: Gizmos<ConstructionSitePreviewGizmos>,
    mut gizmos_dotted: Gizmos<DottedConstructionSitePreviewGizmos>,
) -> Result<(), BevyError> {
    let (mut preview_transform, mut preview_sprite, mut preview_visibility) =
        preview_query.single_mut()?;

    match &preview_target.position_state {
        Ok(_) => {}
        Err(e) => match e {
            PositionValidationError::InvalidPosition => {
                *preview_visibility = Visibility::Hidden;
                return Ok(());
            }
            PositionValidationError::NotWithinSector => {}
            PositionValidationError::TooCloseToSectorEdge => {}
            PositionValidationError::TooCloseTo(conflicts) => {
                for pos in conflicts {
                    gizmos.circle_2d(
                        Isometry2d::from_translation(*pos),
                        constants::STATION_GATE_PLANET_RADIUS,
                        constants::colors::INVALID_PREVIEW_COLOR,
                    );
                }
            }
        },
    };

    *preview_visibility = Visibility::Visible;

    let color = if preview_target.position_state.is_ok() {
        constants::colors::VALID_PREVIEW_COLOR
    } else {
        constants::colors::INVALID_PREVIEW_COLOR
    };

    preview_sprite.color = color;

    let world_pos = preview_target.world_pos.unwrap();
    preview_transform.translation = world_pos.extend(constants::z_layers::TRANSPARENT_PREVIEW_ITEM);
    gizmos.circle_2d(
        Isometry2d::from_translation(world_pos),
        constants::MINIMUM_DISTANCE_BETWEEN_STATIONS,
        color,
    );

    let Some(sector_pos) = preview_target.sector_pos else {
        return Ok(());
    };

    let sector_center = world_pos - sector_pos.local_position;
    for gizmo in preview_target.gizmos.iter() {
        match gizmo {
            PreviewGizmos::PolarRadius { radius } => {
                gizmos.circle_2d(
                    Isometry2d::from_translation(sector_center),
                    *radius,
                    constants::colors::ORBIT_PREVIEW_COLOR,
                );

                gizmos_dotted.circle_2d(
                    Isometry2d::from_translation(sector_center),
                    radius + constants::MINIMUM_DISTANCE_BETWEEN_STATIONS,
                    constants::colors::ORBIT_PREVIEW_COLOR,
                );
                gizmos_dotted.circle_2d(
                    Isometry2d::from_translation(sector_center),
                    radius - constants::MINIMUM_DISTANCE_BETWEEN_STATIONS,
                    constants::colors::ORBIT_PREVIEW_COLOR,
                );
            }
        }
    }

    Ok(())
}

/// Creates a new Construction Site when the player clicks the left mouse button.
#[allow(clippy::too_many_arguments)]
fn spawn_construction_site_on_mouse_click(
    mut commands: Commands,
    mut sector_query: Query<(&mut Sector, Option<&SectorWithCelestials>)>,
    mut station_id_map: ResMut<StationIdMap>,
    mut construction_site_id_map: ResMut<ConstructionSiteIdMap>,
    sprites: Res<SpriteHandles>,
    production_module_manifest: Res<ProductionModuleManifest>,
    shipyard_module_manifest: Res<ShipyardModuleManifest>,
    item_manifest: Res<ItemManifest>,
    recipe_manifest: Res<RecipeManifest>,
    mouse: Res<ButtonInput<MouseButton>>,
    preview_target: Res<PreviewTargetPosition>,
) {
    if preview_target.position_state.is_err() {
        return;
    }

    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    // TODO: This should fire an event, which is then either processed locally or sent to the server
    let Some(sector_pos) = &preview_target.sector_pos else {
        return;
    };

    // TODO: Planned modules and materials should already exist at this point - they should be set up in some kind of dialogue.
    let planned_modules = vec![ConstructableModuleId::ProductionModule(
        SILICA_PRODUCTION_MODULE_ID,
    )];

    let mut total_materials = HashMap::<ItemId, u32>::new();
    for x in &planned_modules {
        let materials = match x {
            ConstructableModuleId::ProductionModule(id) => {
                &production_module_manifest
                    .get_by_ref(id)
                    .unwrap()
                    .get_constructable_data()
                    .required_materials
            }
            ConstructableModuleId::ShipyardModule(id) => {
                &shipyard_module_manifest
                    .get_by_ref(id)
                    .unwrap()
                    .get_constructable_data()
                    .required_materials
            }
        };

        for x in materials {
            if let Some(existing_amount) = total_materials.get_mut(&x.item_id) {
                *existing_amount += x.amount;
            } else {
                total_materials.insert(x.item_id, x.amount);
            }
        }
    }

    let buy_orders = BuyOrders {
        orders: total_materials
            .into_iter()
            .map(|(id, amount)| {
                let price = item_manifest.get_by_ref(&id).unwrap().price.max;
                (
                    id,
                    BuyOrderData {
                        amount,
                        price,
                        buy_up_to: amount,
                        price_setting: PriceSetting::Fixed(price),
                    },
                )
            })
            .collect(),
    };

    let construction_spawn_data = ConstructionSiteSpawnData::new(planned_modules, buy_orders);
    let station_data = StationSpawnData::new("Fancy Station", construction_spawn_data, *sector_pos);

    spawn_station(
        &mut commands,
        &mut sector_query,
        &mut station_id_map,
        &mut construction_site_id_map,
        &sprites,
        &item_manifest,
        &recipe_manifest,
        station_data,
    );
}

enum PositionValidationError {
    /// The world position is invalid, for whatever reason
    InvalidPosition,

    /// The position is not within a sector
    NotWithinSector,

    /// The position is too close to one of the sector edges
    TooCloseToSectorEdge,

    /// The position is too close to one or more existing objects at the given positions.
    TooCloseTo(Vec<Vec2>),
}

/// A [QueryFilter] which filters for all objects big enough to block a construction site.
#[derive(QueryFilter)]
#[allow(clippy::type_complexity)]
struct ConstructionBlockingItemFilter {
    tuple: (Or<(With<Station>, With<Celestial>, With<Gate>)>,),
}

/// Updates the [PreviewTargetPosition] resource before any of the systems depending on it are run.
#[allow(clippy::too_many_arguments)]
fn update_target_position(
    mut preview_target: ResMut<PreviewTargetPosition>,
    mouse_cursor: Res<MouseCursor>,
    all_sectors: Query<(&Sector, Option<&SectorWithCelestials>)>,
    orbiting_objects: Query<&ConstantOrbit>,
    potentially_blocking_transforms: Query<&Transform, ConstructionBlockingItemFilter>,
    map_layout: Res<MapLayout>,
) {
    preview_target.gizmos.clear();
    let Some(world_pos) = mouse_cursor.world_space else {
        preview_target.position_state = Err(PositionValidationError::InvalidPosition);
        return;
    };

    let Some(sector_pos) = &mouse_cursor.sector_space else {
        preview_target.world_pos = Some(world_pos);
        preview_target.sector_pos = None;
        preview_target.position_state = Err(PositionValidationError::NotWithinSector);
        return;
    };

    let (sector, sector_with_celestials) = all_sectors
        .get(sector_pos.sector_position.sector.into())
        .expect("Sector Position within mouse sector pos should always be valid!");

    let blocking_entities = collect_blocking_sector_entities(sector, sector_with_celestials);

    // TODO: Add a definitive marker component for orbit mechanics rather than just checking the star. That way we could easily orbit other things in the future!
    let has_orbit_mechanics = sector_with_celestials.is_some();
    let local_target_pos = if has_orbit_mechanics {
        let polar = calculate_snapped_polar_coordinates(
            &blocking_entities,
            sector_pos.sector_position.local_position,
            &orbiting_objects,
        );
        preview_target.gizmos.push(PreviewGizmos::PolarRadius {
            radius: polar.radial_distance,
        });
        polar.to_cartesian()
    } else {
        sector_pos.sector_position.local_position
    };

    preview_target.world_pos = Some(local_target_pos + sector.world_pos);
    preview_target.sector_pos = Some(SectorPosition {
        local_position: local_target_pos,
        sector: sector_pos.sector_position.sector,
    });
    preview_target.position_state = is_construction_site_position_valid(
        sector.world_pos,
        local_target_pos,
        local_target_pos + sector.world_pos,
        blocking_entities,
        &map_layout,
        has_orbit_mechanics,
        &potentially_blocking_transforms,
    );
}

fn collect_blocking_sector_entities(
    sector: &Sector,
    sector_celestials: Option<&SectorWithCelestials>,
) -> Vec<Entity> {
    let mut blocking_celestials: Vec<Entity> = sector
        .stations
        .iter()
        .map(Entity::from)
        .chain(sector.gates.iter().map(|x| Entity::from(x.1.from)))
        .collect();

    if let Some(sector_celestials) = sector_celestials {
        blocking_celestials.extend(sector_celestials.planets.iter().map(Entity::from));
        blocking_celestials.extend(sector_celestials.stars.iter().map(Entity::from));
        blocking_celestials.extend(sector_celestials.gas_giants.iter().map(Entity::from));
    }

    blocking_celestials
}

/// Calculates the polar coordinates for our new station within a sector with orbit mechanics
fn calculate_snapped_polar_coordinates(
    sector_celestials: &Vec<Entity>,
    local_pos: Vec2,
    orbiting_objects: &Query<&ConstantOrbit>,
) -> PolarCoordinates {
    let desired_polar_pos = PolarCoordinates::from_cartesian(&local_pos);

    let mut closest_position = None;
    let mut closest_distance = f32::MAX;
    for entity in sector_celestials {
        let Ok(orbit) = orbiting_objects.get(*entity) else {
            // Must be the star
            continue;
        };

        let orbit_distance = orbit.polar_coordinates.radial_distance;
        let min = orbit_distance - constants::MINIMUM_DISTANCE_BETWEEN_STATIONS;
        let max = orbit_distance + constants::MINIMUM_DISTANCE_BETWEEN_STATIONS;

        let snap = desired_polar_pos.radial_distance > min - constants::STATION_GATE_PLANET_RADIUS
            && desired_polar_pos.radial_distance < max + constants::STATION_GATE_PLANET_RADIUS;
        if snap {
            let distance_to_orbit = (orbit_distance - desired_polar_pos.radial_distance).abs();
            if distance_to_orbit < closest_distance {
                closest_distance = distance_to_orbit;
                closest_position = Some(PolarCoordinates {
                    angle: desired_polar_pos.angle,
                    radial_distance: orbit_distance,
                });
            }
        } else {
            continue;
        }
    }

    closest_position.unwrap_or(desired_polar_pos)
}

#[allow(clippy::type_complexity)]
fn is_construction_site_position_valid(
    sector_world_pos: Vec2,
    site_local_pos: Vec2,
    site_world_pos: Vec2,
    blocking_entities: Vec<Entity>,
    map_layout: &MapLayout,
    has_orbit_mechanic: bool,
    potentially_blocking_transforms: &Query<&Transform, ConstructionBlockingItemFilter>,
) -> Result<(), PositionValidationError> {
    // Sector Edges
    if has_orbit_mechanic {
        if site_local_pos.length()
            > constants::SECTOR_INCIRCLE_RADIUS - constants::MINIMUM_DISTANCE_BETWEEN_STATIONS
        {
            return Err(PositionValidationError::TooCloseToSectorEdge);
        }
    } else {
        for edge in map_layout.hex_edge_vertices {
            if geometry::intersect_line_with_circle(
                edge[0] + sector_world_pos,
                edge[1] + sector_world_pos,
                site_world_pos,
                constants::MINIMUM_DISTANCE_BETWEEN_STATIONS,
            ) {
                return Err(PositionValidationError::TooCloseToSectorEdge);
            }
        }
    }

    let mut conflicts = Vec::new();
    for entity in blocking_entities {
        let Ok(transform) = potentially_blocking_transforms.get(entity) else {
            warn!(
                "Object that should block construction in sector with ID {:?} did not exist!",
                entity
            );
            continue;
        };

        if is_item_too_close(transform, site_world_pos) {
            conflicts.push(transform.translation.truncate());
        }
    }

    if conflicts.is_empty() {
        Ok(())
    } else {
        Err(PositionValidationError::TooCloseTo(conflicts))
    }
}

#[inline]
fn is_item_too_close(construction_pos: &Transform, pos: Vec2) -> bool {
    pos.distance(construction_pos.translation.truncate())
        < constants::MINIMUM_DISTANCE_BETWEEN_STATIONS + constants::STATION_GATE_PLANET_RADIUS
}
