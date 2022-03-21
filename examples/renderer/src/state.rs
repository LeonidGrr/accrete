use macroquad::prelude::*;
use crate::planet_model::PlanetModel;
use crate::coalescence::{Coalescence, CoalescenceStatus};
use accrete::events::{AccreteEvent, EVENTS};
use accrete::{DustBand, Planetesimal};

pub struct State {
    pub planets: Vec<Planetesimal>,
    pub planet_models: Vec<PlanetModel>,
    pub coalescences: Vec<Coalescence>,
    pub dust: Vec<DustBand>,
    pub event_idx: usize,
    // Delta time step
    pub dt: f32,
}

impl State {
    pub fn new() -> Self {
        State {
            dt: 1.0,
            event_idx: 0,
            planets: vec![],
            planet_models: vec![],
            coalescences: vec![],
            dust: vec![],
        }
    }
 
    pub fn event_handler(&mut self, current_event: &AccreteEvent) {
        match current_event {
            AccreteEvent::PlanetarySystemSetup(_, _) => (),
            AccreteEvent::PlanetesimalCreated(_, planet) => {
                if !planet.is_moon && (planet.id == "Q5okvuf" || planet.id == "6tdVpJl") {
                    let p = PlanetModel::new(&planet, self.dt);
                    self.planet_models.push(p);
                    self.planets.push(planet.clone());
                }
            }
            // AccreteEvent::PlanetesimalAccretedDust(name, _) => name,
            // AccreteEvent::PlanetesimalToGasGiant(name, _) => name,
            AccreteEvent::DustBandsUpdated(_, dust_bands) => self.dust = dust_bands.to_vec(),
            AccreteEvent::PlanetesimalsCoalesced(_, source_planet_id, target_planet_id, coalesced) => {
                let c = Coalescence::new(source_planet_id.to_owned(), target_planet_id.to_owned(), coalesced.clone(), self.dt);
                self.coalescences.push(c);
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

    pub fn update_planets(&mut self) {
        for p in self.planet_models.iter_mut() {
            if let Some(target) = p.coalescence_target {
                p.update_position_by_target(target);
            } else {
                p.update_position(self.dt);
            }
        }
    }

    pub fn update_coalescences(&mut self) {
        let State { planet_models, .. } = self;
        for c in self.coalescences.iter_mut() {
            let Coalescence { target_planet_id, source_planet_id, status, .. } = c;
            
            match status {
                CoalescenceStatus::Approaching => {
                    let mut source_planet_idx = None;
                    let mut target_planet_idx = None;
                    for (idx, p) in planet_models.iter_mut().enumerate() {
                        match &p.planet_id {
                            id if id == source_planet_id => source_planet_idx = Some(idx),
                            id if id == target_planet_id => target_planet_idx = Some(idx),
                            _ => (),
                        }
                    }
                    if let (Some(source_planet_idx), Some(target_planet_idx)) = (source_planet_idx, target_planet_idx) {
                        let coalesce_distance = (planet_models[target_planet_idx].a - planet_models[source_planet_idx].a).abs();
                        let current_distance = planet_models[source_planet_idx].position.distance(planet_models[target_planet_idx].position);
                        if current_distance < coalesce_distance * 1.05 {
                            planet_models[source_planet_idx].coalescence_target = Some(planet_models[target_planet_idx].position);
                        }
                        if current_distance <= 1.0 {
                            planet_models.remove(source_planet_idx);
                            planet_models[target_planet_idx].coalescence_target = Some(c.coalesced_model.position);
                            *status = CoalescenceStatus::Coalescing;
                        }
                    }
                },
                CoalescenceStatus::Coalescing => {},
                CoalescenceStatus::Done => {},
            };
        }
    }
}
