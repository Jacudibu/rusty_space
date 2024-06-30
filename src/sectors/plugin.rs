use crate::sectors::sector::spawn_sector;
use crate::sectors::sector_outlines::{draw_sector_outlines, SectorOutlineGizmos};
use bevy::app::Update;
use bevy::prelude::{App, AppGizmoBuilder, Commands, Plugin, Res, Resource, Startup};
use hexx::{Hex, HexLayout, HexOrientation, Vec2};

pub struct SectorPlugin;
impl Plugin for SectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapLayout>()
            .init_gizmo_group::<SectorOutlineGizmos>()
            .add_systems(Startup, startup)
            .add_systems(Update, draw_sector_outlines);
    }
}

fn startup(mut commands: Commands, map_layout: Res<MapLayout>) {
    for hex in hexx::shapes::hexagon(Hex::ZERO, 5) {
        spawn_sector(&mut commands, &map_layout.hex_layout, hex);
    }
}

#[derive(Resource)]
pub struct MapLayout {
    pub hex_layout: HexLayout,
}

impl Default for MapLayout {
    fn default() -> Self {
        MapLayout {
            hex_layout: HexLayout {
                orientation: HexOrientation::Pointy,
                hex_size: Vec2::splat(500.0),
                ..Default::default()
            },
        }
    }
}
