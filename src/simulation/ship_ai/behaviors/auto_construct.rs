use crate::components::{InSector, SectorComponent};
use crate::simulation::prelude::{
    SimulationTime, SimulationTimestamp, SimulationTransform, TaskInsideQueue, TaskQueue,
};
use crate::simulation::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::simulation::ship_ai::task_started_event::AllTaskStartedEventWriters;
use crate::utils::{ConstructionSiteEntity, SectorEntity, TypedEntity};
use crate::{constants, pathfinding};
use bevy::prelude::{Commands, Component, Entity, Query, Res};
use std::ops::Not;

#[derive(Component)]
pub struct AutoConstructBehavior {
    pub next_idle_update: SimulationTimestamp,
}

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    mut commands: Commands,
    simulation_time: Res<SimulationTime>,
    mut ships: Query<
        (
            Entity,
            &mut TaskQueue,
            &mut AutoConstructBehavior,
            &InSector,
        ),
        ShipIsIdleFilter,
    >,
    all_sectors: Query<&SectorComponent>,
    all_transforms: Query<&SimulationTransform>,
    mut all_task_started_event_writers: AllTaskStartedEventWriters,
) {
    let now = simulation_time.now();

    ships
        .iter_mut()
        .filter(|(_, _, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut queue, mut behavior, in_sector)| {
            let Some((target_sector, build_site)) =
                find_nearby_sector_with_build_site(&all_sectors, in_sector)
            else {
                behavior.next_idle_update =
                    now.add_seconds(constants::SECONDS_BETWEEN_SHIP_BEHAVIOR_IDLE_UPDATES);
                return;
            };

            if target_sector != in_sector.sector {
                let path = pathfinding::find_path(
                    &all_sectors,
                    &all_transforms,
                    in_sector.sector,
                    all_transforms.get(ship_entity).unwrap().translation,
                    target_sector,
                    Some(all_transforms.get(build_site.into()).unwrap().translation),
                )
                .unwrap();

                pathfinding::create_tasks_to_follow_path(&mut queue, path);
            }

            queue.push_back(TaskInsideQueue::MoveToEntity {
                target: TypedEntity::ConstructionSite(build_site),
                stop_at_target: true,
                distance_to_target: constants::DOCKING_DISTANCE_TO_STATION,
            });

            queue.push_back(TaskInsideQueue::Construct { target: build_site });

            queue.apply(
                &mut commands,
                now,
                ship_entity,
                &mut all_task_started_event_writers,
            );
        })
}

#[must_use]
fn find_nearby_sector_with_build_site(
    all_sectors: &Query<&SectorComponent>,
    in_sector: &InSector,
) -> Option<(SectorEntity, ConstructionSiteEntity)> {
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

    Some((target_sector.sector, *target_build_site))
}
