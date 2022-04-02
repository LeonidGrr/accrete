use crate::{planet_model::PlanetModel, state::PlanetModels};
use accrete::Planetesimal;

#[derive(Debug, Clone)]
pub enum MoonCaptureStatus {
    Created,
    Capturing,
    Done,
}

#[derive(Debug, Clone)]
pub struct MoonCapture {
    pub planet_id: String,
    pub moon_id: String,
    pub resulting_model: PlanetModel,
    pub status: MoonCaptureStatus,
}

impl MoonCapture {
    pub fn new(planet_id: &str, moon_id: &str, result: Planetesimal, time: f64) -> Self {
        let resulting_model = PlanetModel::new(result, time);
        MoonCapture {
            planet_id: planet_id.to_owned(),
            moon_id: moon_id.to_owned(),
            resulting_model,
            status: MoonCaptureStatus::Created,
        }
    }

    pub fn update_status(&mut self, planet_models: &mut PlanetModels) {
        let MoonCapture {
            planet_id,
            moon_id,
            resulting_model,
            ..
        } = self;

        let mut moon = None;
        let mut planet = None;
        let resulting_moon = resulting_model.planet.moons.last().expect("No moons present.");
    
        for p in planet_models.values_mut() {
            match &p.planet.id {
                id if id == moon_id => moon = Some(p),
                id if id == planet_id => planet = Some(p),
                _ => (),
            }
        }

        match self.status {
            MoonCaptureStatus::Created => self.status = MoonCaptureStatus::Capturing,
            MoonCaptureStatus::Capturing => {
                if let (Some(moon), Some(planet)) = (moon, planet) {
                    moon.barycenter = planet.position;
                    moon.target_a = Some(resulting_moon.a as f32);
                    // self.status = MoonCaptureStatus::Capturing;
                }
            },
            MoonCaptureStatus::Done => (),
        };
    }
}
