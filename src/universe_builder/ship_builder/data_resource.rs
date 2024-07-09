use crate::ship_ai::BehaviorBuilder;
use crate::universe_builder::local_hex_position::LocalHexPosition;
use crate::universe_builder::ship_builder::instance_builder::ShipSpawnDataInstanceBuilder;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct ShipSpawnData {
    pub ships: Vec<ShipSpawnDataInstanceBuilder>,
}

impl ShipSpawnData {
    pub fn add(
        &mut self,
        position: LocalHexPosition,
        rotation: f32,
        name: String,
        behavior: BehaviorBuilder,
    ) -> &mut ShipSpawnDataInstanceBuilder {
        self.ships.push(ShipSpawnDataInstanceBuilder::new(
            position, rotation, name, behavior,
        ));
        return self.ships.last_mut().unwrap();
    }
}
