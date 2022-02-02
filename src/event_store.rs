use std::sync::Mutex;

use crate::{Accrete, System, Planetesimal, DustBand};
use once_cell::sync::Lazy;

#[derive(Debug)]
pub enum AccreteEvent {
    AccreteInstaceCreated(Accrete),
    PlanetarySystemCreated(System),
    PlanetesimalCreated(Planetesimal),
    PlanetesimalCollectedDust(Planetesimal),
    PlanetesimalToGasGiant(Planetesimal),
    DustBandUpdated(Vec<DustBand>),
    PlanetesimalsCoalesced { smaller: Planetesimal, larger: Planetesimal },
    PlanetesimalCaptureMoon{ moon_id: String, planet: Planetesimal },
    PlanetesimalMoonToRing(Planetesimal),
    PostAccretionStarted,
    PlanetaryEnvironmentGenerated,
    PlanetarySystemComplete(System)
}

pub fn event(event: AccreteEvent) {
    let mut event_store = EVENT_STORE.lock().expect("Failed to access EVENT_STORE");
    event_store.push(event)
}

pub static EVENT_STORE: Lazy<Mutex<Vec<AccreteEvent>>> = Lazy::new(|| {
    Mutex::new(Vec::new())
});