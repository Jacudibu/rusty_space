use crate::persistence::local_hex_position::LocalHexPosition;
use crate::universe_builder::gate_builder::instance_builder::GateSpawnDataInstanceBuilder;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct GateSpawnData {
    pub gates: Vec<GateSpawnDataInstanceBuilder>,
}

impl GateSpawnData {
    pub fn add(
        &mut self,
        from: LocalHexPosition,
        to: LocalHexPosition,
    ) -> &mut GateSpawnDataInstanceBuilder {
        self.gates.push(GateSpawnDataInstanceBuilder { from, to });
        self.gates.last_mut().unwrap()
    }
}
