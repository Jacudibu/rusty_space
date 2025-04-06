use crate::components::{
    ConstantOrbit, GateComponent, PlanetComponent, SectorComponent, SectorPlanetsComponent,
    SectorStarComponent, StarComponent, StationComponent,
};
use crate::entity_selection::MouseCursor;
use crate::game_data::{
    ConstructableModuleId, ItemManifest, RecipeManifest, SILICA_PRODUCTION_MODULE_ID,
};
use crate::map_layout::MapLayout;
use crate::persistence::{ConstructionSiteIdMap, StationIdMap};
use crate::utils::entity_spawners::{ConstructionSiteSpawnData, StationSpawnData, spawn_station};
use crate::utils::intersections;
use crate::utils::polar_coordinates::PolarCoordinates;
use crate::{SpriteHandles, constants};
use bevy::app::{App, Plugin};
use bevy::input::ButtonInput;
use bevy::log::warn;
use bevy::prelude::{
    AppExtStates, AppGizmoBuilder, Commands, Component, Entity, GizmoConfigGroup, Gizmos,
    IntoSystemConfigs, Isometry2d, KeyCode, MouseButton, Name, NextState, OnEnter, OnExit, Query,
    Reflect, Res, ResMut, Resource, State, States, Transform, Update, Vec2, Visibility, With,
    Without, in_state,
};
use bevy::sprite::Sprite;

/// Plugin for placing new Construction Sites.
pub struct ConstructionSitePlacementPlugin;
impl Plugin for ConstructionSitePlacementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ConstructionMode::Off);
        app.init_gizmo_group::<ConstructionSitePreviewGizmos>();
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
                    create_construction_site_on_mouse_click.after(update_target_position),
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

/// Marker component for the construction preview entity
#[derive(Component)]
struct PreviewEntityComponent {}

/// [GizmoConfigGroup] for all gizmos related to Construction Site Previews.
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct ConstructionSitePreviewGizmos;

fn spawn_construction_preview_entity(mut commands: Commands, sprites: Res<SpriteHandles>) {
    commands.spawn((
        Name::new("Construction Site Preview"),
        PreviewEntityComponent {},
        Sprite {
            image: sprites.station.clone(),
            color: constants::INVALID_PREVIEW_COLOR,
            ..Default::default()
        },
    ));

    commands.insert_resource(PreviewTargetPosition::default());
}

/// Despawns all entities marked with a [PreviewEntityComponent]
fn despawn_construction_preview_entity(
    mut commands: Commands,
    query: Query<Entity, With<PreviewEntityComponent>>,
) {
    for x in query.iter() {
        commands.entity(x).despawn();
    }

    commands.remove_resource::<PreviewTargetPosition>()
}

#[derive(Resource)]
struct PreviewTargetPosition {
    pub position_state: Result<(), PositionValidationError>,
}

impl Default for PreviewTargetPosition {
    fn default() -> Self {
        PreviewTargetPosition {
            position_state: Err(PositionValidationError::InvalidPosition),
        }
    }
}

/// Updates the color and position of all entities marked with a [PreviewEntityComponent].
#[allow(clippy::type_complexity)]
fn update_preview_entity(
    mouse_cursor: Res<MouseCursor>,
    preview_target: Res<PreviewTargetPosition>,
    mut preview_query: Query<
        (&mut Transform, &mut Sprite, &mut Visibility),
        (
            With<PreviewEntityComponent>,
            Without<StationComponent>,
            Without<GateComponent>,
            Without<PlanetComponent>,
            Without<StarComponent>,
        ),
    >,
    mut gizmos: Gizmos<ConstructionSitePreviewGizmos>,
) {
    for (mut transform, mut sprite, mut visibility) in preview_query.iter_mut() {
        let color = match &preview_target.position_state {
            Ok(_) => {
                *visibility = Visibility::Visible;
                constants::VALID_PREVIEW_COLOR
            }
            Err(e) => match e {
                PositionValidationError::InvalidPosition => {
                    *visibility = Visibility::Hidden;
                    continue;
                }
                PositionValidationError::NotWithinSector => {
                    *visibility = Visibility::Visible;
                    constants::INVALID_PREVIEW_COLOR
                }
                PositionValidationError::TooCloseToSectorEdge => {
                    *visibility = Visibility::Visible;
                    constants::INVALID_PREVIEW_COLOR
                }
                PositionValidationError::TooCloseTo(conflicts) => {
                    *visibility = Visibility::Visible;

                    for pos in conflicts {
                        gizmos.circle_2d(
                            Isometry2d::from_translation(*pos),
                            constants::STATION_GATE_PLANET_RADIUS,
                            constants::INVALID_PREVIEW_COLOR,
                        );
                    }

                    constants::INVALID_PREVIEW_COLOR
                }
            },
        };

        sprite.color = color;

        let center = mouse_cursor.world_space.unwrap();
        transform.translation = center.extend(constants::z_layers::TRANSPARENT_PREVIEW_ITEM);
        gizmos.circle_2d(
            Isometry2d::from_translation(center),
            constants::MINIMUM_DISTANCE_BETWEEN_STATIONS,
            color,
        );
    }
}

/// Creates a new Construction Site when the player clicks the left mouse button.
#[allow(clippy::too_many_arguments)]
fn create_construction_site_on_mouse_click(
    mut commands: Commands,
    mut sector_query: Query<(&mut SectorComponent, Option<&SectorStarComponent>)>,
    mut station_id_map: ResMut<StationIdMap>,
    mut construction_site_id_map: ResMut<ConstructionSiteIdMap>,
    stars: Query<&StarComponent>,
    sprites: Res<SpriteHandles>,
    item_manifest: Res<ItemManifest>,
    recipe_manifest: Res<RecipeManifest>,
    mouse_cursor: Res<MouseCursor>,
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
    let Some(sector_pos) = &mouse_cursor.sector_space else {
        return;
    };

    // TODO: Test whether position is valid: Too close to other stations or sector edge
    // TODO: Snap position to nearby orbits to avoid collisions / overlaps

    let construction_site =
        ConstructionSiteSpawnData::new(vec![ConstructableModuleId::ProductionModule(
            SILICA_PRODUCTION_MODULE_ID,
        )]);
    let data = StationSpawnData::new(
        "Fancy Station",
        construction_site,
        sector_pos.sector_position,
    );

    spawn_station(
        &mut commands,
        &mut sector_query,
        &stars,
        &mut station_id_map,
        &mut construction_site_id_map,
        &sprites,
        &item_manifest,
        &recipe_manifest,
        data,
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

/// Updates the [PreviewTargetPosition] resource before any of the systems depending on it are run.
#[allow(clippy::too_many_arguments)]
fn update_target_position(
    mut preview_target: ResMut<PreviewTargetPosition>,
    mouse_cursor: Res<MouseCursor>,
    all_sectors: Query<(
        &SectorComponent,
        Option<&SectorPlanetsComponent>,
        Option<&SectorStarComponent>,
    )>,
    all_stations: Query<&Transform, With<StationComponent>>,
    all_gates: Query<&Transform, With<GateComponent>>,
    all_planets: Query<(&Transform, Option<&ConstantOrbit>), With<PlanetComponent>>,
    all_stars: Query<&Transform, With<StarComponent>>,
    map_layout: Res<MapLayout>,
) {
    preview_target.position_state = is_construction_site_position_valid(
        &mouse_cursor,
        &all_sectors,
        &all_stations,
        &all_gates,
        &all_planets,
        &all_stars,
        &map_layout,
    );
}

fn is_construction_site_position_valid(
    position: &MouseCursor,
    all_sectors: &Query<(
        &SectorComponent,
        Option<&SectorPlanetsComponent>,
        Option<&SectorStarComponent>,
    )>,
    all_stations: &Query<&Transform, With<StationComponent>>,
    all_gates: &Query<&Transform, With<GateComponent>>,
    all_planets: &Query<(&Transform, Option<&ConstantOrbit>), With<PlanetComponent>>,
    all_stars: &Query<&Transform, With<StarComponent>>,
    map_layout: &MapLayout,
) -> Result<(), PositionValidationError> {
    let Some(world_pos) = position.world_space else {
        return Err(PositionValidationError::InvalidPosition);
    };

    let Some(sector_pos) = &position.sector_space else {
        return Err(PositionValidationError::NotWithinSector);
    };

    let (sector, sector_planets, sector_star) = all_sectors
        .get(sector_pos.sector_position.sector.into())
        .expect("Sector Position within mouse sector pos should always be valid!");

    // Sector Edges
    for edge in map_layout.hex_edge_vertices {
        if intersections::intersect_line_with_circle(
            edge[0] + sector.world_pos,
            edge[1] + sector.world_pos,
            world_pos,
            constants::MINIMUM_DISTANCE_BETWEEN_STATIONS,
        ) {
            return Err(PositionValidationError::TooCloseToSectorEdge);
        }
    }

    // Other Stations
    let mut conflicts = Vec::new();
    for entity in &sector.stations {
        let Ok(station) = all_stations.get(entity.into()) else {
            warn!("Station in sector with ID {:?} did not exist!", entity);
            continue;
        };

        if is_item_too_close(station, world_pos) {
            conflicts.push(station.translation.truncate());
        }
    }

    // Gates
    for (_, gate_pair) in &sector.gates {
        let Ok(gate) = all_gates.get(gate_pair.from.into()) else {
            warn!("Gate in sector with ID {:?} did not exist!", gate_pair.from);
            continue;
        };

        if is_item_too_close(gate, world_pos) {
            conflicts.push(gate.translation.truncate());
        }
    }

    // Planets
    if let Some(planets) = sector_planets {
        for planet in &planets.planets {
            let Ok((planet_pos, _)) = all_planets.get(planet.into()) else {
                warn!("Planet in sector with ID {:?} did not exist!", planet);
                continue;
            };

            if is_item_too_close(planet_pos, world_pos) {
                conflicts.push(planet_pos.translation.truncate());
            }
        }
    }

    // Celestials
    if let Some(star) = sector_star {
        if let Ok(star) = all_stars.get(star.entity.into()) {
            if is_item_too_close(star, world_pos) {
                conflicts.push(star.translation.truncate());
            }
        } else {
            warn!("Star in sector with ID {:?} did not exist!", star.entity);
        };

        // TODO: Snapping logic. Pull it out of this method, and add the snapped position as function argument
        if let Some(sector_planets) = sector_planets {
            for planet_entity in &sector_planets.planets {
                let (_, orbit) = all_planets.get(planet_entity.into()).unwrap();
                let orbit = orbit.unwrap();

                let orbit_distance = orbit.polar_coordinates.radial_distance;
                let min = orbit_distance - constants::MINIMUM_DISTANCE_BETWEEN_STATIONS;
                let max = orbit_distance - constants::MINIMUM_DISTANCE_BETWEEN_STATIONS;

                // TODO: only preview render min if its > 0
                let local_pos = sector_pos.sector_position.local_position;
                let mut desired_polar_pos = PolarCoordinates::from_cartesian(&local_pos);

                let snap = desired_polar_pos.radial_distance > min
                    && desired_polar_pos.radial_distance < max;
                if snap {
                    desired_polar_pos.radial_distance = orbit_distance;
                    // TODO: Return this and add the radial distance to preview
                    let target_pos = desired_polar_pos.to_cartesian();
                } else {
                    continue;
                }
            }
        }

        // TODO: Check for Orbit conflicts with all gates, stations and planets. This will be a bit of a headache.
        //       Bonus points if we draw funny orbit gizmos and snap the position onto existing orbits when nearby.
    }

    if conflicts.is_empty() {
        Ok(())
    } else {
        Err(PositionValidationError::TooCloseTo(conflicts))
    }
}

fn is_item_too_close(construction_pos: &Transform, pos: Vec2) -> bool {
    pos.distance(construction_pos.translation.truncate())
        < constants::MINIMUM_DISTANCE_BETWEEN_STATIONS + constants::STATION_GATE_PLANET_RADIUS
}
