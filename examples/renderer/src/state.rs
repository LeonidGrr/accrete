use accrete::events::AccreteEvent;
use crate::planet::Planet;

pub struct State<'a> {
    pub planets: Vec<Planet<'a>>,
    pub event_idx: usize,
    pub current_event: &'a AccreteEvent,
    pub step: f64,
}

impl State<'_> {
    pub fn event_handler(&mut self) {
        match self.current_event {
            AccreteEvent::PlanetarySystemSetup(_, _) => (),
            AccreteEvent::PlanetesimalCreated(_, planet) => self.planets.push(Planet::new(planet)),
            // AccreteEvent::PlanetesimalAccretedDust(name, _) => name,
            // AccreteEvent::PlanetesimalToGasGiant(name, _) => name,
            // AccreteEvent::DustBandsUpdated(name, _) => name,
            // AccreteEvent::PlanetesimalsCoalesced(name, _, _, _) => name,
            // AccreteEvent::PlanetesimalCaptureMoon(name, _, _, _) => name,
            // AccreteEvent::PlanetesimalMoonToRing(name, _) => name,
            // AccreteEvent::PostAccretionStarted(name) => name,
            // AccreteEvent::OuterBodyInjected(name, _) => name,
            // AccreteEvent::PlanetaryEnvironmentGenerated(name, _) => name,
            // AccreteEvent::PlanetarySystemComplete(name, _) => name,
            _ => ()
        }
    }
}