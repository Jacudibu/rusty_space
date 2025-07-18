use bevy::prelude::{
    Camera, GlobalTransform, Query, Res, ResMut, Resource, Single, Vec2, Window, With,
};
use camera::MainCamera;
use common::constants::BevyResult;
use common::types::entity_id_map::SectorIdMap;
use common::types::map_layout::MapLayout;
use common::types::sector_position::SectorPosition;
use hexx::Hex;
use std::ops::Deref;

/// The Current position of the mouse, in various formats.
#[derive(Resource, Default)]
pub struct MouseCursor {
    /// The cursor position in this window in physical pixels. `None` if the cursor is outside the window area.
    pub screen_space: Option<Vec2>,

    /// The cursor position converted to world space.
    pub world_space: Option<Vec2>,

    /// The cursor position converted into a [SectorPosition].
    pub sector_space: Option<MouseSectorPosition>,
}

/// Component for our [MouseCursor].
/// Contains a [SectorPosition] and the [Hex] associated with that sector to avoid evaluating one of them in other systems.
pub struct MouseSectorPosition {
    /// The [SectorPosition] represented by this instance.
    pub sector_position: SectorPosition,

    /// The [Hex] associated to the [SectorEntity] within [SectorPosition].
    pub coordinates: Hex,
}

/// Updates the [MouseCursor] Resource with new Values for this frame.
pub(crate) fn update_mouse_cursor_position(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    map: Res<MapLayout>,
    sectors: Res<SectorIdMap>,
    mut cursor: ResMut<MouseCursor>,
) -> BevyResult {
    if let Some(position) = window.cursor_position() {
        let (camera, transform) = camera.deref();
        let world_pos = camera.viewport_to_world_2d(transform, position);

        cursor.screen_space = Some(position);
        if let Ok(world_pos) = world_pos {
            cursor.world_space = Some(world_pos);
            cursor.sector_space = calculate_sector_pos(world_pos, &map, &sectors);
        } else {
            cursor.world_space = None;
            cursor.sector_space = None;
        }
    } else {
        cursor.screen_space = None;
        cursor.world_space = None;
        cursor.sector_space = None;
    }

    Ok(())
}

/// Converts the position of the mouse cursor to a sector position. None if there is no sector at the position.
fn calculate_sector_pos(
    world_pos: Vec2,
    map: &MapLayout,
    sectors: &SectorIdMap,
) -> Option<MouseSectorPosition> {
    let sector_hex = map.hex_layout.world_pos_to_hex(world_pos);
    let sector = sectors.get_entity(&sector_hex)?;
    let sector_center_pos = map.hex_layout.hex_to_world_pos(sector_hex);

    MouseSectorPosition {
        sector_position: SectorPosition {
            sector: *sector,
            local_position: world_pos - sector_center_pos,
        },
        coordinates: sector_hex,
    }
    .into()
}
