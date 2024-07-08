use crate::universe_builder::gate_builder::instance_builder::{
    GateSpawnDataInstanceBuilder, HexPosition,
};
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct GateSpawnData {
    pub gates: Vec<GateSpawnDataInstanceBuilder>,
}

impl GateSpawnData {
    pub fn add(&mut self, from: HexPosition, to: HexPosition) -> &mut GateSpawnDataInstanceBuilder {
        self.gates.push(GateSpawnDataInstanceBuilder { from, to });
        self.gates.last_mut().unwrap()
    }
}
