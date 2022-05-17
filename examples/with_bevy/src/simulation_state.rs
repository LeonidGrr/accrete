use crate::active_event::{ActiveEvent, ActiveEventStatus};
use accrete::events::AccreteEvent;
use bevy::prelude::*;

#[derive(Debug)]
pub struct SimulationState {
    pub event_idx: usize,
    pub simulation_speed: f32,
    pub current_step: f32,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState {
            event_idx: 0,
            simulation_speed: 5.0,
            current_step: 0.0,
        }
    }

    pub fn is_open(&self, active_event: &Res<ActiveEvent>, total_events: usize) -> bool {
        let SimulationState { event_idx, .. } = *self;
        event_idx < total_events - 1 && active_event.status == ActiveEventStatus::Done
    }
}

fn simulation_step_system(time: Res<Time>, mut state: ResMut<SimulationState>) {
    state.current_step += state.simulation_speed * time.delta_seconds();
}

fn event_handler_system(
    mut commands: Commands,
    active_event: Res<ActiveEvent>,
    log: Res<Vec<AccreteEvent>>,
    mut state: ResMut<SimulationState>,
) {
    let current_event = &log[state.event_idx];
    let event_lock = matches!(current_event, AccreteEvent::PlanetarySystemComplete(..));

    if !event_lock && state.is_open(&active_event, log.len()) {
        commands.insert_resource(ActiveEvent::from(current_event));
        state.event_idx += 1;
    }
}

pub struct SimulationStatePlugin;

impl Plugin for SimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationState::new())
            .add_system(simulation_step_system)
            .add_system(event_handler_system);
    }
}
