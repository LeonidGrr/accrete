use bevy::prelude::*;

#[derive(Debug, Clone, Bundle)]
pub struct RingModel {
    pub width: Width,
    pub radius: Radius,
}

#[derive(Debug, Clone, Component)]
pub struct Width(pub f32);

#[derive(Debug, Clone, Component)]
pub struct Radius(pub f32);
