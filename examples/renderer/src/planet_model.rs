use crate::consts::COALESCE_DISTANCE;
use bevy::{prelude::*, render::camera::CameraProjection};

// pub type PlanetModels = HashMap<String, PlanetModel>;

#[derive(Debug, Clone, Bundle)]
pub struct PlanetModel {
    pub planet_id: PlanetId,
    // pub moon_models: PlanetModels,
    pub position: PlanetPosition,
    pub barycenter: PlanetBarycenter,
    pub orbit: Orbit,
}

#[derive(Debug, Clone, Component)]
pub struct PlanetId(pub String);

#[derive(Debug, Clone, Copy, Component)]
pub struct PlanetPosition(pub Vec3);

impl PlanetPosition {
    pub fn update_position(&mut self, barycenter: &PlanetBarycenter, orbit: &Orbit, time: f64) {
        let Orbit { a, t, b, focus, .. } = orbit;

        let current_t = (time / (t * 360.0) as f64) * 100000.0;

        self.0.x = barycenter.0.x + focus + (a * current_t.cos() as f32);
        self.0.z = barycenter.0.y + (b * current_t.sin() as f32);

        // for m in moon_models.values_mut() {
        //     m.barycenter = *position;
        //     m.update_position(time);
        // }
        // TODO speed up near star
    }
}

#[derive(Debug, Clone, Component)]
pub struct PlanetBarycenter(pub Vec3);

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
                        self.a += 0.25;
                    }
                    false => {
                        self.a -= 0.25;
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
        app
            .add_system(update_orbits_system)
            .add_system(update_planets_position_system);
    }
}

fn update_orbits_system(mut query: Query<&mut Orbit>) {
    for mut orbit in query.iter_mut() {
        orbit.update_orbit();
    }
}


fn update_planets_position_system(time: Res<Time>, mut query: Query<(&mut PlanetPosition, &PlanetBarycenter, &Orbit, &mut Transform)>) {
    let passed_time = time.seconds_since_startup();
    for (mut planet_position, barycenter, orbit, mut transform) in query.iter_mut() {
        planet_position.update_position(barycenter, &orbit, passed_time);
        transform.translation.x = planet_position.0.x;
        transform.translation.z = planet_position.0.z;
    }
}
