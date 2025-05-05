use crate::ship_ai::ship_is_idle_filter::ShipIsIdleFilter;
use bevy::prelude::{Query, Res, Time, Without};
use common::components::ship_velocity::ShipVelocity;
use common::components::{Engine, IsDocked};

pub fn stop_idle_ships(
    mut idle_ships: Query<(&Engine, &mut ShipVelocity), (Without<IsDocked>, ShipIsIdleFilter)>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_secs();

    idle_ships
        .par_iter_mut()
        .for_each(|(engine, mut velocity)| {
            if velocity.forward > 0.0 {
                // TODO: Contemplate adding a marker component to unstopped idle ships so we can filter through the query
                //       Might be best to just always add some kind of "NeedsToBeStoppedWhenIdle" marker whenever a behavior assigns new tasks to a ship?
                velocity.decelerate(engine, delta_seconds);
            }
        })
}
