use crate::consts::{
    COALESCE_DISTANCE_RATE, PLANET_PERIOD_FACTOR, PLANET_RADIUS_SCALE_FACTOR, SCALE_FACTOR,
    UPDATE_RATE_A,
};
use accrete::Planetesimal;
use bevy::{math::vec3, prelude::*, tasks::TaskPool};

#[derive(Debug, Clone, Bundle)]
pub struct PlanetModel {
    pub planet_id: PlanetId,
    pub position: PlanetPosition,
    pub orbit: Orbit,
}

impl From<&Planetesimal> for PlanetModel {
    fn from(planetesimal: &Planetesimal) -> Self {
        let Planetesimal { id, a, e, .. } = planetesimal;
        let a = *a as f32 * SCALE_FACTOR;
        let planet_id = PlanetId(id.to_owned());
        let position = PlanetPosition(vec3(-(a - 0.001), 0.0, 0.0));
        let orbit = Orbit::new(a, *e as f32);

        PlanetModel {
            planet_id,
            position,
            orbit,
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct PlanetId(pub String);

#[derive(Debug, Clone, Copy, Component)]
pub struct PlanetPosition(pub Vec3);

impl PlanetPosition {
    pub fn update_position(&mut self, orbit: &Orbit, time: f64) {
        let Orbit { a, t, b, focus, .. } = orbit;

        let current_t = (time / (t * 360.0) as f64) * PLANET_PERIOD_FACTOR;

        self.0.x = focus + (a * current_t.cos() as f32);
        self.0.z = b * current_t.sin() as f32;

        // TODO speed up near star
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
        }
    }

    pub fn update_orbit(&mut self, target_a: f32, immediate: bool) {
        let distance = (target_a - self.a).abs();
        if immediate || distance < COALESCE_DISTANCE_RATE {
            self.a = target_a;
        } else {
            let modifier = UPDATE_RATE_A * distance.powf(2.0);
            match self.a < target_a {
                true => {
                    self.a += modifier;
                }
                false => {
                    self.a -= modifier;
                }
            }
        }
        self.b = Orbit::get_semiminor_axis(self.a, self.e);
        self.focus = Orbit::get_focus(self.a, self.b);
        self.t = Orbit::get_orbital_period(self.a, self.u);
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
        app.add_system(update_planets_position_system);
    }
}

fn update_planets_position_system(
    time: Res<Time>,
    mut query: Query<(&mut PlanetPosition, &Orbit, &mut Transform)>,
) {
    let passed_time = time.seconds_since_startup();
    let taskpool = TaskPool::new();
    query.par_for_each_mut(
        &taskpool,
        16,
        |(mut planet_position, orbit, mut transform)| {
            planet_position.update_position(orbit, passed_time);
            transform.translation.x = planet_position.0.x;
            transform.translation.z = planet_position.0.z;
        },
    );
}

pub fn udpate_planet_mesh_from_planetesimal(
    mesh_handle: &Handle<Mesh>,
    meshes: &mut ResMut<Assets<Mesh>>,
    planetesimal: &Planetesimal,
) {
    if let Some(mesh) = meshes.get_mut(mesh_handle) {
        let next_mesh = Mesh::from(shape::Icosphere {
            radius: planetesimal.radius as f32 * PLANET_RADIUS_SCALE_FACTOR,
            subdivisions: 32,
        });
        mesh.clone_from(&next_mesh);
    }
}
