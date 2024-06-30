mod plugin;
mod simulation_time_resource;
mod simulation_timestamp;

pub use plugin::SimulationTimePlugin;
pub use simulation_time_resource::SimulationTime;
pub use simulation_timestamp::CurrentSimulationTimestamp;
pub use simulation_timestamp::SimulationTimestamp;

pub type Milliseconds = u64;

const MILLIS_PER_SECOND: u64 = 1000;
