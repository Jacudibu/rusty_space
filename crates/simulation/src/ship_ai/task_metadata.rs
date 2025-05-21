use common::simulation_time::SimulationTimestamp;

/// MetaData for every active ShipTask. Can be used for diagnostics and other stuff.
pub struct ActiveTaskMetaData {
    /// The [SimulationTimestamp] when this task was started.
    pub started: SimulationTimestamp,
}
