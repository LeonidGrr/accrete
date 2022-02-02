use std::sync::Mutex;

use crate::{DustBand, Planetesimal, System};
use once_cell::sync::Lazy;

/// List of events emitted during system generation
#[derive(Debug)]
pub enum AccreteEvent {
    /// once at the very start of accretion
    PlanetarySystemSetup(System),
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
    PlanetarySystemComplete(System),
}

pub fn event(event: AccreteEvent) {
    let mut event_store = EVENT_STORE.lock().expect("Failed to access EVENT_STORE");
    event_store.push(event)
}

pub static EVENT_STORE: Lazy<Mutex<Vec<AccreteEvent>>> = Lazy::new(|| Mutex::new(Vec::new()));
