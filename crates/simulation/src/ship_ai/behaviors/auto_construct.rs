use crate::ship_ai::task_filters::ShipIsIdleFilter;
use bevy::prelude::{Entity, EventWriter, Query, Res};
use common::components::ship_behavior::ShipBehavior;
use common::components::{InSector, Sector};
use common::constants;
use common::events::task_events::{InsertTaskIntoQueueCommand, TaskInsertionMode};
use common::simulation_time::SimulationTime;
use common::types::entity_wrappers::ConstructionSiteEntity;
use common::types::ship_behaviors::AutoConstructBehavior;
use common::types::ship_tasks::Construct;
use std::ops::Not;

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    simulation_time: Res<SimulationTime>,
    mut ships: Query<
        (Entity, &mut ShipBehavior<AutoConstructBehavior>, &InSector),
        ShipIsIdleFilter,
    >,
    all_sectors: Query<&Sector>,
    mut event_writer: EventWriter<InsertTaskIntoQueueCommand<Construct>>,
) {
    let now = simulation_time.now();

    ships
        .iter_mut()
        .filter(|(_, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut behavior, in_sector)| {
            let Some(build_site) = find_nearby_sector_with_build_site(&all_sectors, in_sector)
            else {
                behavior.next_idle_update =
                    now.add_seconds(constants::SECONDS_BETWEEN_SHIP_BEHAVIOR_IDLE_UPDATES);
                return;
            };

            event_writer.write(InsertTaskIntoQueueCommand {
                task_data: Construct { target: build_site },
                entity: ship_entity,
                insertion_mode: TaskInsertionMode::Append,
            });
        })
}

#[must_use]
fn find_nearby_sector_with_build_site(
    all_sectors: &Query<&Sector>,
    in_sector: &InSector,
) -> Option<ConstructionSiteEntity> {
    let nearby_sectors_with_build_sites =
        pathfinding::surrounding_sector_search::surrounding_sector_search(
            all_sectors,
            in_sector.sector,
            0,
            u8::MAX, // TODO: Should be limited
            all_sectors,
            |x| x.construction_sites.is_empty().not(),
        );

    if nearby_sectors_with_build_sites.is_empty() {
        return None;
    }

    let target_sector = nearby_sectors_with_build_sites.iter().min()?;
    let target_build_site = all_sectors
        .get(target_sector.sector.into())
        .unwrap()
        .construction_sites
        .iter()
        .next()
        .unwrap();

    Some(*target_build_site)
}
