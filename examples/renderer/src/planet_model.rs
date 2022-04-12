use std::collections::HashSet;
use crate::consts::{COALESCE_DISTANCE, SCALE_FACTOR, UPDATE_RATE_A, UPDATE_RATE_BARYCENTER, PLANET_RADIUS_SCALE_FACTOR};
use accrete::Planetesimal;
use bevy::{prelude::*, math::vec3};

#[derive(Debug, Clone, Bundle)]
pub struct PlanetModel {
    pub planet_id: PlanetId,
    pub moons_ids: MoonsIds,
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
        let moons_ids = MoonsIds(HashSet::new());

        PlanetModel {
            planet_id,
            position,
            barycenter,
            orbit,
            moons_ids,
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct MoonsIds(pub HashSet<String>);

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

// impl PlanetBarycenter {
    // pub fn update_barycenter(&mut self, other_id: &PlanetId, position: &PlanetPosition, other_position: &PlanetPosition) {
    //     if let Some(barycenter_id) = &self.id {
    //         if barycenter_id == &other_id.0 {
    //             let distance = other_position.0.distance(position.0);                
    //             let direction = (other_position.0 - position.0).normalize();
    //             if distance > COALESCE_DISTANCE {
    //                 self.position += direction * UPDATE_RATE_BARYCENTER;
    //             }
    //         }
    //     }
    //     // match &barycenter.id, &barycenter2.id) {
    //     //     (Some(barycenter_id), None) => if &id2.0 == barycenter_id { barycenter.position = position2.0 },
    //     //     (None, Some(barycenter_id)) => if &id.0 == barycenter_id { barycenter2.position = position.0 },
    //     //     _ => ()
    //     // }
    // }
// }

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

    pub fn update_orbit(&mut self, target_a: f32) {
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
    mut query: Query<(
        &mut PlanetPosition,
        &PlanetBarycenter,
        &Orbit,
        &mut Transform,
    )>,
) {
    let passed_time = time.seconds_since_startup();
    for (mut planet_position, barycenter, orbit, mut transform) in query.iter_mut() {
        planet_position.update_position(barycenter, orbit, passed_time);
        transform.translation.x = planet_position.0.x;
        transform.translation.z = planet_position.0.z;
    }
}

fn update_barycenters_system(mut query: Query<(
    &PlanetId,
    &MoonsIds,
    &PlanetPosition,
    &PlanetBarycenter,
)>) {

}

pub fn udpate_planet_mesh_from_planetesimal(mesh_handle: &Handle<Mesh>, mut meshes: ResMut<Assets<Mesh>>, planetesimal: &Planetesimal) {
    if let Some(mesh) = meshes.get_mut(mesh_handle) {
        let next_mesh = Mesh::from(shape::Icosphere {
            radius: planetesimal.radius as f32 * PLANET_RADIUS_SCALE_FACTOR,
            subdivisions: 32,
        });
        mesh.clone_from(&next_mesh);
    }
}