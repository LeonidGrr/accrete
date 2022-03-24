use macroquad::prelude::*;
use crate::planet_model::PlanetModel;
use crate::coalescence::{Coalescence, CoalescenceStatus};
use accrete::events::AccreteEvent;
use accrete::DustBand;

pub struct State {
    pub planet_models: Vec<PlanetModel>,
    pub coalescence: Option<Coalescence>,
    pub dust_model: Vec<DustBand>,
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
            planet_models: vec![],
            coalescence: None,
            dust_model: vec![],
            event_lock: false,
        }
    }
 
    pub fn event_handler(&mut self, current_event: &AccreteEvent, time: f64) {
        match current_event {
            AccreteEvent::PlanetarySystemSetup(_, _) => (),
            AccreteEvent::PlanetesimalCreated(_, planet) => {
                if !planet.is_moon && (planet.id == "Q5okvuf" || planet.id == "6tdVpJl") {
                // if !planet.is_moon {
                    let p = PlanetModel::new(planet.clone(), time);
                    self.planet_models.push(p);
                }
            }
            // AccreteEvent::PlanetesimalAccretedDust(name, _) => name,
            // AccreteEvent::PlanetesimalToGasGiant(name, _) => name,
            AccreteEvent::DustBandsUpdated(_, dust_bands) => self.dust_model = dust_bands.to_vec(),
            AccreteEvent::PlanetesimalsCoalesced(_, source_planet_id, target_planet_id, coalesced) => {
                let c = Coalescence::new(source_planet_id, target_planet_id, coalesced.clone(), time);
                self.coalescence = Some(c);
                self.event_lock = true;
            },
            // AccreteEvent::PlanetesimalCaptureMoon(name, _, _, _) => name,
            // AccreteEvent::PlanetesimalMoonToRing(name, _) => name,
            // AccreteEvent::PostAccretionStarted(name) => name,
            // AccreteEvent::OuterBodyInjected(name, _) => name,
            // AccreteEvent::PlanetaryEnvironmentGenerated(name, _) => name,
            // AccreteEvent::PlanetarySystemComplete(name, _) => name,
            _ => (),
        }
    }

    pub fn update_planets(&mut self, time: f64) {
        for p in self.planet_models.iter_mut() {
            if let Some(coalescence_a) = p.coalescence_a {
                p.update_a(coalescence_a);
            }
            p.update_position(time);
        }
    }

    pub fn update_coalescences(&mut self) {
        let State { planet_models, coalescence, .. } = self;
        if let Some(c) = coalescence {
            match c.status {
                CoalescenceStatus::Done => {
                    self.event_lock = false;
                    debug!("{} {:?}", self.event_lock, c.status);
                },
                _ => c.update_status(planet_models),
            }
        }
    }
}
