use crate::orbit::OrbitalParameters;
use accrete::{consts::KM_PER_AU, Ring};
use bevy::{math::vec3, prelude::*};
use rand::Rng;

#[derive(Debug, Clone, Bundle)]
pub struct RingModel {
    pub ring_radius: RingRadius,
    pub radius: Radius,
    pub id: RingId,
}

impl From<&Ring> for RingModel {
    fn from(ring: &Ring) -> RingModel {
        RingModel {
            ring_radius: RingRadius(OrbitalParameters::scaled_radius(ring.width / 2.0)),
            radius: Radius(OrbitalParameters::scaled_radius(ring.a * KM_PER_AU)),
            id: RingId(ring.id.clone()),
        }
    }
}

impl RingModel {
    pub fn create_ring_resources(
        commands: &mut Commands,
        planet_entity: Entity,
        ring: &Ring,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let ring_model = RingModel::from(ring);
        let ring_entity = commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Torus {
                    ring_radius: ring_model.ring_radius.0,
                    radius: ring_model.radius.0,
                    ..default()
                })),
                material: materials.add(RingModel::get_ring_color().into()),
                transform: Transform::from_scale(vec3(1.0, 0.0001, 1.0)),
                ..default()
            })
            .insert_bundle(ring_model)
            .id();

        commands.entity(planet_entity).add_child(ring_entity);
    }

    pub fn get_ring_color() -> Color {
        let mut rng = rand::thread_rng();
        let rand_r = rng.gen_range(0.95..1.0);
        let rand_g = rng.gen_range(0.87..0.91);
        let rand_b = rng.gen_range(0.75..0.79);
        let rand_a = rng.gen_range(0.15..0.5);
        Color::rgba(rand_r, rand_g, rand_b, rand_a)
    }
}

#[derive(Debug, Clone, Component)]
pub struct RingId(pub String);

#[derive(Debug, Clone, Component)]
pub struct RingRadius(pub f32);

#[derive(Debug, Clone, Component)]
pub struct Radius(pub f32);
