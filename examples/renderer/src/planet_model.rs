use crate::consts::{COALESCE_DISTANCE, SCALE_FACTOR, UPDATE_RATE_A};
use accrete::Planetesimal;
use bevy::{prelude::*, math::vec3};

// pub type PlanetModels = HashMap<String, PlanetModel>;

#[derive(Debug, Clone, Bundle)]
pub struct PlanetModel {
    pub planet_id: PlanetId,
    // pub moon_models: PlanetModels,
    pub position: PlanetPosition,
    pub barycenter: PlanetBarycenter,
    pub orbit: Orbit,
}

impl From<&Planetesimal> for PlanetModel {
    fn from(planetesimal: &Planetesimal) -> Self {
        let Planetesimal { id, a, e, .. } = planetesimal;
        let a = *a as f32 * SCALE_FACTOR;
        let planet_id = PlanetId(id.to_owned());
        let position = PlanetPosition(vec3(-(a - 0.001), 0.0, 0.0));
        let barycenter = PlanetBarycenter {
            position: Vec3::ZERO,
            id: None,
        };
        let orbit = Orbit::new(a, *e as f32);

        PlanetModel {
            planet_id,
            position,
            barycenter,
            orbit,
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct PlanetId(pub String);

#[derive(Debug, Clone, Copy, Component)]
pub struct PlanetPosition(pub Vec3);

impl PlanetPosition {
    pub fn update_position(&mut self, barycenter: &PlanetBarycenter, orbit: &Orbit, time: f64) {
        let Orbit { a, t, b, focus, .. } = orbit;

        let current_t = (time / (t * 360.0) as f64) * 100000.0;

        self.0.x = barycenter.position.x + focus + (a * current_t.cos() as f32);
        self.0.z = barycenter.position.z + b * current_t.sin() as f32;

        // for m in moon_models.values_mut() {
        //     m.barycenter = *position;
        //     m.update_position(time);
        // }
        // TODO speed up near star
    }
}

#[derive(Debug, Clone, Component)]
pub struct PlanetBarycenter {
    pub position: Vec3,
    pub id: Option<String>,
}

impl PlanetBarycenter {
    pub fn update_barycenter(&mut self, other_id: &PlanetId, position: &PlanetPosition, other_position: &PlanetPosition) {
        if let Some(barycenter_id) = &self.id {
            if barycenter_id == &other_id.0 {
                self.position = other_position.0;
                // let distance = other_position.0.distance(position.0);
                // let direction = (position.0 - other_position.0).normalize();
                // if distance > COALESCE_DISTANCE {
                //     self.position += direction * UPDATE_RATE_A;
                // }
            }
        }
        // match &barycenter.id, &barycenter2.id) {
        //     (Some(barycenter_id), None) => if &id2.0 == barycenter_id { barycenter.position = position2.0 },
        //     (None, Some(barycenter_id)) => if &id.0 == barycenter_id { barycenter2.position = position.0 },
        //     _ => ()
        // }
    }
}

#[derive(Debug, Clone, Component)]
pub struct Orbit {
    pub a: f32,
    pub b: f32,
    pub e: f32,
    pub focus: f32,
    pub u: f32,
    pub t: f32,
    pub target_a: Option<f32>,
}

impl Orbit {
    pub fn new(a: f32, e: f32) -> Self {
        let u = 1.0;
        let b = Orbit::get_semiminor_axis(a, e);
        Orbit {
            a,
            u,
            e,
            b,
            focus: Orbit::get_focus(a, b),
            t: Orbit::get_orbital_period(a, u),
            target_a: None,
        }
    }

    pub fn update_orbit(&mut self) {
        if let Some(target_a) = self.target_a {
            let coalesce_distance = (target_a - self.a).abs();
            if coalesce_distance > COALESCE_DISTANCE {
                match self.a < target_a {
                    true => {
                        self.a += UPDATE_RATE_A;
                    }
                    false => {
                        self.a -= UPDATE_RATE_A;
                    }
                }
            }
            self.b = Orbit::get_semiminor_axis(self.a, self.e);
            self.focus = Orbit::get_focus(self.a, self.b);
            self.t = Orbit::get_orbital_period(self.a, self.u);
        }
    }

    fn get_focus(a: f32, b: f32) -> f32 {
        (a.powf(2.0) - b.powf(2.0)).sqrt()
    }

    fn get_orbital_period(a: f32, u: f32) -> f32 {
        2.0 * std::f32::consts::PI * a.powf(1.5) / u.powf(0.5)
    }

    fn get_semiminor_axis(a: f32, e: f32) -> f32 {
        a * (1.0 - e.powf(2.0)).sqrt()
    }
}

pub struct PlanetsPlugin;

impl Plugin for PlanetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_orbits_system)
            .add_system(update_barycenters_system)
            .add_system(update_planets_position_system);
    }
}

fn update_orbits_system(mut query: Query<&mut Orbit>) {
    for mut orbit in query.iter_mut() {
        orbit.update_orbit();
    }
}

fn update_barycenters_system(mut query: Query<(&mut PlanetBarycenter, &PlanetId, &PlanetPosition)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(mut barycenter, id, position), (mut barycenter2, id2, position2)]) =
        iter.fetch_next()
    {
        barycenter.update_barycenter(id2, position, position2);
        barycenter2.update_barycenter(id, position2, position);
    }
}

fn update_planets_position_system(
    time: Res<Time>,
    mut query: Query<(
        &PlanetId,
        &mut PlanetPosition,
        &PlanetBarycenter,
        &Orbit,
        &mut Transform,
    )>,
) {
    let passed_time = time.seconds_since_startup();
    for (id, mut planet_position, barycenter, orbit, mut transform) in query.iter_mut() {
        planet_position.update_position(barycenter, orbit, passed_time);
        transform.translation.x = planet_position.0.x;
        transform.translation.z = planet_position.0.z;
    }
}
