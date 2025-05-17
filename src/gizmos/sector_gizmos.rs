use bevy::app::{App, Plugin};
use bevy::gizmos::GizmoAsset;
use bevy::prelude::{Assets, ChildOf, Commands, Entity, Gizmo, Name, OnEnter, Query, Res, ResMut};
use common::components::Sector;
use common::constants::BevyResult;
use common::states::ApplicationState;
use common::types::map_layout::MapLayout;

pub struct SectorGizmoPlugin;
impl Plugin for SectorGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ApplicationState::InGame),
            spawn_retained_sector_outlines,
        );
    }
}

fn spawn_retained_sector_outlines(
    sectors: Query<(Entity, &Sector)>,
    layout: Res<MapLayout>,
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) -> BevyResult {
    // TODO: Looks like we can store one GizmoAsset per possible sector color in a separate resource
    //       and then just insert a different handle later in case sector ownership changes. Neat!
    let mut gizmo = GizmoAsset::default();
    for edge in layout.hex_edge_vertices {
        gizmo.line_2d(edge[0], edge[1], bevy::color::palettes::css::YELLOW);
    }

    let handle = gizmo_assets.add(gizmo);

    for (entity, sector) in sectors.iter() {
        commands
            .spawn((
                Name::new(format!(
                    "Sector [{}/{}] Outline",
                    sector.coordinate.x, sector.coordinate.y
                )),
                Gizmo {
                    handle: handle.clone(),
                    ..Default::default()
                },
            ))
            .insert(ChildOf(entity));
    }

    Ok(())
}
