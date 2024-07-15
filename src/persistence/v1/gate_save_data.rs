use crate::components::{Gate, InSector, Sector};
use crate::persistence::AllEntityIdMaps;
use bevy::math::Vec2;
use bevy::prelude::{Query, Transform};
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GatePairSaveData {
    pub from_sector: Hex,
    pub from_position: Vec2,
    pub to_sector: Hex,
    pub to_position: Vec2,
}

impl GatePairSaveData {
    pub fn extract_from_sector_query(
        sectors: &Query<&Sector>,
        gates: &Query<(&Gate, &InSector, &Transform)>,
        all_entity_id_maps: &AllEntityIdMaps,
    ) -> Vec<GatePairSaveData> {
        sectors
            .iter()
            .flat_map(|x| x.gates.iter().map(|(_, gate_pair)| gate_pair))
            .filter_map(|gate_pair| {
                let [(from_gate, from_sector, from_transform), (to_gate, to_sector, to_transform)] =
                    gates
                        .get_many([gate_pair.from.into(), gate_pair.to.into()])
                        .unwrap();

                // Every GatePair is defined twice; Once for every sector it's in.
                if from_sector.sector > to_sector.sector {
                    return None;
                }

                // That should just never happen...
                debug_assert!(from_sector.sector != to_sector.sector);

                let from_hex = all_entity_id_maps.sectors.entity_to_id()[&from_sector.sector];
                let to_hex = all_entity_id_maps.sectors.entity_to_id()[&to_sector.sector];

                Some(GatePairSaveData {
                    from_sector: from_hex,
                    from_position: from_transform.translation.truncate(),
                    to_sector: to_hex,
                    to_position: to_transform.translation.truncate(),
                })
            })
            .collect()
    }
}
