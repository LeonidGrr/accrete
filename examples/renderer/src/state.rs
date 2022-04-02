use std::collections::HashMap;
use crate::coalescence::{Coalescence, CoalescenceStatus};
use crate::moon_capture::{MoonCapture, MoonCaptureStatus};
use crate::planet_model::PlanetModel;
use accrete::events::AccreteEvent;
use accrete::DustBand;
use macroquad::prelude::*;

pub type PlanetModels = HashMap<String, PlanetModel>;

pub struct State {
    pub planet_models: PlanetModels,
    pub coalescence: Option<Coalescence>,
    pub moon_capture: Option<MoonCapture>,
    pub dust_model: HashMap<String, DustBand>,
    pub event_idx: usize,
    // Delta time step
    pub dt: f32,
    pub event_lock: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            dt: 1.0,
            event_idx: 0,
            planet_models: HashMap::new(),
            coalescence: None,
            moon_capture: None,
            dust_model: HashMap::new(),
            event_lock: false,
        }
    }

    pub fn event_handler(&mut self, current_event: &AccreteEvent, time: f64) {
        match current_event {
            // AccreteEvent::PlanetarySystemSetup(_, _) => (),
            AccreteEvent::PlanetesimalCreated(_, planet) => {
                if self.planet_models.len() <= 20 {
                    let p = PlanetModel::new(planet.clone(), time);
                    self.planet_models.insert(p.id.clone(), p);
                }
            }
            // AccreteEvent::PlanetesimalAccretedDust(name, _) => name,
            AccreteEvent::PlanetesimalToGasGiant(_, gas_giant) => {
                if self.planet_models.get(&gas_giant.id).is_some() {
                    self.planet_models.insert(gas_giant.id.clone(), PlanetModel::new(gas_giant.clone(), time));
                }
            }
            // AccreteEvent::DustBandsUpdated(_, _) => (),
            AccreteEvent::PlanetesimalsCoalesced(_, source_planet_id, target_planet_id, result) => {
                let c = Coalescence::new(source_planet_id, target_planet_id, result.clone(), time);
                self.coalescence = Some(c);
                self.event_lock = true;
            }
            AccreteEvent::PlanetesimalCaptureMoon(_, planet_id, moon_id, result) => {
                let m = MoonCapture::new(planet_id, moon_id, result.clone(), time);
                self.moon_capture = Some(m);
                self.event_lock = true;
            }
            // AccreteEvent::PlanetesimalMoonToRing(name, _) => name,
            // AccreteEvent::PostAccretionStarted(name) => name,
            // AccreteEvent::OuterBodyInjected(name, _) => name,
            // AccreteEvent::PlanetaryEnvironmentGenerated(name, _) => name,
            AccreteEvent::PlanetarySystemComplete(_, _) => (),
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
            ..
        } = self;
        if let Some(c) = coalescence {
            match c.status {
                CoalescenceStatus::Done => self.event_lock = false,
                _ => c.update_status(planet_models),
            }
        }
    }

    pub fn update_moon_capture(&mut self) {
        let State {
            planet_models,
            moon_capture,
            ..
        } = self;
        if let Some(m) = moon_capture {
            match m.status {
                MoonCaptureStatus::Done => self.event_lock = false,
                _ => m.update_status(planet_models),
            }
        }
    }
}
