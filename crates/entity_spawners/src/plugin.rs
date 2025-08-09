use bevy::prelude::{Added, App, Query, Res, Sprite, Update};
use common::components::{Faction, Owner};
use common::constants::BevyResult;
use common::types::entity_id_map::FactionIdMap;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_ship_color_to_match_owning_faction);
}

fn update_ship_color_to_match_owning_faction(
    added: Query<(&mut Sprite, &Owner), Added<Owner>>,
    factions: Query<&Faction>,
    faction_id_map: Res<FactionIdMap>,
) -> BevyResult {
    for (mut sprite, owner) in added {
        let faction = factions.get(
            faction_id_map
                .get_entity(&owner.faction_id)
                .expect("FactionIds should always be mappable")
                .into(),
        )?;
        sprite.color = faction.faction_color;
    }

    Ok(())
}
