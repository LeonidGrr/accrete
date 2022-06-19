use crate::{structs::dust::DustBands, Planetesimal, Ring, System};
use serde::{Deserialize, Serialize};

pub type AccreteEvents = Vec<AccreteEvent>;

/// Event emitted during system generation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccreteEvent {
    None,
    /// Once at the very start of accretion
    PlanetarySystemSetup(String, System),
    /// New planetesimal created during accretion process
    PlanetesimalCreated(String, Planetesimal),
    /// Planetesimal finished accretion of dust and gas
    PlanetesimalUpdated(String, Planetesimal),
    /// Planetesimal become gas giant
    PlanetesimalToGasGiant(String, Planetesimal),
    /// Dust bands recalculated every time planetesimal finish accretion
    DustBandsUpdated(String, DustBands),
    /// Two planetesimals coalesce
    PlanetesimalsCoalesced(String, String, String, Planetesimal),
    /// Two moons coalesce
    MoonsCoalesced(String, String, String, Planetesimal),
    /// One planetesimal catch another as moon
    PlanetesimalCaptureMoon(String, String, String, Planetesimal),
    /// Moons turned into rings
    PlanetesimalMoonToRing(String, String, String, Ring),
    /// Once at the very end of accretion
    PostAccretionStarted(String),
    /// For every outer body injected into system
    OuterBodyInjected(String, Planetesimal),
    /// Planetary environment generated for all planets
    PlanetaryEnvironmentGenerated(String, System),
    /// Planetary system generation completed
    PlanetarySystemComplete(String, System),
}

impl AccreteEvent {
    pub fn name(&self) -> &str {
        match self {
            AccreteEvent::PlanetarySystemSetup(name, _) => name,
            AccreteEvent::PlanetesimalCreated(name, _) => name,
            AccreteEvent::PlanetesimalUpdated(name, _) => name,
            AccreteEvent::PlanetesimalToGasGiant(name, _) => name,
            AccreteEvent::DustBandsUpdated(name, _) => name,
            AccreteEvent::PlanetesimalsCoalesced(name, _, _, _) => name,
            AccreteEvent::MoonsCoalesced(name, _, _, _) => name,
            AccreteEvent::PlanetesimalCaptureMoon(name, _, _, _) => name,
            AccreteEvent::PlanetesimalMoonToRing(name, _, _, _) => name,
            AccreteEvent::PostAccretionStarted(name) => name,
            AccreteEvent::OuterBodyInjected(name, _) => name,
            AccreteEvent::PlanetaryEnvironmentGenerated(name, _) => name,
            AccreteEvent::PlanetarySystemComplete(name, _) => name,
            AccreteEvent::None => "",
        }
    }
}
