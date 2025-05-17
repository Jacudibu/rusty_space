use crate::CLICK_TIME;
use bevy::math::Vec2;
use bevy::prelude::Resource;
use std::time::Duration;

#[derive(Resource)]
pub(crate) struct MouseInteraction {
    pub(crate) origin: Vec2,
    pub(crate) current: Vec2,
    pub(crate) start: Duration,
    total_travel: f32,
}

impl MouseInteraction {
    pub(crate) fn new(position: Vec2, start: Duration) -> Self {
        Self {
            origin: position,
            current: position,
            start,
            total_travel: 0.0,
        }
    }

    pub(crate) fn update(&mut self, new_pos: Vec2) {
        self.total_travel += self.current.distance(new_pos);
        self.current = new_pos;
    }

    pub(crate) fn counts_as_click(&self, current_time: Duration) -> bool {
        (current_time - self.start).as_millis() < CLICK_TIME
    }

    pub(crate) fn counts_as_drag(&self) -> bool {
        // TODO: Should make this depend on zoom level
        // Using logical positions might also be an option, but that would exclude camera movement through wasd
        self.total_travel > 1.0
    }
}

#[derive(Resource)]
pub(crate) struct LastMouseInteraction {
    pub(crate) counts_as_click: bool,
    pub(crate) click_end: Duration,
}

impl LastMouseInteraction {
    pub(crate) fn from(value: &MouseInteraction, click_end: Duration) -> Self {
        Self {
            counts_as_click: value.counts_as_click(click_end),
            click_end,
        }
    }
}
