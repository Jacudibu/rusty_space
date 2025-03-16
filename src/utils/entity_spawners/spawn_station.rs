use crate::components::{
    BuyOrders, ConstructionSiteComponent, ConstructionSiteStatus, InteractionQueue, Inventory,
    SectorComponent, SelectableEntity, SellOrders, StationComponent,
};
use crate::game_data::{ConstructableModuleId, ItemId, ItemManifest, RecipeManifest};
use crate::persistence::{
    ConstructionSiteIdMap, PersistentConstructionSiteId, PersistentStationId, StationIdMap,
};
use crate::simulation::prelude::simulation_transform::SimulationScale;
use crate::simulation::production::{ProductionComponent, ShipyardComponent};
use crate::simulation::transform::simulation_transform::SimulationTransform;
use crate::utils::{ConstructionSiteEntity, SectorPosition, StationEntity};
use crate::{SpriteHandles, constants};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Query, Sprite, default};
use bevy::sprite::Anchor;
use std::ops::Not;

/// Spawn Data for new Stations.
pub struct StationSpawnData {
    /// The persistent entity id for this station.
    pub id: PersistentStationId,
    /// The name station should have
    pub name: String,
    /// The position at which this station should be spawned
    pub sector_position: SectorPosition,
    /// Unless we are loading a save file, new stations should always spawn with a construction site.
    pub construction_site: Option<ConstructionSiteSpawnData>,
    /// Buy data for this Station.
    pub buys: Vec<ItemId>,
    /// Sell data for this Station.
    pub sells: Vec<ItemId>,
    /// Production data for this Station.
    pub production: Option<ProductionComponent>,
    /// The shipyard component for this station.
    pub shipyard: Option<ShipyardComponent>,
}

impl StationSpawnData {
    /// Creates a new instance of [StationSpawnData] with defaults for all values which should only be set when loading save games.
    pub fn new(
        name: impl Into<String>,
        construction_site: ConstructionSiteSpawnData,
        sector_position: SectorPosition,
    ) -> Self {
        Self {
            id: PersistentStationId::next(),
            name: name.into(),
            sector_position,
            construction_site: Some(construction_site),
            buys: Default::default(),
            sells: Default::default(),
            production: Default::default(),
            shipyard: Default::default(),
        }
    }
}

/// Spawn Data for new Construction Sites.
pub struct ConstructionSiteSpawnData {
    /// The [PersistentEntityId] our construction site will have. Defaults to the next available id.
    pub id: PersistentConstructionSiteId,
    /// The modules which should be built. This should never be empty.
    pub build_order: Vec<ConstructableModuleId>,
    /// The current progress in building this site. Defaults to 0.
    pub current_progress: f32,
}

impl ConstructionSiteSpawnData {
    /// Validates the provided arguments and returns a new instance of [ConstructionSiteSpawnData] with defaults for all values which should only be set when loading save games.
    pub fn new(build_order: Vec<ConstructableModuleId>) -> Self {
        debug_assert!(
            build_order.is_empty().not(),
            "New ConstructionSite build_order should never be empty!"
        );

        Self {
            id: PersistentConstructionSiteId::next(),
            build_order,
            current_progress: 0.0,
        }
    }
}

/// Creates a new Station Entity with all the required bells and whistles attached.
/// Unless we are loading a save file, new stations should always spawn with a construction site and no modules built or.
#[allow(clippy::too_many_arguments)] // It's hopeless... :')
pub fn spawn_station(
    commands: &mut Commands,
    sector_query: &mut Query<&mut SectorComponent>,
    station_id_map: &mut StationIdMap,
    construction_site_id_map: &mut ConstructionSiteIdMap,
    sprites: &SpriteHandles,
    item_manifest: &ItemManifest,
    recipe_manifest: &RecipeManifest,
    data: StationSpawnData,
) -> StationEntity {
    let mut sector = sector_query
        .get_mut(data.sector_position.sector.into())
        .unwrap();

    // TODO: Station Icon should be part of spawn data
    let icon_sprite = match data.sells.first() {
        None => {
            if data.shipyard.is_some() {
                sprites.icon_ship.clone()
            } else {
                sprites.icon_unknown.clone()
            }
        }
        Some(item) => item_manifest.get_by_ref(item).unwrap().icon.clone(),
    };

    let simulation_transform = SimulationTransform::from_translation(
        data.sector_position.local_position + sector.world_pos,
    );

    // TODO: Icon EntityID needs to be persisted since we want to be able to change it
    let _icon_entity = commands
        .spawn((
            Name::new(format!("{} (Icon)", data.name)),
            Sprite {
                image: icon_sprite,
                ..default()
            },
            simulation_transform.as_bevy_transform(constants::z_layers::STATION_ICON),
        ))
        .id();

    let entity = commands
        .spawn((
            Name::new(data.name.clone()),
            SelectableEntity::Station,
            Sprite::from_image(sprites.station.clone()),
            simulation_transform.as_bevy_transform(constants::z_layers::STATION),
            simulation_transform,
            InteractionQueue::new(constants::SIMULTANEOUS_STATION_INTERACTIONS),
            SimulationScale::default(),
        ))
        .id();

    sector.add_station(commands, data.sector_position.sector, entity.into());

    let construction_site = data.construction_site.map(|construction_site| {
        spawn_construction_site(
            commands,
            construction_site_id_map,
            sector_query,
            sprites,
            &data.name,
            data.sector_position,
            entity.into(),
            construction_site,
        )
    });

    let mut entity_commands = commands.entity(entity);
    entity_commands.insert(StationComponent::new(data.id, construction_site));

    let buy_sell_and_production_count = {
        // This doesn't yet account for duplicates in case we ever want to copy this somewhere after the mock-data era is over
        // (so items which are produced, sold *and* purchased will count twice or thrice)
        let mut buy_sell_and_production_count = (data.buys.len() + data.sells.len()) as u32;
        if let Some(production) = &data.production {
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
    // TODO: Remove mock data
    for sold_item in &data.sells {
        inventory.add_item(
            *sold_item,
            constants::MOCK_STATION_INVENTORY_SIZE
                / buy_sell_and_production_count
                / item_manifest.get_by_ref(sold_item).unwrap().size
                / fill_ratio,
            item_manifest,
        )
    }

    // TODO: Remove mock data
    for purchased_item in &data.buys {
        inventory.add_item(
            *purchased_item,
            constants::MOCK_STATION_INVENTORY_SIZE
                / buy_sell_and_production_count
                / item_manifest.get_by_ref(purchased_item).unwrap().size
                / fill_ratio,
            item_manifest,
        )
    }

    // Reserve storage space for products
    if let Some(production) = &data.production {
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

    if !data.buys.is_empty() {
        entity_commands.insert(BuyOrders::mock(
            &data
                .buys
                .iter()
                .map(|x| item_manifest.get_by_ref(x).unwrap())
                .collect::<Vec<_>>(),
            &data
                .sells
                .iter()
                .map(|x| item_manifest.get_by_ref(x).unwrap())
                .collect::<Vec<_>>(),
        ));
    }
    if !data.sells.is_empty() {
        entity_commands.insert(SellOrders::mock(
            &data
                .buys
                .iter()
                .map(|x| item_manifest.get_by_ref(x).unwrap())
                .collect::<Vec<_>>(),
            &data
                .sells
                .iter()
                .map(|x| item_manifest.get_by_ref(x).unwrap())
                .collect::<Vec<_>>(),
            &mut inventory,
            item_manifest,
        ));
    }

    if let Some(production) = data.production {
        entity_commands.insert(production);
    }

    if let Some(shipyard) = data.shipyard {
        entity_commands.insert(shipyard);
    }

    entity_commands.insert(inventory);
    station_id_map.insert(data.id, entity.into());
    entity.into()
}

/// Spawns a construction site for the specified station entity.
/// This can happen in two situations:
///
/// a) A new station was just spawned. A construction site will be automatically attached to it.
///
/// b) An existing construction site has triggered expansion (not yet implemented)
#[allow(clippy::too_many_arguments)] // It's hopeless... :')
fn spawn_construction_site(
    commands: &mut Commands,
    construction_site_id_map: &mut ConstructionSiteIdMap,
    sector_query: &mut Query<&mut SectorComponent>,
    sprites: &SpriteHandles,
    station_name: &str,
    sector_position: SectorPosition,
    station_entity: StationEntity,
    data: ConstructionSiteSpawnData,
) -> ConstructionSiteEntity {
    let mut sector = sector_query.get_mut(sector_position.sector.into()).unwrap();
    let simulation_transform =
        SimulationTransform::from_translation(sector_position.local_position + sector.world_pos);

    // TODO: implement proper construction site... constructing
    let construction_site = ConstructionSiteComponent {
        id: data.id,
        station: station_entity,
        build_order: data.build_order,
        current_build_progress: data.current_progress,
        total_build_power: 0,
        construction_ships: Default::default(),
        status: ConstructionSiteStatus::MissingBuilders,
    };

    // TODO: Build site has buy orders and an inventory
    // TODO: Figure out the least painful way to sync simulation transform to the position of the station
    //         (probably a new marker component + system that's run after the regular transform update)
    //         (only necessary in sectors with some form of perpetual motion)
    let entity = commands
        .spawn((
            Name::new(station_name.to_string() + " (Build Site)"),
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
    construction_site_id_map.insert(data.id, entity);
    sector.add_construction_site(commands, sector_position.sector, entity);
    entity
}
