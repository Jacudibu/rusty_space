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
use crate::utils::polar_coordinates::PolarCoordinates;
use crate::utils::{SectorPosition, intersections};
use crate::{SpriteHandles, constants};
use bevy::app::{App, Plugin};
use bevy::input::ButtonInput;
use bevy::log::warn;
use bevy::prelude::{
    AppExtStates, AppGizmoBuilder, Commands, Component, Entity, GizmoConfigGroup, Gizmos,
    IntoSystemConfigs, Isometry2d, KeyCode, MouseButton, Name, NextState, OnEnter, OnExit, Or,
    Query, Reflect, Res, ResMut, Resource, State, States, Transform, Update, Vec2, Visibility,
    With, Without, in_state,
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
    pub world_pos: Option<Vec2>,
    pub sector_pos: Option<SectorPosition>,
    pub position_state: Result<(), PositionValidationError>,
}

impl Default for PreviewTargetPosition {
    fn default() -> Self {
        PreviewTargetPosition {
            world_pos: None,
            sector_pos: None,
            position_state: Err(PositionValidationError::InvalidPosition),
        }
    }
}

/// Updates the color and position of all entities marked with a [PreviewEntityComponent].
#[allow(clippy::type_complexity)]
fn update_preview_entity(
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

        let center = preview_target.world_pos.unwrap();
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

    let construction_site =
        ConstructionSiteSpawnData::new(vec![ConstructableModuleId::ProductionModule(
            SILICA_PRODUCTION_MODULE_ID,
        )]);
    let data = StationSpawnData::new("Fancy Station", construction_site, *sector_pos);

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
    orbiting_objects: Query<&ConstantOrbit>,
    potentially_blocking_transforms: Query<
        &Transform,
        Or<(
            With<StationComponent>,
            With<PlanetComponent>,
            With<StarComponent>,
            With<GateComponent>,
        )>,
    >,
    map_layout: Res<MapLayout>,
) {
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

    let (sector, sector_planets, sector_star) = all_sectors
        .get(sector_pos.sector_position.sector.into())
        .expect("Sector Position within mouse sector pos should always be valid!");

    let blocking_entities = collect_blocking_sector_entities(sector, sector_planets, sector_star);

    // TODO: Add a definitive marker component for orbit mechanics rather than just checking the star. That way we could easily orbit other things in the future!
    let local_target_pos = if sector_star.is_some() {
        let polar = calculate_snapped_polar_coordinates(
            &blocking_entities,
            sector_pos.sector_position.local_position,
            &orbiting_objects,
        );
        // TODO: Add orbit gizmo
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
        local_target_pos + sector.world_pos,
        blocking_entities,
        &map_layout,
        &potentially_blocking_transforms,
    );
}

fn collect_blocking_sector_entities(
    sector: &SectorComponent,
    sector_planets: Option<&SectorPlanetsComponent>,
    sector_star: Option<&SectorStarComponent>,
) -> Vec<Entity> {
    let mut sector_celestials: Vec<Entity> = sector
        .stations
        .iter()
        .map(Entity::from)
        .chain(sector.gates.iter().map(|x| Entity::from(x.1.from)))
        .collect();

    if let Some(sector_planets) = sector_planets {
        sector_celestials.extend(sector_planets.planets.iter().map(Entity::from));
    }

    if let Some(sector_star) = sector_star {
        sector_celestials.push(sector_star.entity.into());
    }

    sector_celestials
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
    site_world_pos: Vec2,
    blocking_entities: Vec<Entity>,
    map_layout: &MapLayout,
    potentially_blocking_transforms: &Query<
        &Transform,
        Or<(
            With<StationComponent>,
            With<PlanetComponent>,
            With<StarComponent>,
            With<GateComponent>,
        )>,
    >,
) -> Result<(), PositionValidationError> {
    // Sector Edges
    for edge in map_layout.hex_edge_vertices {
        if intersections::intersect_line_with_circle(
            edge[0] + sector_world_pos,
            edge[1] + sector_world_pos,
            site_world_pos,
            constants::MINIMUM_DISTANCE_BETWEEN_STATIONS,
        ) {
            return Err(PositionValidationError::TooCloseToSectorEdge);
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

fn is_item_too_close(construction_pos: &Transform, pos: Vec2) -> bool {
    pos.distance(construction_pos.translation.truncate())
        < constants::MINIMUM_DISTANCE_BETWEEN_STATIONS + constants::STATION_GATE_PLANET_RADIUS
}
