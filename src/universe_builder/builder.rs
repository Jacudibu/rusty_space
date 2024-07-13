use crate::universe_builder::{gate_builder, sector_builder};

#[derive(Default)]
pub struct UniverseBuilder {
    pub sectors: sector_builder::SectorSpawnData,
    pub gates: gate_builder::GateSpawnData,
}

#[cfg(test)]
mod test_helpers {
    use super::*;
    use crate::asteroids::SectorWasSpawnedEvent;
    use crate::gizmos::SetupGateConnectionEvent;
    use crate::map_layout::MapLayout;
    use crate::universe_builder::plugin::UniverseBuilderPlugin;
    use crate::SpriteHandles;
    use bevy::prelude::*;

    impl UniverseBuilder {
        pub fn build_test_app(self) -> App {
            let mut app = App::new();
            app.init_resource::<MapLayout>();
            app.init_resource::<SpriteHandles>();
            app.add_event::<SectorWasSpawnedEvent>();
            app.add_event::<SetupGateConnectionEvent>();
            app.insert_resource(self.sectors);
            app.insert_resource(self.gates);

            app.add_plugins(UniverseBuilderPlugin);
            app.finish();
            app.update();

            app
        }
    }
}
