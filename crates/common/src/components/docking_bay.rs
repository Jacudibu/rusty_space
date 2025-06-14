use crate::events::task_events::TaskCompletedEvent;
use crate::types::entity_wrappers::ShipEntity;
use crate::types::ship_tasks::AwaitingSignal;
use bevy::prelude::{Component, EventWriter};
use std::collections::{HashSet, VecDeque};

/// An entity with a [DockingBay] allows ships to dock at it.
///
/// # Remarks
/// Ships may have [DockingBay]s, but make sure that they only accept smaller ships
/// ...unless you want to create a black hole of sorts.
/// TODO: Ships docked in ships which are docked cause some weirdness when undocking.
#[derive(Component)]
pub struct DockingBay {
    /// A queue for docking.
    pub dock_queue: VecDeque<ShipEntity>,
    /// A queue for undocking. Undocking has a higher priority than docking in order to make room.
    pub undock_queue: VecDeque<ShipEntity>,
    /// Only this many ships may dock/undock at once
    pub simultaneous_interaction_capacity: u32,
    /// How many ships may be docked at once.
    pub capacity: u32,
    /// All ships which are currently docked
    pub occupied_active_slots: HashSet<ShipEntity>,
    /// The ships which are currently docked here.
    pub docked: HashSet<ShipEntity>,
}

impl DockingBay {
    pub fn new(capacity: u32, simultaneous_interaction_capacity: u32) -> Self {
        Self {
            capacity,
            simultaneous_interaction_capacity,
            dock_queue: Default::default(),
            undock_queue: Default::default(),
            occupied_active_slots: Default::default(),
            docked: Default::default(),
        }
    }

    #[inline]
    pub fn can_support_more_simultaneous_interactions(&self) -> bool {
        self.simultaneous_interaction_capacity > self.occupied_active_slots.len() as u32
    }

    pub fn has_capacity_for_more_ships(&self) -> bool {
        self.capacity > self.docked.len() as u32
    }

    /// Attempts to start docking.
    ///
    /// # Returns
    /// - **Ok** - The entity may interact immediately.
    /// - **Err** - We are currently at capacity, the entity has been added to the queue and will receive a signal once it may proceed.
    pub fn try_dock(&mut self, requester: ShipEntity) -> Result<(), ()> {
        if self.can_support_more_simultaneous_interactions() && self.has_capacity_for_more_ships() {
            self.occupied_active_slots.insert(requester);
            Ok(())
        } else {
            self.dock_queue.push_back(requester);
            Err(())
        }
    }

    /// Attempts to start undocking.
    ///
    /// # Returns
    /// - **Ok** - The entity may interact immediately.
    /// - **Err** - We are currently at capacity, the entity has been added to the queue and will receive a signal once it may proceed.
    pub fn try_undock(&mut self, requester: ShipEntity) -> Result<(), ()> {
        if self.can_support_more_simultaneous_interactions() {
            self.occupied_active_slots.insert(requester);
            Ok(())
        } else {
            self.undock_queue.push_back(requester);
            Err(())
        }
    }

    /// Adds the ship to this docking bay and frees up the interaction slot.
    /// Also notifies the next waiting entity within the queue, if there are any.
    pub fn finish_docking(
        &mut self,
        ship: ShipEntity,
        event_writer: &mut EventWriter<TaskCompletedEvent<AwaitingSignal>>,
    ) {
        self.occupied_active_slots.remove(&ship);
        self.docked.insert(ship);
        if self.can_support_more_simultaneous_interactions() {
            if let Some(next) = self.undock_queue.pop_front() {
                self.occupied_active_slots.insert(next);
                event_writer.write(TaskCompletedEvent::new(next));
            }

            if self.has_capacity_for_more_ships() {
                if let Some(next) = self.dock_queue.pop_front() {
                    self.occupied_active_slots.insert(next);
                    event_writer.write(TaskCompletedEvent::new(next));
                }
            }
        }
    }

    /// Removes this ship from the list of docked ships.
    pub fn start_undocking(&mut self, entity: ShipEntity) {
        self.docked.remove(&entity);
    }

    /// Frees up the interaction slot, then notifies the next waiting entity within the queue, if there are any.
    pub fn finish_undocking(
        &mut self,
        ship: ShipEntity,
        event_writer: &mut EventWriter<TaskCompletedEvent<AwaitingSignal>>,
    ) {
        self.occupied_active_slots.remove(&ship);
        if self.can_support_more_simultaneous_interactions() {
            if let Some(next) = self.undock_queue.pop_front() {
                self.occupied_active_slots.insert(next);
                event_writer.write(TaskCompletedEvent::new(next));
            }

            if self.has_capacity_for_more_ships() {
                if let Some(next) = self.dock_queue.pop_front() {
                    self.occupied_active_slots.insert(next);
                    event_writer.write(TaskCompletedEvent::new(next));
                }
            }
        }
    }

    /// Removes the provided entity from the queue.
    ///
    /// Needs to be called whenever an [AwaitingSignal] task gets cancelled.
    pub fn remove_from_docking_queue(&mut self, entity: ShipEntity) {
        Self::remove_from_queue(&mut self.dock_queue, entity);
    }

    /// Removes the provided entity from the queue.
    ///
    /// Needs to be called whenever an [AwaitingSignal] task gets cancelled.
    pub fn remove_from_undocking_queue(&mut self, entity: ShipEntity) {
        Self::remove_from_queue(&mut self.undock_queue, entity);
    }

    fn remove_from_queue(queue: &mut VecDeque<ShipEntity>, entity: ShipEntity) {
        if let Some(position) = queue.iter().position(|x| x == &entity) {
            queue.remove(position);
        }
    }
}
