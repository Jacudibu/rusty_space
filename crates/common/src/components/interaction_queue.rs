use crate::events::send_signal_event::SendSignalEvent;
use crate::types::entity_wrappers::ShipEntity;
use bevy::prelude::{Component, MessageWriter};
use std::collections::VecDeque;

/// An entity with an [InteractionQueue] only allow a set amount of other entities to interact with it at once.
/// Once that limit is reached, they'll be queued up and notified for their turn.  
#[derive(Component)]
pub struct InteractionQueue {
    maximum_simultaneous_interactions: u32,
    currently_interacting: u32,
    waiting_queue: VecDeque<ShipEntity>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum InteractionQueueResult {
    ProceedImmediately,
    EnteredQueuePleaseAddAwaitingSignalToQueue,
}

impl InteractionQueue {
    pub fn new(maximum_simultaneous_interactions: u32) -> Self {
        Self {
            maximum_simultaneous_interactions,
            currently_interacting: 0,
            waiting_queue: Default::default(),
        }
    }

    pub fn maximum_interactions(&self) -> u32 {
        self.maximum_simultaneous_interactions
    }

    pub fn currently_interacting(&self) -> u32 {
        self.currently_interacting
    }

    /// Attempts to start an interaction.
    ///
    /// # Returns
    /// - **Ok** - The entity may interact immediately.
    /// - **Err** - We are currently at capacity, the entity has been added to the queue and will receive a signal once it may proceed.
    pub fn try_start_interaction(&mut self, requester: ShipEntity) -> InteractionQueueResult {
        if self.currently_interacting < self.maximum_simultaneous_interactions {
            self.currently_interacting += 1;
            InteractionQueueResult::ProceedImmediately
        } else {
            self.waiting_queue.push_back(requester);
            InteractionQueueResult::EnteredQueuePleaseAddAwaitingSignalToQueue
        }
    }

    /// Notifies the next waiting entity within the queue, if there are any.
    ///
    /// Needs to be called whenever something stops interacting with the respective object!
    pub fn finish_interaction(&mut self, event_writer: &mut MessageWriter<SendSignalEvent>) {
        self.currently_interacting -= 1;
        if self.currently_interacting <= self.maximum_simultaneous_interactions {
            if let Some(next) = self.waiting_queue.pop_front() {
                self.currently_interacting += 1;
                event_writer.write(SendSignalEvent { entity: next });
            }
        }
    }

    /// Removes the provided entity from the queue.
    ///
    /// Needs to be called whenever an [AwaitingSignal] task gets cancelled.
    pub fn remove_from_queue(&mut self, entity: ShipEntity) {
        if let Some(position) = self.waiting_queue.iter().position(|x| x == &entity) {
            self.waiting_queue.remove(position);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::components::interaction_queue::{InteractionQueue, InteractionQueueResult};
    use bevy::prelude::{Component, Entity};

    use crate::types::entity_wrappers::typed_entity_wrapper::TypedEntityWrapper;

    // TODO: That's a duplicate from test_utils due to circular dependency shenanigans.
    //       Might need to move structs with logic somewhere else or move the logic elsewhere
    fn mock_entity_id<T: Component>(id: u32) -> TypedEntityWrapper<T> {
        Entity::from_raw_u32(id).unwrap().into()
    }

    #[test]
    fn interacting_below_capacity() {
        let mut queue = InteractionQueue::new(2);
        assert_eq!(
            InteractionQueueResult::ProceedImmediately,
            queue.try_start_interaction(mock_entity_id(1))
        );

        assert_eq!(1, queue.currently_interacting)
    }

    #[test]
    fn interacting_at_and_above_capacity() {
        let mut queue = InteractionQueue::new(2);
        queue.try_start_interaction(mock_entity_id(1));
        queue.try_start_interaction(mock_entity_id(2));

        assert_eq!(
            InteractionQueueResult::EnteredQueuePleaseAddAwaitingSignalToQueue,
            queue.try_start_interaction(mock_entity_id(3))
        );
        assert_eq!(
            InteractionQueueResult::EnteredQueuePleaseAddAwaitingSignalToQueue,
            queue.try_start_interaction(mock_entity_id(4))
        );

        assert_eq!(2, queue.currently_interacting);
        assert_eq!(2, queue.waiting_queue.len());
        assert_eq!(mock_entity_id(3), queue.waiting_queue[0]);
        assert_eq!(mock_entity_id(4), queue.waiting_queue[1]);
    }

    #[test]
    fn interacting_above_capacity_same_frame() {
        let mut queue = InteractionQueue::new(2);
        queue.try_start_interaction(mock_entity_id(1));
        queue.try_start_interaction(mock_entity_id(2));

        let first = mock_entity_id(1);
        let second = mock_entity_id(2);
        let third = mock_entity_id(3);

        assert_eq!(
            InteractionQueueResult::EnteredQueuePleaseAddAwaitingSignalToQueue,
            queue.try_start_interaction(first)
        );
        assert_eq!(
            InteractionQueueResult::EnteredQueuePleaseAddAwaitingSignalToQueue,
            queue.try_start_interaction(second)
        );
        assert_eq!(
            InteractionQueueResult::EnteredQueuePleaseAddAwaitingSignalToQueue,
            queue.try_start_interaction(third)
        );

        assert_eq!(2, queue.currently_interacting);
        assert_eq!(3, queue.waiting_queue.len());

        assert_eq!(first, queue.waiting_queue[0]);
        assert_eq!(second, queue.waiting_queue[1]);
        assert_eq!(third, queue.waiting_queue[2]);
    }
}
