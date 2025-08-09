use crate::types::persistent_entity_id::{PersistentFactionId, PersistentPlayerId};
use bevy::prelude::{Color, Component, Resource};

/// Entities with this component are owned by a faction - than can be either NPCs or players.
/// For obvious reasons, players may only issue commands to entities they themselves own.
#[derive(Component)]
#[component(immutable)]
pub struct Owner {
    /// The unique identifier of the owner of the entity with this component.
    pub faction_id: PersistentFactionId,
}

/// Holds a reference to the [Faction] of the local player.
#[derive(Resource)]
pub struct LocalPlayerFaction {
    /// The unique identifier of the faction representing the local player.
    pub faction_id: PersistentFactionId,
}

/// A Faction is a grouping of multiple players or AIs, sharing ownership and control over entities.
#[derive(Component)]
pub struct Faction {
    /// The unique identifier of this faction.
    pub faction_id: PersistentFactionId,

    /// Multiple players may share a faction to facilitate cooperative play.
    pub players: Vec<PersistentPlayerId>,

    /// A list of NPCs
    /// TODO: This way, players and NPCs could share a faction.
    ///       This could be used by players to automate certain things.
    ///       We could also split AI into multiple agents with different responsibilities...
    // pub non_player_characters: Vec<u64>,

    /// The color which should be used to tint entities belonging to this faction.
    pub faction_color: Color,
}

/// An entity with this component represents a human player.
#[derive(Component)]
#[component(immutable)]
pub struct Player {
    /// The unique identifier of this player.
    pub player_id: PersistentPlayerId,
}

// /// An entity with this component represents a computer player.
// #[derive(Component)]
// #[component(immutable)]
// pub struct NonPlayerCharacter {
//     /// The unique identifier of this player.
//     pub npc_id: u64, // TODO
// }
