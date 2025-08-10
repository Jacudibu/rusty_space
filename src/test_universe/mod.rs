use crate::initialize_data;
use bevy::app::{App, Plugin};
use bevy::prelude::{IntoScheduleConfigs, Name, Startup, World};
use common::components::{Faction, LocalPlayerFaction, Player};
use common::game_data::AsteroidManifest;
use common::session_data::SessionData;
use common::types::entity_id_map::{FactionIdMap, PlayerIdMap};
use common::types::persistent_entity_id::{PersistentFactionId, PersistentPlayerId};

mod coordinates;
mod gate_test_data;
mod sector_test_data;
mod ship_test_data;
mod station_test_data;

pub struct TestUniverseDataPlugin;
impl Plugin for TestUniverseDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_test_universe.after(initialize_data));
    }
}

pub fn load_test_universe(world: &mut World) {
    SessionData::initialize_mock_data(world);

    let player_faction = spawn_test_player_faction(world);

    world.insert_resource(sector_test_data::create_test_data(
        player_faction,
        world
            .get_resource::<AsteroidManifest>()
            .expect("Manifests should be parsed before TestUniversePlugin is added!"),
    ));
    world.insert_resource(gate_test_data::create_test_data());
    world.insert_resource(station_test_data::create_test_data(player_faction));
    world.insert_resource(ship_test_data::create_test_data(player_faction));
}

fn spawn_test_player_faction(world: &mut World) -> PersistentFactionId {
    let player_id = PersistentPlayerId::next();
    let faction_id = PersistentFactionId::next();
    world.insert_resource(LocalPlayerFaction { faction_id });

    let faction_entity = world
        .spawn((
            Name::new("Player Faction"),
            Faction {
                faction_id,
                faction_color: bevy::color::palettes::css::LIME.into(),
                players: vec![player_id],
            },
        ))
        .id();

    let mut faction_id_map = FactionIdMap::default();
    faction_id_map.insert(faction_id, faction_entity.into());
    world.insert_resource(faction_id_map);

    let player_entity = world
        .spawn((Name::new("Player"), Player { player_id }))
        .id();

    let mut player_id_map = PlayerIdMap::default();
    player_id_map.insert(player_id, player_entity.into());
    world.insert_resource(player_id_map);

    faction_id
}
