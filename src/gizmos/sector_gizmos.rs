use bevy::app::{App, Plugin};
use bevy::gizmos::GizmoAsset;
use bevy::prelude::{
    Assets, ChildOf, Color, Commands, Entity, Gizmo, Handle, IntoScheduleConfigs, Name, OnEnter,
    Query, Res, ResMut, Resource,
};
use common::components::{Faction, Owner, Sector};
use common::constants;
use common::constants::BevyResult;
use common::states::ApplicationState;
use common::types::map_layout::MapLayout;
use common::types::persistent_entity_id::PersistentFactionId;
use std::collections::HashMap;

pub struct SectorGizmoPlugin;
impl Plugin for SectorGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ApplicationState::InGame),
            (initialize_cache, spawn_retained_sector_outlines).chain(),
        );
    }
}

/// Holds the individually colored AssetHandles for sector outlines, one per faction.
#[derive(Resource)]
struct SectorFactionColorSectorAssetGizmoCache {
    pub neutral: Handle<GizmoAsset>,
    pub faction_assets: HashMap<PersistentFactionId, Handle<GizmoAsset>>,
}

fn initialize_cache(
    factions: Query<&Faction>,
    layout: Res<MapLayout>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    mut commands: Commands,
) {
    let neutral = gizmo_assets.add(generate_sector_outline_gizmo_asset(
        &layout,
        constants::NEUTRAL_COLOR,
    ));

    let mut handles = HashMap::new();
    for x in factions {
        handles.insert(
            x.faction_id,
            gizmo_assets.add(generate_sector_outline_gizmo_asset(
                &layout,
                x.faction_color,
            )),
        );
    }

    commands.insert_resource(SectorFactionColorSectorAssetGizmoCache {
        neutral,
        faction_assets: handles,
    })
}

fn generate_sector_outline_gizmo_asset(layout: &MapLayout, color: Color) -> GizmoAsset {
    let mut gizmo = GizmoAsset::default();
    for edge in layout.hex_edge_vertices {
        gizmo.line_2d(edge[0], edge[1], color);
    }
    gizmo
}

fn spawn_retained_sector_outlines(
    sectors: Query<(Entity, &Sector, Option<&Owner>)>,
    mut commands: Commands,
    gizmos: Res<SectorFactionColorSectorAssetGizmoCache>,
) -> BevyResult {
    // TODO: Looks like we can store one GizmoAsset per possible sector color in a separate resource
    //       and then just insert a different handle later in case sector ownership changes. Neat!

    for (entity, sector, owner) in sectors.iter() {
        let asset_handle = if let Some(owner) = owner {
            gizmos
                .faction_assets
                .get(&owner.faction_id)
                .expect("Faction gizmo should have been generated above!")
        } else {
            &gizmos.neutral
        };

        commands
            .spawn((
                Name::new(format!(
                    "Sector [{}/{}] Outline",
                    sector.coordinate.x, sector.coordinate.y
                )),
                Gizmo {
                    handle: asset_handle.clone(),
                    ..Default::default()
                },
            ))
            .insert(ChildOf(entity));
    }

    Ok(())
}
