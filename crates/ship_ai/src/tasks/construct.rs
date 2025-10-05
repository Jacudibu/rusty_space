use crate::task_lifecycle_traits::task_cancellation_active::TaskCancellationForActiveTaskEventHandler;
use crate::task_lifecycle_traits::task_cancellation_in_queue::TaskCancellationForTaskInQueueEventHandler;
use crate::task_lifecycle_traits::task_completed::TaskCompletedEventHandler;
use crate::task_lifecycle_traits::task_creation::{
    GeneralPathfindingArgs, TaskCreationEventHandler,
};
use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
use crate::task_lifecycle_traits::task_update_runner::TaskUpdateRunner;
use crate::task_metadata;
use crate::task_metadata::TaskMetaData;
use crate::utility::ship_task::ShipTask;
use crate::utility::task_preconditions::create_preconditions_and_move_to_entity;
use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::math::Vec2;
use bevy::prelude::{BevyError, MessageWriter, Query, Res, error};
use common::components::task_kind::TaskKind;
use common::components::task_queue::TaskQueue;
use common::components::{ConstructionSite, Ship};
use common::constants::BevyResult;
use common::events::task_events::{
    InsertTaskIntoQueueCommand, TaskCanceledWhileActiveEvent, TaskCompletedEvent, TaskStartedEvent,
};
use common::session_data::ShipConfigurationManifest;
use common::simulation_transform::SimulationTransform;
use common::types::entity_wrappers::ShipEntity;
use common::types::ship_tasks::Construct;
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

/// Registers a ship as an active worker for the provided [ConstructionSite].
pub fn register_ship(site: &mut ConstructionSite, entity: ShipEntity, build_power: u32) {
    site.total_build_power_of_ships += build_power;
    if let Some(old_value) = site.construction_ships.insert(entity, build_power) {
        site.total_build_power_of_ships -= old_value;
    }
}

/// Removes a ship registration from the provided [ConstructionSite]
pub fn deregister_ship(site: &mut ConstructionSite, entity: ShipEntity) {
    if let Some(build_power) = site.construction_ships.remove(&entity) {
        site.total_build_power_of_ships -= build_power;
    }
}

impl<'w, 's> TaskUpdateRunner<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = ();

    fn run_all_tasks(
        _args: StaticSystemParam<Self::Args>,
        _args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> Result<Arc<Mutex<Vec<TaskCompletedEvent<Self>>>>, BevyError> {
        panic!("This should never be called")
    }

    fn update(
        _event_writer: MessageWriter<TaskCompletedEvent<Self>>,
        _args: StaticSystemParam<Self::Args>,
        _args_mut: StaticSystemParam<Self::ArgsMut>,
    ) -> BevyResult {
        Ok(())
    }
}

impl<'w, 's> TaskCreationEventHandler<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = ();

    fn create_tasks_for_command(
        event: &InsertTaskIntoQueueCommand<Construct>,
        task_queue: &TaskQueue,
        general_pathfinding_args: &GeneralPathfindingArgs,
        _args: &StaticSystemParam<Self::Args>,
        _args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<VecDeque<TaskKind>, BevyError> {
        // let args = args.deref_mut();

        let mut new_tasks = create_preconditions_and_move_to_entity(
            event.entity,
            event.task_data.target.into(),
            task_queue,
            general_pathfinding_args,
        )?;

        new_tasks.push_back(TaskKind::Construct {
            data: event.task_data.clone(),
        });

        Ok(new_tasks)
    }
}

#[derive(SystemParam)]
pub struct TaskStartedArgsMut<'w, 's> {
    construction_tasks: Query<'w, 's, (&'static ShipTask<Construct>, &'static Ship)>,
    construction_sites: Query<'w, 's, &'static mut ConstructionSite>,
    ship_configurations: Res<'w, ShipConfigurationManifest>,
}

impl<'w, 's> TaskStartedEventHandler<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = TaskStartedArgsMut<'w, 's>;

    fn on_task_started(
        event: &TaskStartedEvent<Self>,
        _args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();

        let (task, ship) = args_mut.construction_tasks.get(event.entity.into())?;
        let mut construction_site = args_mut.construction_sites.get_mut(task.target.into())?;

        let Some(ship_config) = args_mut.ship_configurations.get_by_id(&ship.config_id()) else {
            error!(
                "Attempted to start construction task on ship without existing configuration: {:?}",
                event.entity
            );
            return Ok(());
        };

        let Some(build_power) = ship_config.computed_stats.build_power else {
            error!(
                "Attempted to start construction task on ship without build power: {:?}",
                event.entity
            );
            return Ok(());
        };

        register_ship(&mut construction_site, event.entity, build_power);
        Ok(())
    }
}
impl<'w, 's> TaskCancellationForTaskInQueueEventHandler<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = ();

    fn skip_cancelled_in_queue() -> bool {
        true
    }
}

#[derive(SystemParam)]
pub struct TaskCancellationForActiveTaskArgsMut<'w, 's> {
    construction_sites: Query<'w, 's, &'static mut ConstructionSite>,
}

impl<'w, 's> TaskCancellationForActiveTaskEventHandler<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = TaskCancellationForActiveTaskArgsMut<'w, 's>;

    fn can_task_be_cancelled_while_active() -> bool {
        true
    }

    fn on_task_cancellation_while_in_active(
        event: &TaskCanceledWhileActiveEvent<Self>,
        _args: &StaticSystemParam<Self::Args>,
        args_mut: &mut StaticSystemParam<Self::ArgsMut>,
    ) -> Result<(), BevyError> {
        let args_mut = args_mut.deref_mut();
        let mut site = args_mut
            .construction_sites
            .get_mut(event.task_data.target.into())?;

        deregister_ship(&mut site, event.entity);
        Ok(())
    }
}

impl<'w, 's> TaskCompletedEventHandler<'w, 's, Self> for Construct {
    type Args = ();
    type ArgsMut = ();

    fn skip_completed() -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use crate::task_lifecycle_traits::task_started::TaskStartedEventHandler;
    use crate::utility::ship_task::ShipTask;
    use bevy::prelude::{Entity, Update, With};
    use common::components::{ConstructionSite, Ship};
    use common::constants::BevyResult;
    use common::events::task_events::TaskStartedEvent;
    use common::game_data::{ConstructableModuleId, REFINED_METALS_PRODUCTION_MODULE_ID};
    use common::session_data::ShipConfigurationManifest;
    use common::session_data::ship_configs::MOCK_CONSTRUCTION_SHIP_CONFIG_ID;
    use common::types::local_hex_position::LocalHexPosition;
    use common::types::persistent_entity_id::PersistentFactionId;
    use common::types::ship_tasks::Construct;
    use hexx::Hex;
    use persistence::data::ShipBehaviorSaveData;
    use test_utils::test_app::TestApp;
    use universe_builder::sector_builder::SectorBuilder;
    use universe_builder::ship_builder::ShipBuilder;
    use universe_builder::station_builder::StationBuilder;

    #[test]
    fn starting_task_should_add_construction_power_to_site() -> BevyResult {
        let mut station_builder = StationBuilder::default();
        station_builder
            .add(
                LocalHexPosition::default(),
                "SomeStation",
                PersistentFactionId::next(),
            )
            .with_construction_site(
                vec![ConstructableModuleId::ProductionModule(
                    REFINED_METALS_PRODUCTION_MODULE_ID,
                )],
                0.0,
            );

        let mut ship_builder = ShipBuilder::default();
        ship_builder.add(
            MOCK_CONSTRUCTION_SHIP_CONFIG_ID,
            LocalHexPosition::default(),
            0.0,
            "Construction Ship",
            ShipBehaviorSaveData::HoldPosition,
            PersistentFactionId::next(),
        );

        let mut sector_builder = SectorBuilder::default();
        sector_builder.add(Hex::default());

        let mut app = TestApp::default()
            .with_sectors(sector_builder)
            .with_stations(station_builder)
            .with_ships(ship_builder)
            .build();

        app.add_message::<TaskStartedEvent<Construct>>();
        app.add_systems(Update, Construct::task_started_event_listener);
        app.finish();

        let construction_site = app
            .world_mut()
            .query_filtered::<Entity, With<ConstructionSite>>()
            .single(app.world())?;

        let ship = app
            .world_mut()
            .query_filtered::<Entity, With<Ship>>()
            .single(app.world())?;

        app.world_mut()
            .commands()
            .get_entity(ship)?
            .insert(ShipTask::new(Construct {
                target: construction_site.into(),
            }));

        app.world_mut()
            .send_event(TaskStartedEvent::<Construct>::new(ship.into()));

        app.update();

        let construction_site = app
            .world_mut()
            .query::<&ConstructionSite>()
            .single(app.world())?;
        let (registered_ship, build_power) =
            construction_site.construction_ships.iter().next().unwrap();
        let expected_build_power = app
            .world()
            .resource::<ShipConfigurationManifest>()
            .get_by_id(&MOCK_CONSTRUCTION_SHIP_CONFIG_ID)
            .unwrap()
            .computed_stats
            .build_power
            .unwrap();

        assert_eq!(Entity::from(registered_ship), ship);
        assert_eq!(expected_build_power, *build_power);

        Ok(())
    }
}

impl<'w, 's> TaskMetaData<'w, 's, Self> for Construct {
    fn task_target_position(&self, all_transforms: &Query<&SimulationTransform>) -> Option<Vec2> {
        task_metadata::get_entity_global_position(all_transforms, self.target.into())
    }
}
