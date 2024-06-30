mod plugin;
mod simulation_time_resource;
mod simulation_timestamp;

pub use plugin::SimulationTimePlugin;
pub use simulation_time_resource::SimulationTime;

pub type Milliseconds = u64;
