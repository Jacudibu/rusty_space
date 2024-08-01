use crate::simulation::prelude::{
    AwaitingSignal, CurrentSimulationTimestamp, SimulationTimestamp, TaskFinishedEvent,
};
use crate::utils::ShipEntity;
use bevy::prelude::{Component, EventWriter};
use std::collections::BTreeMap;

#[derive(Component)]
pub struct InteractionQueue {
    pub maximum_simultaneous_interactions: u32,
    pub currently_interacting: u32,
    pub waiting_queue: BTreeMap<SimulationTimestamp, ShipEntity>,
}

impl InteractionQueue {
    pub fn new(maximum_simultaneous_interactions: u32) -> Self {
        Self {
            maximum_simultaneous_interactions,
            currently_interacting: 0,
            waiting_queue: Default::default(),
        }
    }

    /// Attempts to start an interaction.
    ///
    /// # Returns
    /// - **Ok** - The entity may interact immediately.
    /// - **Err** - We are currently at capacity, the entity has been added to the queue and will receive a signal once it may proceed.
    pub fn try_start_interaction(
        &mut self,
        now: &CurrentSimulationTimestamp,
        requester: ShipEntity,
    ) -> Result<(), ()> {
        if self.currently_interacting < self.maximum_simultaneous_interactions {
            self.currently_interacting += 1;
            Ok(())
        } else {
            self.insert_into_queue(now.into(), requester);
            Err(())
        }
    }

    fn insert_into_queue(&mut self, now: SimulationTimestamp, requester: ShipEntity) {
        if let Some(conflict) = self.waiting_queue.insert(now, requester) {
            self.insert_into_queue(now + 1, conflict);
        }
    }

    /// Notifies the next waiting entity within the queue, if there are any.
    ///
    /// Needs to be called whenever something stops interacting with the respective object!
    pub fn finish_interaction(
        &mut self,
        event_writer: &mut EventWriter<TaskFinishedEvent<AwaitingSignal>>,
    ) {
        self.currently_interacting -= 1;
        if self.currently_interacting <= self.maximum_simultaneous_interactions {
            if let Some((_, next)) = self.waiting_queue.pop_first() {
                self.currently_interacting += 1;
                event_writer.send(TaskFinishedEvent::new(next.into()));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::components::interaction_queue::InteractionQueue;
    use crate::simulation::prelude::SimulationTimestamp;
    use crate::utils::ShipEntity;

    #[test]
    fn interacting_below_capacity() {
        let mut queue = InteractionQueue::new(2);
        assert_eq!(Ok(()), queue.try_start_interaction(&1.into(), 1.into()));

        assert_eq!(1, queue.currently_interacting)
    }

    #[test]
    fn interacting_at_and_above_capacity() {
        let mut queue = InteractionQueue::new(2);
        queue.try_start_interaction(&1.into(), 1.into()).unwrap();
        queue.try_start_interaction(&2.into(), 2.into()).unwrap();

        assert_eq!(Err(()), queue.try_start_interaction(&3.into(), 3.into()));
        assert_eq!(Err(()), queue.try_start_interaction(&4.into(), 4.into()));

        assert_eq!(2, queue.currently_interacting);
        assert_eq!(2, queue.waiting_queue.len());
        assert_eq!(
            &SimulationTimestamp::from(3),
            queue.waiting_queue.first_entry().unwrap().key()
        );
        assert_eq!(
            &SimulationTimestamp::from(4),
            queue.waiting_queue.last_entry().unwrap().key()
        );
    }

    #[test]
    fn interacting_above_capacity_same_frame() {
        let mut queue = InteractionQueue::new(2);
        queue.try_start_interaction(&1.into(), 1.into()).unwrap();
        queue.try_start_interaction(&2.into(), 2.into()).unwrap();

        let first = ShipEntity::from(1);
        let second = ShipEntity::from(2);
        let third = ShipEntity::from(3);

        assert_eq!(Err(()), queue.try_start_interaction(&3.into(), first));
        assert_eq!(Err(()), queue.try_start_interaction(&3.into(), second));
        assert_eq!(Err(()), queue.try_start_interaction(&3.into(), third));

        assert_eq!(2, queue.currently_interacting);
        assert_eq!(3, queue.waiting_queue.len());

        // Order gets inverted, but we don't care about that
        assert_eq!(&third, queue.waiting_queue.get(&3.into()).unwrap());
        assert_eq!(&second, queue.waiting_queue.get(&4.into()).unwrap());
        assert_eq!(&first, queue.waiting_queue.get(&5.into()).unwrap());
    }
}
