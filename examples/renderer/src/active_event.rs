use accrete::events::*;
use bevy::prelude::*;
use crate::planet_model::{PlanetId, PlanetBarycenter, Orbit};
use crate::simulation_state::SimulationState;
use crate::consts::SCALE_FACTOR;

#[derive(Debug, PartialEq, Eq)]
pub enum ActiveEventStatus {
    None,
    Created,
    Approaching,
    Executing,
    Done,
}

#[derive(Debug, Component)]
pub struct ActiveEvent {
    pub event: Option<AccreteEvent>,
    pub status: ActiveEventStatus,
}

impl Default for ActiveEvent {
    fn default() -> Self {
        ActiveEvent {
            event: None,
            status: ActiveEventStatus::None,
        }
    }
}

impl From<&AccreteEvent> for ActiveEvent {
    fn from(accrete_event: &AccreteEvent) -> Self {
        ActiveEvent {
            event: Some(accrete_event.clone()),
            status: ActiveEventStatus::Created,
        }
    }
}

impl ActiveEvent {
    fn created(
        &self,
        mut query: Query<(&PlanetId, &mut PlanetBarycenter, &mut Orbit)>,
    ) {
        if let Some(event) = &self.event {
            match event {
                AccreteEvent::PlanetesimalsCoalesced(_, target_id, source_id, resulting_planet) => {
                    for (planet_id, _, mut orbit) in query.iter_mut() {
                        if target_id == &planet_id.0 || source_id == &planet_id.0 {
                            orbit.target_a = Some(resulting_planet.a as f32 * SCALE_FACTOR);
                        }
                    }
                },
                AccreteEvent::PlanetesimalCaptureMoon(_, target_id, source_id, resulting_planet) => {
                    for (planet_id, mut barycenter, mut orbit) in query.iter_mut() {
                        if target_id == &planet_id.0 {
                            orbit.target_a = Some(resulting_planet.a as f32 * SCALE_FACTOR);
                        }
                        if source_id == &planet_id.0 {
                            let resulting_moon = resulting_planet.moons.iter().find(|m| &m.id == source_id);
                            if let Some(resulting_moon) = resulting_moon {
                                orbit.target_a = Some(resulting_moon.a as f32 * SCALE_FACTOR);
                                barycenter.id = Some(target_id.to_owned());
                            }
                        }
                    }
                },
                _ => (),
            }
        }
    }
}

pub fn active_event_system(
    mut state: ResMut<SimulationState>,
    mut query: Query<(&PlanetId, &mut PlanetBarycenter, &mut Orbit)>,
) {
    match &state.active_event.status {
        ActiveEventStatus::Created => {
            state.active_event.created(query);
            state.active_event.status = ActiveEventStatus::Approaching;
        },
        ActiveEventStatus::Approaching => (),
        ActiveEventStatus::Executing => (),
        ActiveEventStatus::Done => (),
        ActiveEventStatus::None => (),
    }
}
