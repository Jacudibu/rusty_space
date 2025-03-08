use crate::components::{InSector, Sector};
use crate::persistence::BuildSiteIdMap;
use crate::simulation::prelude::{
    SimulationTime, SimulationTimestamp, SimulationTransform, TaskInsideQueue, TaskQueue,
};
use crate::simulation::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use crate::utils::{BuildSiteEntity, SectorEntity, TypedEntity};
use crate::{constants, pathfinding};
use bevy::prelude::{Component, Entity, Query, Res};

#[derive(Component)]
pub struct AutoBuildBehavior {
    pub next_idle_update: SimulationTimestamp,
}

#[allow(clippy::too_many_arguments)]
pub fn handle_idle_ships(
    simulation_time: Res<SimulationTime>,
    mut ships: Query<(Entity, &mut TaskQueue, &mut AutoBuildBehavior, &InSector), ShipIsIdleFilter>,
    all_sectors: Query<&Sector>,
    all_transforms: Query<&SimulationTransform>,
    build_site_id_map: Res<BuildSiteIdMap>,
) {
    let now = simulation_time.now();

    ships
        .iter_mut()
        .filter(|(_, _, behavior, _)| now.has_passed(behavior.next_idle_update))
        .for_each(|(ship_entity, mut queue, mut behavior, in_sector)| {
            let Some((target_sector, build_site)) =
                find_nearby_sector_with_build_site(&all_sectors, in_sector, &build_site_id_map)
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
                target: TypedEntity::BuildSite(build_site),
                stop_at_target: true,
                distance_to_target: 0.0,
            });

            queue.push_back(TaskInsideQueue::Build { target: build_site })
        })
}

#[must_use]
fn find_nearby_sector_with_build_site(
    all_sectors: &Query<&Sector>,
    in_sector: &InSector,
    build_site_id_map: &BuildSiteIdMap,
) -> Option<(SectorEntity, BuildSiteEntity)> {
    let nearby_sectors_with_build_sites =
        pathfinding::surrounding_sector_search::surrounding_sector_search(
            all_sectors,
            in_sector.sector,
            1,
            u8::MAX, // TODO: Should be limited
            all_sectors,
            |x| x.build_sites.len() > 0,
        );

    if nearby_sectors_with_build_sites.len() == 0 {
        return None;
    }

    let target_sector = nearby_sectors_with_build_sites.iter().min()?;
    let target_build_site = all_sectors
        .get(target_sector.sector.into())
        .unwrap()
        .build_sites
        .iter()
        .next()
        .unwrap();

    Some((
        target_sector.sector,
        build_site_id_map
            .get_entity(&target_build_site.id)
            .unwrap()
            .clone(),
    ))
}
