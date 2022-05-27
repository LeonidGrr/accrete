use super::accrete_event::AccreteEvent;
use crate::{structs::dust::DustBands, Planetesimal, Ring, System};
use once_cell::sync::Lazy;
use std::sync::Mutex;

type Events = Vec<AccreteEvent>;

pub static EVENTS: Lazy<Mutex<Events>> = Lazy::new(|| Mutex::new(Vec::new()));

pub trait EventSource: Clone {
    fn event(&self, _event_type: &str) {}
}

impl EventSource for System {
    fn event(&self, event_type: &str) {
        let mut events = EVENTS.lock().expect("Failed to access EVENT_STORE");
        let event = match event_type {
            "system_setup" => Some(AccreteEvent::PlanetarySystemSetup(
                event_type.to_string(),
                self.clone(),
            )),
            "post_accretion_started" => {
                Some(AccreteEvent::PostAccretionStarted(event_type.to_string()))
            }
            "planetary_environment_generated" => Some(AccreteEvent::PlanetaryEnvironmentGenerated(
                event_type.to_string(),
                self.clone(),
            )),
            "system_complete" => Some(AccreteEvent::PlanetarySystemComplete(
                event_type.to_string(),
                self.clone(),
            )),
            _ => None,
        };

        if let Some(e) = event {
            events.push(e)
        }
    }
}

impl EventSource for Planetesimal {
    fn event(&self, event_type: &str) {
        let mut events = EVENTS.lock().expect("Failed to access EVENT_STORE");
        let mut event = match event_type {
            "planetesimal_created" => Some(AccreteEvent::PlanetesimalCreated(
                event_type.to_string(),
                self.clone(),
            )),
            "planetesimal_updated" => Some(AccreteEvent::PlanetesimalUpdated(
                event_type.to_string(),
                self.clone(),
            )),
            "planetesimal_to_gas_giant" => Some(AccreteEvent::PlanetesimalToGasGiant(
                event_type.to_string(),
                self.clone(),
            )),
            "outer_body_injected" => Some(AccreteEvent::OuterBodyInjected(
                event_type.to_string(),
                self.clone(),
            )),
            _ => None,
        };

        if event_type.contains("planetesimals_coalesced") {
            let data: Vec<&str> = event_type.split(':').collect();
            event = Some(AccreteEvent::PlanetesimalsCoalesced(
                data[0].to_string(),
                data[1].to_string(),
                data[2].to_string(),
                self.clone(),
            ));
        }

        if event_type.contains("moons_coalesced") {
            let data: Vec<&str> = event_type.split(':').collect();
            event = Some(AccreteEvent::MoonsCoalesced(
                data[0].to_string(),
                data[1].to_string(),
                data[2].to_string(),
                self.clone(),
            ));
        }

        if event_type.contains("planetesimal_capture_moon") {
            let data: Vec<&str> = event_type.split(':').collect();
            event = Some(AccreteEvent::PlanetesimalCaptureMoon(
                data[0].to_string(),
                data[1].to_string(),
                data[2].to_string(),
                self.clone(),
            ));
        }

        if let Some(e) = event {
            events.push(e)
        }
    }
}

impl EventSource for Ring {
    fn event(&self, event_type: &str) {
        let mut events = EVENTS.lock().expect("Failed to access EVENT_STORE");
        if event_type.contains("moon_to_ring") {
            let data: Vec<&str> = event_type.split(':').collect();
            events.push(AccreteEvent::PlanetesimalMoonToRing(
                data[0].to_string(),
                data[1].to_string(),
                data[2].to_string(),
                self.clone(),
            ));
        }
    }
}

impl EventSource for DustBands {
    fn event(&self, event_type: &str) {
        let mut events = EVENTS.lock().expect("Failed to access EVENT_STORE");
        let event = match event_type {
            "dust_bands_updated" => Some(AccreteEvent::DustBandsUpdated(
                event_type.to_string(),
                self.clone(),
            )),
            _ => None,
        };

        if let Some(e) = event {
            events.push(e)
        }
    }
}
