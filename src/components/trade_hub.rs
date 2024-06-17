use bevy::prelude::Component;

#[derive(Component)]
pub struct TradeHub {
    // TODO this needs to be a mit more complicated than that. :D
    pub buying: bool,
    pub selling: bool,
}
