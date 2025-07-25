use crate::game_data::ItemId;
use crate::simulation_time::SimulationTimestamp;
use crate::types::entity_wrappers::{
    AsteroidEntity, CelestialEntity, ConstructionSiteEntity, GateEntity, SectorEntity, TypedEntity,
};
use crate::types::exchange_ware_data::ExchangeWareData;
use crate::types::gate_traversal_state::GateTraversalState;
use crate::types::sector_position::SectorPosition;
use bevy::math::Vec2;
use std::fmt::Debug;

/// Marker trait to define that a struct may be used as a ShipTask during simulation.
pub trait ShipTaskData: Clone + Debug + Send + Sync + 'static {}

// mod signal_kind {
//     struct Dock;
//     struct Undock;
//     struct HarvestGas;
// }

/// A ship with this task will be idle until it receives a signal through an event.
#[derive(Clone, Debug)]
pub struct AwaitingSignal {
    pub from: TypedEntity,
}
impl ShipTaskData for AwaitingSignal {}

/// Ships with this task are actively working on a construction site.
#[derive(Clone, Debug)]
pub struct Construct {
    pub target: ConstructionSiteEntity,
}
impl ShipTaskData for Construct {}

/// Ships with this task are currently docking at the specified target entity.
/// They'll move into the target and scale out of existence, after which this task will be completed.
#[derive(Clone, Debug)]
pub struct DockAtEntity {
    /// The Entity this ship is currently docking at.
    pub target: TypedEntity,
}
impl ShipTaskData for DockAtEntity {}

/// Ships with this task are currently trading wares with the specified target entity.
/// (They basically just wait until a timer runs out and then transfer the items)
#[derive(Clone, Debug)]
pub struct ExchangeWares {
    /// The [SimulationTimestamp] at which this transaction is supposed to finish.
    pub finishes_at: SimulationTimestamp,

    /// The entity representing our trading partner.
    pub target: TypedEntity,

    /// Further information on which wares are going to be exchanged.
    pub exchange_data: ExchangeWareData,
}
impl ShipTaskData for ExchangeWares {}
impl ExchangeWares {
    pub fn new(target: TypedEntity, exchange_data: ExchangeWareData) -> Self {
        Self {
            finishes_at: SimulationTimestamp::MAX,
            target,
            exchange_data,
        }
    }
}

/// Ships with this task are currently harvesting gas from a gas giant.
#[derive(Clone, Debug)]
pub struct HarvestGas {
    /// The entity of the gas giant from which we are harvesting.
    pub target: CelestialEntity,

    /// The gas which we are collecting
    pub gas: ItemId,

    /// A [SimulationTimestamp] to denote when the next inventory update occurs.
    /// Will be initialized in the OnTaskStarted event.
    pub next_update: Option<SimulationTimestamp>,
}
impl ShipTaskData for HarvestGas {}
impl HarvestGas {
    pub fn new(target: CelestialEntity, gas: ItemId) -> Self {
        Self {
            target,
            gas,
            next_update: None,
        }
    }
}

/// Ships with this task are currently mining ore from an asteroid.
#[derive(Clone, Debug)]
pub struct MineAsteroid {
    /// The Asteroid which we are mining
    pub target: AsteroidEntity,

    /// A [SimulationTimestamp] denoting when our next item transfer with the asteroid is scheduled to happen.
    /// Will be initialized in the OnTaskStarted event.
    pub next_update: Option<SimulationTimestamp>,
}
impl ShipTaskData for MineAsteroid {}
impl MineAsteroid {
    pub fn new(target: AsteroidEntity) -> Self {
        Self {
            target,
            next_update: None,
        }
    }
}

/// Ships with this task are currently moving towards another entity.
#[derive(Clone, Debug)]
pub struct MoveToEntity {
    /// The entity to which we are moving.
    pub target: TypedEntity,

    /// Whether the ship should slow down as it reaches the target, or just zoom past it.
    pub stop_at_target: bool,

    /// In case that we stop at the target, how far from it would be the perfect distance to do so?
    /// 0 would be right on top.
    pub desired_distance_to_target: f32,
}
impl ShipTaskData for MoveToEntity {}

/// Ships with this task are currently moving towards a specific position.
#[derive(Clone, Debug)]
pub struct MoveToPosition {
    /// The position to which we are moving.
    pub sector_position: SectorPosition,
    /// The global position represented by [sector_position].
    /// TODO: This stinks. SectorPosition should be enough without causing overhead.
    ///       SimulationTransform should support local space, otherwise bigger maps might cause floating point precision issues
    pub global_position: Vec2,
}
impl ShipTaskData for MoveToPosition {}

/// Ships with this task are currently moving towards a specific sector.
#[derive(Clone, Debug)]
pub struct MoveToSector {
    /// The position to which we are moving.
    pub sector: SectorEntity,
}
impl ShipTaskData for MoveToSector {}

/// Intermediate task to reserve a spot inside an [`InteractionQueue`] attached to the [`target`].
///
/// Will always be immediately completed on execution, with two possible results depending on the queue's state:
///  - free: proceeding with the next task in this entity's local [`TaskQueue`]
///  - busy: spawning an [`AwaitingSignal`] Task
#[derive(Clone, Debug)]
pub struct RequestAccess {
    /// The entity we want to access. Should have an [InteractionQueue].
    pub target: TypedEntity,
    /// What do we want to accomplish?
    pub goal: RequestAccessGoal,
}
impl ShipTaskData for RequestAccess {}

/// The kind of Access we want to [RequestAccess] for.
#[derive(Clone, Debug)]
pub enum RequestAccessGoal {
    Docking,
    Undocking,
    PlanetOrbit,
}

/// Ships with this are currently undocking from another entity.
/// They'll move in a straight line away from said entity whilst scaling into existence, after which this task completes.
/// This task cannot be canceled.
#[derive(Clone, Debug)]
pub struct Undock {
    /// The entity we are undocking from.
    pub from: TypedEntity,
    /// The position from which we are undocking. Will be set once the task has been started.
    pub start_position: Option<Vec2>,
}
impl ShipTaskData for Undock {}

/// Ships with this task are currently using a [Gate].
/// This task cannot be canceled.
#[derive(Clone, Debug)]
pub struct UseGate {
    /// How far along the line connecting the two gates we are.
    pub progress: f32,

    /// The current state of our little journey.
    pub traversal_state: GateTraversalState,

    /// The entity of the Gate we entered
    pub enter_gate: GateEntity,

    /// The sector we are about to enter when finishing this task.
    pub exit_sector: SectorEntity,
}
impl ShipTaskData for UseGate {}

impl UseGate {
    pub fn new(enter_gate: GateEntity, exit_sector: SectorEntity) -> Self {
        Self {
            enter_gate,
            exit_sector,
            progress: 0.0,
            traversal_state: GateTraversalState::default(),
        }
    }
}
