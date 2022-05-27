use crate::{structs::dust::DustBands, Planetesimal, Ring, System};

/// List of events emitted during system generation
#[derive(Debug, Clone)]
pub enum AccreteEvent {
    None,
    /// once at the very start of accretion
    PlanetarySystemSetup(String, System),
    /// new planetesimal created during accretion process
    PlanetesimalCreated(String, Planetesimal),
    ///planetesimal finished accretion of dust and gas
    PlanetesimalUpdated(String, Planetesimal),
    /// planetesimal become gas giant
    PlanetesimalToGasGiant(String, Planetesimal),
    /// dust bands recalculated every time planetesimal finish accretion
    DustBandsUpdated(String, DustBands),
    /// two planetesimals coalesce
    PlanetesimalsCoalesced(String, String, String, Planetesimal),
    /// two moons coalesce
    MoonsCoalesced(String, String, String, Planetesimal),
    /// one planetesimal catch another as moon
    PlanetesimalCaptureMoon(String, String, String, Planetesimal),
    /// moons turned into rings
    PlanetesimalMoonToRing(String, String, String, Ring),
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
