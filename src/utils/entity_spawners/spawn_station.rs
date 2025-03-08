use crate::components::{
    BuildSite, BuyOrders, InteractionQueue, Inventory, Sector, SelectableEntity, SellOrders,
    Station,
};
use crate::game_data::{ItemData, ItemManifest, RecipeManifest};
use crate::persistence::{PersistentStationId, StationIdMap};
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::production::{ProductionComponent, ShipyardComponent};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::{SectorEntity, StationEntity};
use crate::{constants, SpriteHandles};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Query, Sprite};
use bevy::sprite::Anchor;

#[allow(clippy::too_many_arguments)] // It's hopeless... :')
pub fn spawn_station(
    commands: &mut Commands,
    sector_query: &mut Query<&mut Sector>,
    station_id_map: &mut StationIdMap,
    sprites: &SpriteHandles,
    id: PersistentStationId,
    name: &str,
    local_pos: Vec2,
    sector_entity: SectorEntity,
    buys: Vec<&ItemData>,
    sells: Vec<&ItemData>,
    production: Option<ProductionComponent>,
    shipyard: Option<ShipyardComponent>,
    build_site: Option<BuildSite>,
    item_manifest: &ItemManifest,
    recipe_manifest: &RecipeManifest,
) {
    let mut sector = sector_query.get_mut(sector_entity.into()).unwrap();

    let icon_sprite = match sells.first() {
        None => {
            if shipyard.is_some() {
                sprites.icon_ship.clone()
            } else {
                sprites.icon_unknown.clone()
            }
        }
        Some(item) => item.icon.clone(),
    };

    let simulation_transform = SimulationTransform::from_translation(local_pos + sector.world_pos);

    let _icon_entity = commands
        .spawn((
            Name::new(format!("{name} (Icon)")),
            Sprite {
                image: icon_sprite,
                ..default()
            },
            simulation_transform.as_bevy_transform(constants::z_layers::STATION_ICON),
        ))
        .id();

    let build_site_id = if let Some(build_site) = build_site {
        let build_site_id = Some(build_site.id);

        // TODO: Build site has buy orders and an inventory
        // TODO: Figure out the least painful way to sync simulation transform to the position of the station
        //         (probably a new marker component + system that's run after the regular transform update)
        //         (only necessary in sectors with some form of perpetual motion)
        commands.spawn((
            Name::new(name.to_string() + " (Build Site)"),
            build_site,
            simulation_transform.as_bevy_transform(constants::z_layers::BUILD_SITE),
            Sprite {
                image: sprites.building_site.clone(),
                anchor: Anchor::Custom(Vec2::splat(-0.7)),
                ..Default::default()
            },
            SimulationTransform::new(
                simulation_transform.translation,
                simulation_transform.rotation,
            ),
            Inventory::new(u32::MAX), // TODO: This should be a special inventory with precise capacities for the required materials
            BuyOrders::default(), // TODO: These should sync with the inventory capacities. Prices are a bit complicated, but MAX is enough for starters.
        ));

        build_site_id
    } else {
        None
    };

    let entity = commands
        .spawn((
            Name::new(name.to_string()),
            SelectableEntity::Station,
            Station::new(id, build_site_id),
            Sprite::from_image(sprites.station.clone()),
            simulation_transform.as_bevy_transform(constants::z_layers::STATION),
            simulation_transform,
            InteractionQueue::new(constants::SIMULTANEOUS_STATION_INTERACTIONS),
            SimulationScale::default(),
        ))
        .id();

    let buy_sell_and_production_count = {
        // This doesn't yet account for duplicates in case we ever want to copy this somewhere after the mock-data era is over
        // (so items which are produced, sold *and* purchased will count twice or thrice)
        let mut buy_sell_and_production_count = (buys.len() + sells.len()) as u32;
        if let Some(production) = &production {
            for (_, module) in &production.modules {
                let recipe = recipe_manifest.get_by_ref(&module.recipe).unwrap();
                buy_sell_and_production_count += recipe.output.len() as u32;
            }
        }

        buy_sell_and_production_count
    };

    let mut inventory = Inventory::new(constants::MOCK_STATION_INVENTORY_SIZE);

    let fill_ratio = 2;
    for sold_item in &sells {
        inventory.add_item(
            sold_item.id,
            constants::MOCK_STATION_INVENTORY_SIZE
                / buy_sell_and_production_count
                / sold_item.size
                / fill_ratio,
            item_manifest,
        )
    }

    for purchased_item in &buys {
        inventory.add_item(
            purchased_item.id,
            constants::MOCK_STATION_INVENTORY_SIZE
                / buy_sell_and_production_count
                / purchased_item.size
                / fill_ratio,
            item_manifest,
        )
    }

    // Reserve storage space for products
    if let Some(production) = &production {
        for (_, module) in &production.modules {
            let recipe = recipe_manifest.get_by_ref(&module.recipe).unwrap();
            for x in &recipe.output {
                let item = item_manifest.get_by_ref(&x.item_id).unwrap();
                let amount = constants::MOCK_STATION_INVENTORY_SIZE
                    / buy_sell_and_production_count
                    / item.size;

                inventory.set_production_reservation(&x.item_id, amount, item_manifest);
            }
        }
    }

    if !buys.is_empty() {
        commands
            .entity(entity)
            .insert(BuyOrders::mock(&buys, &sells));
    }
    if !sells.is_empty() {
        commands.entity(entity).insert(SellOrders::mock(
            &buys,
            &sells,
            &mut inventory,
            item_manifest,
        ));
    }

    if let Some(production) = production {
        commands.entity(entity).insert(production);
    }

    if let Some(shipyard) = shipyard {
        commands.entity(entity).insert(shipyard);
    }

    commands.entity(entity).insert(inventory);

    station_id_map.insert(id, StationEntity::from(entity));
    sector.add_station(commands, sector_entity, StationEntity::from(entity));
}
