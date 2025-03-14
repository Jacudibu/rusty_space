use crate::components::{
    BuyOrders, ConstructionSiteComponent, InteractionQueue, Inventory, SectorComponent,
    SelectableEntity, SellOrders, StationComponent,
};
use crate::game_data::{ItemData, ItemManifest, RecipeManifest};
use crate::persistence::{
    ConstructionSiteIdMap, PersistentConstructionSiteId, PersistentStationId, StationIdMap,
};
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::production::{ProductionComponent, ShipyardComponent};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::{ConstructionSiteEntity, SectorEntity, StationEntity};
use crate::{SpriteHandles, constants};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Query, Sprite, default};
use bevy::sprite::Anchor;

#[allow(clippy::too_many_arguments)] // It's hopeless... :')
pub fn spawn_station(
    commands: &mut Commands,
    sector_query: &mut Query<&mut SectorComponent>,
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
    item_manifest: &ItemManifest,
    recipe_manifest: &RecipeManifest,
) -> StationEntity {
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

    let entity = commands
        .spawn((
            Name::new(name.to_string()),
            SelectableEntity::Station,
            StationComponent::new(id, None),
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
                for element in &module.queued_recipes {
                    let recipe = recipe_manifest.get_by_ref(&element.recipe).unwrap();
                    buy_sell_and_production_count += recipe.output.len() as u32;
                }
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
            for element in &module.queued_recipes {
                let recipe = recipe_manifest.get_by_ref(&element.recipe).unwrap();
                // TODO: This probably doesn't work once we have something with multiple recipes
                for x in &recipe.output {
                    let item = item_manifest.get_by_ref(&x.item_id).unwrap();
                    let amount = constants::MOCK_STATION_INVENTORY_SIZE
                        / buy_sell_and_production_count
                        / item.size;

                    inventory.set_production_reservation(&x.item_id, amount, item_manifest);
                }
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

    station_id_map.insert(id, entity.into());
    sector.add_station(commands, sector_entity, entity.into());

    entity.into()
}

#[allow(clippy::too_many_arguments)] // It's hopeless... :')
pub fn spawn_construction_site(
    commands: &mut Commands,
    construction_site: ConstructionSiteComponent,
    construction_site_id_map: &mut ConstructionSiteIdMap,
    sector_query: &mut Query<&mut SectorComponent>,
    sprites: &SpriteHandles,
    id: PersistentConstructionSiteId,
    name: &str,
    local_pos: Vec2,
    sector_entity: SectorEntity,
) -> ConstructionSiteEntity {
    let mut sector = sector_query.get_mut(sector_entity.into()).unwrap();
    let simulation_transform = SimulationTransform::from_translation(local_pos + sector.world_pos);

    // TODO: Build site has buy orders and an inventory
    // TODO: Figure out the least painful way to sync simulation transform to the position of the station
    //         (probably a new marker component + system that's run after the regular transform update)
    //         (only necessary in sectors with some form of perpetual motion)
    let entity = commands
        .spawn((
            Name::new(name.to_string() + " (Build Site)"),
            construction_site,
            simulation_transform.as_bevy_transform(constants::z_layers::BUILD_SITE),
            Sprite {
                image: sprites.construction_site.clone(),
                anchor: Anchor::Custom(Vec2::splat(-0.7)),
                ..Default::default()
            },
            SimulationTransform::new(
                simulation_transform.translation,
                simulation_transform.rotation,
            ),
            Inventory::new(u32::MAX), // TODO: This should be a special inventory with precise capacities for the required materials
            BuyOrders::default(), // TODO: These should sync with the inventory capacities. Prices are a bit complicated, but MAX is enough for starters.
        ))
        .id();

    let entity = ConstructionSiteEntity::from(entity);
    construction_site_id_map.insert(id, entity);
    sector.add_construction_site(commands, sector_entity, entity);
    entity
}
