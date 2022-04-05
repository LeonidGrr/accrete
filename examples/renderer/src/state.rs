use crate::coalescence::CoalescenceOption;
use crate::moon_capture::MoonCaptureOption;
use crate::planet_model::PlanetModel;
use accrete::events::AccreteEvent;
use accrete::DustBand;
use macroquad::prelude::*;
use std::collections::HashMap;

pub type PlanetModels = HashMap<String, PlanetModel>;

pub struct State {
    pub planet_models: PlanetModels,
    pub coalescence: CoalescenceOption,
    pub moon_capture: MoonCaptureOption,
    pub dust_model: HashMap<String, DustBand>,
    pub event_idx: usize,
    pub event_lock: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            dt: 1.0,
            event_idx: 0,
            planet_models: HashMap::new(),
            coalescence: CoalescenceOption::none(),
            moon_capture: MoonCaptureOption::none(),
            dust_model: HashMap::new(),
            event_lock: false,
        }
    }

    pub fn event_handler(&mut self, current_event: &AccreteEvent, time: f64) {
        match current_event {
            // AccreteEvent::PlanetarySystemSetup(_, _) => (),
            AccreteEvent::PlanetesimalCreated(_, planet) => {
                let new_planet_model = PlanetModel::new(planet.clone(), time);
                self.planet_models
                    .insert(planet.id.clone(), new_planet_model);
            }
            AccreteEvent::PlanetesimalUpdated(_, planet) => {
                if let Some(current_planet_model) = self.planet_models.get(&planet.id) {
                    let mut next_planet_model = PlanetModel::new(planet.clone(), time);
                    next_planet_model.position = current_planet_model.position;
                    self.planet_models
                        .insert(planet.id.clone(), next_planet_model);
                }
            }
            AccreteEvent::PlanetesimalToGasGiant(_, gas_giant) => {
                if let Some(current_planet_model) = self.planet_models.get(&gas_giant.id) {
                    let mut next_planet_model = PlanetModel::new(gas_giant.clone(), time);
                    next_planet_model.position = current_planet_model.position;
                    self.planet_models
                        .insert(gas_giant.id.clone(), next_planet_model);
                }
            }
            // AccreteEvent::DustBandsUpdated(_, _) => (),
            AccreteEvent::PlanetesimalsCoalesced(_, source_planet_id, target_planet_id, result) => {
                self.coalescence = CoalescenceOption::new(
                    source_planet_id,
                    target_planet_id,
                    result.clone(),
                    time,
                );
            }
            // AccreteEvent::MoonsCoalesced(_, source_moon_id, target_moon_id, result) => {},
            AccreteEvent::PlanetesimalCaptureMoon(_, planet_id, moon_id, result) => {
                self.moon_capture =
                    MoonCaptureOption::new(planet_id, moon_id, result.clone(), time);
            }
            // AccreteEvent::PlanetesimalMoonToRing(name, _) => name,
            AccreteEvent::PostAccretionStarted(_) => self.event_lock = true,
            // AccreteEvent::OuterBodyInjected(name, _) => name,
            // AccreteEvent::PlanetaryEnvironmentGenerated(name, _) => name,
            // AccreteEvent::PlanetarySystemComplete(_, _) => (),
            _ => (),
        }
    }

    pub fn update_planets(&mut self, time: f64) {
        for p in self.planet_models.values_mut() {
            p.update_position(time);
            p.update_a();
        }
    }

    pub fn update_coalescences(&mut self) {
        let State {
            planet_models,
            coalescence,
            event_lock,
            ..
        } = self;
        coalescence.update_status(planet_models, event_lock);
    }

    pub fn update_moon_capture(&mut self) {
        let State {
            planet_models,
            moon_capture,
            event_lock,
            ..
        } = self;
        moon_capture.update_status(planet_models, event_lock);
    }
}
