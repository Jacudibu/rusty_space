use crate::components::{Sector, SectorStarComponent, Star};
use crate::persistence::data::v1::*;
use crate::persistence::{PersistentPlanetId, PlanetIdMap, SectorIdMap};
use crate::utils::{spawn_helpers, EarthMass};
use crate::SpriteHandles;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Query, Res};
use hexx::Hex;

#[derive(SystemParam)]
pub struct Args<'w, 's> {
    commands: Commands<'w, 's>,
    sprites: Res<'w, SpriteHandles>,
    sectors: Query<'w, 's, (&'static mut Sector, &'static SectorStarComponent)>,
    stars: Query<'w, 's, &'static Star>,
    sector_id_map: Res<'w, SectorIdMap>,
}

type SaveData = SaveDataCollection<PlanetSaveData>;

pub fn spawn_all(data: Res<SaveData>, mut args: Args) {
    let mut planet_id_map = PlanetIdMap::new();
    for builder in &data.data {
        builder.build(&mut args, &mut planet_id_map);
    }

    args.commands.remove_resource::<SaveData>();
    args.commands.insert_resource(planet_id_map);
}

impl SaveData {
    pub fn add(
        &mut self,
        name: String,
        sector: Hex,
        mass: EarthMass,
        orbit: ConstantOrbitSaveData,
    ) -> &mut PlanetSaveData {
        self.data.push(PlanetSaveData {
            id: PersistentPlanetId::next(),
            name,
            sector,
            mass,
            orbit,
        });
        self.data.last_mut().unwrap()
    }
}

impl PlanetSaveData {
    pub fn build(&self, args: &mut Args, planet_id_map: &mut PlanetIdMap) {
        let sector_entity = args.sector_id_map.id_to_entity()[&self.sector];

        spawn_helpers::spawn_planet(
            &mut args.commands,
            planet_id_map,
            &args.sprites,
            self.name.clone(),
            &mut args.sectors,
            &args.stars,
            sector_entity,
            self.orbit.radius,
            self.orbit.current_rotational_fraction,
            self.mass,
        )
    }
}
