use accrete::events::AccreteEvent;
use bevy::prelude::*;

use crate::{planet_model::PlanetId, simulation_state::SimulationState};

#[derive(Debug, PartialEq, Eq)]
pub enum AccreteEventStatus {
    None,
    Created,
    Approaching,
    Executing,
    Done,
}

#[derive(Debug, Component)]
pub struct ActiveEvent {
    pub event: Option<AccreteEvent>,
    pub status: AccreteEventStatus,
}

impl Default for ActiveEvent {
    fn default() -> Self {
        ActiveEvent {
            event: None,
            status: AccreteEventStatus::None,
        }
    }
}

impl From<&AccreteEvent> for ActiveEvent {
    fn from(accrete_event: &AccreteEvent) -> Self {
        ActiveEvent {
            event: Some(accrete_event.clone()),
            status: AccreteEventStatus::Created,
        }
    }
}

pub fn active_event_system(
    mut commands: Commands,
    time: Res<Time>,
    log: Res<Vec<AccreteEvent>>,
    mut state: ResMut<SimulationState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&PlanetId, &Handle<Mesh>)>,
) {
}
