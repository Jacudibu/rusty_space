use bevy::math::Vec2;
use bevy::prelude::Resource;
use std::time::Duration;

#[derive(Resource)]
pub struct MouseInteraction {
    pub origin: Vec2,
    pub current: Vec2,
    pub start: Duration,
    total_travel: f32,
}

impl MouseInteraction {
    pub fn new(position: Vec2, start: Duration) -> Self {
        Self {
            origin: position,
            current: position,
            start,
            total_travel: 0.0,
        }
    }

    pub fn update(&mut self, new_pos: Vec2) {
        self.total_travel += self.current.distance(new_pos);
        self.current = new_pos;
    }

    pub fn counts_as_click(&self, current_time: Duration) -> bool {
        // The average mouse click lasts about 85 milliseconds
        (current_time - self.start).as_millis() < 100
    }

    pub fn counts_as_drag(&self) -> bool {
        // TODO: Should make this depend on zoom level
        // Using logical positions might also be an option, but that would exclude camera movement through wasd
        self.total_travel > 1.0
    }
}

#[derive(Resource)]
pub struct LastMouseInteraction {
    counts_as_click: bool,
}

impl LastMouseInteraction {
    pub fn from(value: &MouseInteraction, click_end: Duration) -> Self {
        Self {
            counts_as_click: value.counts_as_click(click_end),
        }
    }
}
