use bevy::app::{App, Plugin};
use bevy::prelude::{AppExtStates, StateSet, States, SubStates};

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ApplicationState>();
        app.add_sub_state::<SimulationState>();
        app.add_sub_state::<MenuState>();
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ApplicationState {
    #[allow(dead_code)]
    Menu,
    #[default]
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(ApplicationState = ApplicationState::InGame)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(ApplicationState = ApplicationState::Menu)]
pub enum MenuState {
    #[default]
    MainMenu,
}
