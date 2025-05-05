use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct SpriteHandles {
    pub gate: Handle<Image>,
    pub gate_selected: Handle<Image>,
    pub planet: Handle<Image>,
    pub planet_selected: Handle<Image>,
    pub star: Handle<Image>,
    pub star_selected: Handle<Image>,
    pub station: Handle<Image>,
    pub station_selected: Handle<Image>,
    pub construction_site: Handle<Image>,
    pub icon_unknown: Handle<Image>,
    pub icon_ship: Handle<Image>,
}
