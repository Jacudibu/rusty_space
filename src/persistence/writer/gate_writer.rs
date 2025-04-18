use bevy::prelude::Query;

use crate::components::{GateComponent, InSector, SectorComponent};
use crate::persistence::ComponentWithPersistentId;
use crate::persistence::data::v1::*;
use crate::persistence::local_hex_position::LocalHexPosition;
use crate::simulation::transform::simulation_transform::SimulationTransform;

impl GatePairSaveData {
    pub fn extract_from_sector_query(
        sectors: &Query<&SectorComponent>,
        gates: &Query<(&GateComponent, &InSector, &SimulationTransform)>,
    ) -> Vec<GatePairSaveData> {
        sectors
            .iter()
            .flat_map(|x| x.gates.iter().map(|(_, gate_pair)| gate_pair))
            .filter_map(|gate_pair| {
                let [
                    (from_gate, from_sector, from_transform),
                    (to_gate, to_sector, to_transform),
                ] = gates
                    .get_many([gate_pair.from.into(), gate_pair.to.into()])
                    .unwrap();

                // Every GatePair is defined twice; Once for every sector it's in.
                if from_sector.sector > to_sector.sector {
                    return None;
                }

                // And that should just never happen...
                debug_assert!(from_sector.sector != to_sector.sector);

                let [from_sector, to_sector] = sectors
                    .get_many([from_sector.sector.into(), to_sector.sector.into()])
                    .unwrap();

                Some(GatePairSaveData {
                    from_id: from_gate.id(),
                    from_position: LocalHexPosition::from(from_sector, from_transform),
                    to_id: to_gate.id(),
                    to_position: LocalHexPosition::from(to_sector, to_transform),
                })
            })
            .collect()
    }
}
