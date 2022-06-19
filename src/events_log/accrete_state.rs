use super::accrete_event::AccreteEvent;
use crate::{Planetesimal, System};

#[derive(Debug, Clone)]
pub struct AccreteState {
    pub system: System,
}

impl From<&System> for AccreteState {
    fn from(system: &System) -> Self {
        AccreteState {
            system: system.clone(),
        }
    }
}

impl TryFrom<&AccreteEvent> for AccreteState {
    type Error = String;

    fn try_from(event: &AccreteEvent) -> Result<Self, Self::Error> {
        match event {
            AccreteEvent::PlanetarySystemSetup(_, system)
            | AccreteEvent::PlanetaryEnvironmentGenerated(_, system)
            | AccreteEvent::PlanetarySystemComplete(_, system) => Ok(AccreteState {
                system: system.clone(),
            }),
            _ => Err(
                "Failed to restore state from AccreteEvent. No System data present with event."
                    .to_string(),
            ),
        }
    }
}

impl AccreteState {
    pub fn set_from_event(&mut self, event: &AccreteEvent) {
        match event {
            AccreteEvent::PlanetarySystemSetup(_, system)
            | AccreteEvent::PlanetaryEnvironmentGenerated(_, system) => {
                self.system = system.clone()
            }

            AccreteEvent::OuterBodyInjected(_, planetesimal)
            | AccreteEvent::PlanetesimalCreated(_, planetesimal) => {
                self.system.planets.push(planetesimal.clone())
            }

            AccreteEvent::PlanetesimalUpdated(_, planetesimal)
            | AccreteEvent::PlanetesimalToGasGiant(_, planetesimal) => {
                let planet = self.find_planetesimal_mut(&planetesimal.id);
                *planet = planetesimal.clone();
            }

            AccreteEvent::DustBandsUpdated(_, dust_bands) => {
                self.system.dust_bands = dust_bands.clone()
            }

            AccreteEvent::PlanetesimalCaptureMoon(_, id1, id2, planetesimal)
            | AccreteEvent::PlanetesimalsCoalesced(_, id1, id2, planetesimal) => {
                self.system.planets = self
                    .system
                    .planets
                    .iter()
                    .filter(|p| &p.id != id1 && &p.id != id2)
                    .cloned()
                    .collect();
                self.system.planets.push(planetesimal.clone());
            }

            AccreteEvent::PlanetesimalMoonToRing(_, planet_id, moon_id, ring) => {
                let planet = self.find_planetesimal_mut(planet_id);
                planet.moons = planet
                    .moons
                    .iter()
                    .filter(|m| &m.id != moon_id)
                    .cloned()
                    .collect();
                planet.rings.push(ring.clone());
            }
            AccreteEvent::MoonsCoalesced(_, id1, id2, planetesimal) => {
                let mut planet = None;
                for p in self.system.planets.iter_mut() {
                    if planet.is_some() {
                        break;
                    }

                    planet = p.moons.iter_mut().find(|m| m.id == planetesimal.id);
                }
                if let Some(planet) = planet {
                    planet.moons = planet
                        .moons
                        .iter()
                        .filter(|m| &m.id != id1 && &m.id != id2)
                        .cloned()
                        .collect();
                    planet.moons.push(planetesimal.clone());
                }
            }
            AccreteEvent::PostAccretionStarted(_) => (),
            AccreteEvent::PlanetarySystemComplete(_, system) => {
                assert_eq!(format!("{:?}", &self.system), format!("{:?}", system))
            }
            AccreteEvent::None => (),
        }
        self.system.planets.sort_by(|p1, p2| {
            p1.a.partial_cmp(&p2.a)
                .expect("Failed to sort planetesimals.")
        })
    }

    pub fn find_planetesimal_mut(&mut self, id: &str) -> &mut Planetesimal {
        self.system
            .planets
            .iter_mut()
            .find(|p| p.id == id)
            .expect("Failed to find planet by id")
    }

    pub fn find_planetesimal(&self, id: &str) -> &Planetesimal {
        self.system
            .planets
            .iter()
            .find(|p| p.id == id)
            .expect("Failed to find planet by id")
    }
}
