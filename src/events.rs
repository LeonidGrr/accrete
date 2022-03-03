use std::sync::Mutex;

use crate::{structs::dust::DustBands, Planetesimal, System};
use once_cell::sync::Lazy;

type Events = Vec<AccreteEvent>;

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
            "planetesimal_accreted_dust" => Some(AccreteEvent::PlanetesimalAccretedDust(
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
            "moon_to_ring" => Some(AccreteEvent::PlanetesimalMoonToRing(
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

/// List of events emitted during system generation
#[derive(Debug)]
pub enum AccreteEvent {
    /// once at the very start of accretion
    PlanetarySystemSetup(String, System),
    /// new planetesimal created during accretion process
    PlanetesimalCreated(String, Planetesimal),
    ///planetesimal finished accretion of dust and gas
    PlanetesimalAccretedDust(String, Planetesimal),
    /// planetesimal become gas giant
    PlanetesimalToGasGiant(String, Planetesimal),
    /// dust bands recalculated every time planetesimal finish accretion
    DustBandsUpdated(String, DustBands),
    /// two planetesimals coalesce
    PlanetesimalsCoalesced(String, String, String, Planetesimal),
    /// one planetesimal catch another as moon
    PlanetesimalCaptureMoon(String, String, String, Planetesimal),
    /// moons turned into rings
    PlanetesimalMoonToRing(String, Planetesimal),
    /// once at the very end of accretion
    PostAccretionStarted(String),
    /// for every outer body injected into system
    OuterBodyInjected(String, Planetesimal),
    /// planetary environment generated for all planets
    PlanetaryEnvironmentGenerated(String, System),
    /// planetary system generation completed
    PlanetarySystemComplete(String, System),
}

impl AccreteEvent {
    pub fn name(&self) -> &str {
        match self {
            AccreteEvent::PlanetarySystemSetup(name, _) => name,
            AccreteEvent::PlanetesimalCreated(name, _) => name,
            AccreteEvent::PlanetesimalAccretedDust(name, _) => name,
            AccreteEvent::PlanetesimalToGasGiant(name, _) => name,
            AccreteEvent::DustBandsUpdated(name, _) => name,
            AccreteEvent::PlanetesimalsCoalesced(name, _, _, _) => name,
            AccreteEvent::PlanetesimalCaptureMoon(name, _, _, _) => name,
            AccreteEvent::PlanetesimalMoonToRing(name, _) => name,
            AccreteEvent::PostAccretionStarted(name) => name,
            AccreteEvent::OuterBodyInjected(name, _) => name,
            AccreteEvent::PlanetaryEnvironmentGenerated(name, _) => name,
            AccreteEvent::PlanetarySystemComplete(name, _) => name,
        }
    }
}

pub static EVENTS: Lazy<Mutex<Events>> = Lazy::new(|| Mutex::new(Vec::new()));
