use crate::utils::SectorEntity;
use bevy::prelude::Resource;
use bevy::utils::HashMap;
use hexx::Hex;

#[derive(Resource)]
pub struct HexToSectorEntityMap {
    pub(crate) map: HashMap<Hex, SectorEntity>,
}
