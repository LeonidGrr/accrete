use accrete::Ring;
use bevy::prelude::*;

#[derive(Debug, Clone, Bundle)]
pub struct RingModel {
    pub ring_radius: RingRadius,
    pub radius: Radius,
    pub id: RingId,
}

impl From<&Ring> for RingModel {
    fn from(ring: &Ring) -> RingModel {
        RingModel {
            ring_radius: RingRadius(ring.width as f32 / 2.0),
            radius: Radius(ring.a as f32),
            id: RingId(ring.id.clone()),
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct RingId(pub String);

#[derive(Debug, Clone, Component)]
pub struct RingRadius(pub f32);

#[derive(Debug, Clone, Component)]
pub struct Radius(pub f32);
