use std::sync::Mutex;

use crate::{DustBand, Planetesimal, System};
use once_cell::sync::Lazy;

type EventStore = Vec<AccreteEvent>;

pub trait EventSource: Clone {
    fn event(&self, event_type: &str) {}
}

impl EventSource for System {
    fn event(&self, event_type: &str) {
        let mut event_store = EVENT_STORE.lock().expect("Failed to access EVENT_STORE");
        let event = match event_type {
            "system_setup" => Some(AccreteEvent::PlanetarySystemSetup(event_type.to_string(), self.clone())),
            "system_complete" => Some(AccreteEvent::PlanetarySystemComplete(event_type.to_string(), self.clone())),
            _ => None,
        };

        if let Some(e) = event {
            event_store.push(e)
        }
    }
}

impl EventSource for Planetesimal {
    fn event(&self, event_type: &str) {}
}

impl EventSource for DustBand {
    fn event(&self, event_type: &str) {}

}

/// List of events emitted during system generation
#[derive(Debug)]
pub enum AccreteEvent {
    /// once at the very start of accretion
    PlanetarySystemSetup(String, System),
    /// new planetesimal created during accretion process
    PlanetesimalCreated(Planetesimal),
    ///planetesimal finished accretion of dust and gas
    PlanetesimalAccreteDust(Planetesimal),
    /// planetesimal become gas giant
    PlanetesimalToGasGiant(Planetesimal),
    /// dust bands recalculated every time planetesimal finish accretion
    DustBandsUpdated(Vec<DustBand>),
    /// two planetesimals coalesce
    PlanetesimalsCoalesced {
        smaller: Planetesimal,
        larger: Planetesimal,
    },
    /// one planetesimal catch another as moon
    PlanetesimalCaptureMoon {
        moon_id: String,
        planet: Planetesimal,
    },
    /// moons turned into rings
    PlanetesimalMoonToRing(Planetesimal),
    /// once at the very end of accretion
    PostAccretionStarted,
    /// for every outer body injected into system
    OuterBodyInjected(Planetesimal),
    /// planetary environment generated for all planets
    PlanetaryEnvironmentGenerated,
    /// planetary system generation completed
    PlanetarySystemComplete(String, System),
}

// pub fn event<T: EventSource>(event_type: &str, ctx: &T) {
//     let mut event_store = EVENT_STORE.lock().expect("Failed to access EVENT_STORE");
//     let event = match event_type {
//         "system_complete" => Some(AccreteEvent::PlanetarySystemComplete(System::into_orig(ctx))),
//         _ => None,
//     };

//     if let Some(e) = event {
//         event_store.push(e)
//     }
// }

pub static EVENT_STORE: Lazy<Mutex<EventStore>> = Lazy::new(|| Mutex::new(Vec::new()));
