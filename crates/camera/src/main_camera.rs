use crate::panning::SmoothPanning;
use crate::zooming::SmoothZooming;
use bevy::prelude::Component;

/// Marker component for the Main Camera. It's the camera that renders onto the whole screen the whole time.
#[derive(Component, Default)]
#[component(immutable)]
#[require(SmoothZooming, SmoothPanning)]
pub struct MainCamera;
