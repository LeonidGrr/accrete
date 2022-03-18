use crate::planet::Planet;
use accrete::events::AccreteEvent;
use accrete::DustBand;

pub struct State<'a> {
    pub planets: Vec<Planet<'a>>,
    pub dust: Vec<DustBand>,
    pub event_idx: usize,
    pub current_event: &'a AccreteEvent,
    pub step: f64,
}

impl<'a> State<'a> {
    pub fn event_handler(&mut self) {
        match self.current_event {
            AccreteEvent::PlanetarySystemSetup(_, _) => (),
            AccreteEvent::PlanetesimalCreated(_, planet) => {
                if !planet.is_moon {
                    let p = Planet::new(planet);
                    self.planets.push(p);
                }
            },
            // AccreteEvent::PlanetesimalAccretedDust(name, _) => name,
            // AccreteEvent::PlanetesimalToGasGiant(name, _) => name,
            AccreteEvent::DustBandsUpdated(_, dust_bands) => self.dust = dust_bands.to_vec(),
            // AccreteEvent::PlanetesimalsCoalesced(name, _, _, _) => name,
            // AccreteEvent::PlanetesimalCaptureMoon(name, _, _, _) => name,
            // AccreteEvent::PlanetesimalMoonToRing(name, _) => name,
            // AccreteEvent::PostAccretionStarted(name) => name,
            // AccreteEvent::OuterBodyInjected(name, _) => name,
            // AccreteEvent::PlanetaryEnvironmentGenerated(name, _) => name,
            // AccreteEvent::PlanetarySystemComplete(name, _) => name,
            _ => (),
        }
    }
}
