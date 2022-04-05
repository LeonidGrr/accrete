use crate::{consts::MOON_CAPTURE_DISTANCE, planet_model::PlanetModel, state::PlanetModels};
use accrete::Planetesimal;

#[derive(Debug, Clone)]
enum MoonCaptureStatus {
    Created,
    Approaching,
    Capturing,
    Done,
}

#[derive(Debug, Clone)]
struct MoonCapture {
    planet_id: String,
    moon_id: String,
    resulting_model: PlanetModel,
    status: MoonCaptureStatus,
}

pub struct MoonCaptureOption(Option<MoonCapture>);

impl MoonCaptureOption {
    pub fn none() -> Self {
        MoonCaptureOption(None)
    }
    pub fn new(planet_id: &str, moon_id: &str, result: Planetesimal, time: f64) -> Self {
        let resulting_model = PlanetModel::new(result, time);
        let m = MoonCapture {
            planet_id: planet_id.to_owned(),
            moon_id: moon_id.to_owned(),
            resulting_model,
            status: MoonCaptureStatus::Created,
        };

        MoonCaptureOption(Some(m))
    }

    pub fn update_status(&mut self, planet_models: &mut PlanetModels, event_lock: &mut bool) {
        if let Some(m) = &mut self.0 {
            let MoonCapture {
                planet_id,
                moon_id,
                resulting_model,
                ..
            } = m;

            let mut moon = None;
            let mut planet = None;
            let resulting_moon = resulting_model
                .planet
                .moons
                .last()
                .expect("No moons present.");

            for p in planet_models.values_mut() {
                match &p.planet.id {
                    id if id == moon_id => moon = Some(p),
                    id if id == planet_id => planet = Some(p),
                    _ => (),
                }
            }

            match m.status {
                MoonCaptureStatus::Created => {
                    *event_lock = true;
                    m.status = MoonCaptureStatus::Approaching;
                }
                MoonCaptureStatus::Approaching => {
                    if let (Some(moon), Some(planet)) = (moon, planet) {
                        moon.barycenter = planet.position;
                        moon.target_a = Some(resulting_moon.a as f32);

                        let distance = planet.position.distance(moon.position);

                        println!("Distance to moon capture: {} au", distance);
                        if distance <= MOON_CAPTURE_DISTANCE {
                            m.status = MoonCaptureStatus::Capturing;
                        }
                    }
                }
                MoonCaptureStatus::Capturing => {
                    if let (Some(moon), Some(planet)) = (moon, planet) {
                        resulting_model.position = planet.position;
                        let resulting_moon_model = resulting_model.moon_models.get_mut(moon_id).expect("Failed to find moon by id.");
                        resulting_moon_model.position = moon.position;
                        planet_models.remove(moon_id);
                        planet_models.remove(planet_id);
                        planet_models.insert(resulting_model.id.clone(), resulting_model.clone());
                        m.status = MoonCaptureStatus::Done;
                    }
                }
                MoonCaptureStatus::Done => {
                    *self = MoonCaptureOption(None);
                    *event_lock = false;
                }
            };
        }
    }
}
