use bevy::prelude::{StateSet, States, SubStates};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MouseCursorOverUiState {
    #[default]
    NotOverUI,
    OverUI,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ApplicationState {
    #[allow(dead_code)]
    Menu,
    #[default]
    LoadingUniverse,
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
