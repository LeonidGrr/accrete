use crate::active_event::{active_event_system, ActiveEvent, ActiveEventStatus};
use crate::planet_model::{Orbit, PlanetId, PlanetPosition};
use accrete::{events::AccreteEvent, Planetesimal};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SimulationState {
    pub event_idx: usize,
    pub planets: HashMap<String, Planetesimal>,
    pub cached_planets: Option<(Entity, Entity)>,
    pub simulation_speed: f32,
    pub current_step: f32,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState {
            event_idx: 0,
            planets: HashMap::new(),
            cached_planets: None,
            simulation_speed: 50.0,
            current_step: 0.0,
        }
    }

    pub fn is_open(
        &self,
        active_event: &Res<ActiveEvent>,
        total_events: usize,
    ) -> bool {
        let SimulationState { event_idx, .. } = *self;
        event_idx < total_events - 1
            && active_event.status == ActiveEventStatus::Done
    }

    pub fn cache_planets(
        &mut self,
        query: &mut Query<(
            Entity,
            &PlanetId,
            &mut PlanetPosition,
            &mut Orbit,
            &Handle<Mesh>,
            &mut Visibility,
        )>,
        source_id: &str,
        target_id: &str,
    ) {
        if self.cached_planets.is_none() {
            let mut iter = query.iter_combinations_mut();
            while let Some([(entity1, id1, _, _, _, _), (entity2, id2, _, _, _, _)]) = iter.fetch_next() {
                let moon_and_planet = match (&id1.0, &id2.0) {
                    (id1, id2) if id1 == source_id && id2 == target_id => Some((entity1, entity2)),
                    (id1, id2) if id2 == source_id && id1 == target_id => Some((entity2, entity1)),
                    _ => None,
                };
                if moon_and_planet.is_some() {
                    self.cached_planets = moon_and_planet;
                }
            }
        }
    }

    pub fn clear_cahed_planets(&mut self) {
        self.cached_planets = None;
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
    let event_lock = matches!(current_event, AccreteEvent::PostAccretionStarted(_));

    if !event_lock && state.is_open(&active_event, log.len()) {
        commands.insert_resource(ActiveEvent::from(current_event));
        state.event_idx += 1;
    }
}

pub struct SimulationStatePlugin;

impl Plugin for SimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SimulationState::new())
            .insert_resource(ActiveEvent::default())
            .add_system(simulation_step_system)
            .add_system(event_handler_system)
            .add_system(active_event_system);
    }
}
