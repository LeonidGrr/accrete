use crate::consts::{COLLISION_DISTANCE, UPDATE_A_LIMIT};
use crate::planet_model::{Orbit, PlanetId, PlanetModel, PlanetPosition};
use crate::ring_model::RingModel;
use crate::simulation_state::SimulationState;
use crate::surface::get_planet_color;
use accrete::{events::*, PrimaryStar};
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveEventStatus {
    Created,
    Approached,
    Executed,
    Done,
}

#[derive(Debug, Component)]
pub struct ActiveEvent {
    pub event: Option<AccreteEvent>,
    pub status: ActiveEventStatus,
}

impl Default for ActiveEvent {
    fn default() -> Self {
        ActiveEvent {
            event: None,
            status: ActiveEventStatus::Done,
        }
    }
}

impl From<&AccreteEvent> for ActiveEvent {
    fn from(accrete_event: &AccreteEvent) -> Self {
        ActiveEvent {
            event: Some(accrete_event.clone()),
            status: ActiveEventStatus::Created,
        }
    }
}

impl ActiveEvent {
    fn created(
        &mut self,
        mut commands: Commands,
        primary_star: Res<PrimaryStar>,
        mut state: ResMut<SimulationState>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut query: Query<(
            Entity,
            &PlanetId,
            &mut PlanetPosition,
            &mut Orbit,
            &Handle<Mesh>,
            &Handle<StandardMaterial>,
            &mut Visibility,
        )>,
    ) {
        if let Some(event) = &self.event {
            match event {
                AccreteEvent::PlanetesimalCreated(_, planet) => {
                    let mut planet_model = PlanetModel::new(planet, &primary_star);
                    planet_model
                        .position
                        .update_position(&mut planet_model.orbit, state.current_step);
                    state.planets.insert(planet.id.to_owned(), planet.clone());
                    let color = get_planet_color(&planet);

                    commands
                        .spawn()
                        .insert_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius: Orbit::scaled_radius(planet.radius),
                                subdivisions: 32,
                            })),
                            material: materials.add(color.into()),
                            transform: Transform::from_translation(planet_model.position.0),
                            visibility: Visibility { is_visible: false },
                            ..default()
                        })
                        .insert_bundle(planet_model);

                    self.status = ActiveEventStatus::Executed;
                }
                AccreteEvent::PlanetesimalUpdated(_, planet) => {
                    for (
                        _,
                        planet_id,
                        _,
                        mut planet_orbit,
                        mesh_handle,
                        material_handle,
                        mut visibility,
                    ) in query.iter_mut()
                    {
                        if planet_id.0 == planet.id {
                            let resulting_planet_a = Orbit::scaled_a(planet.a);
                            planet_orbit.update_orbit(
                                resulting_planet_a,
                                planet.e,
                                planet.mass,
                                primary_star.stellar_mass,
                            );

                            if (resulting_planet_a - planet_orbit.a) < UPDATE_A_LIMIT {
                                PlanetModel::update_planet_resources(
                                    mesh_handle,
                                    material_handle,
                                    &mut meshes,
                                    &mut materials,
                                    planet,
                                );
                                visibility.is_visible = true;
                                state.planets.insert(planet.id.to_owned(), planet.clone());
                                planet_orbit.update_orbit_immediate(
                                    resulting_planet_a,
                                    planet.e,
                                    planet.mass,
                                    primary_star.stellar_mass,
                                );
                                self.status = ActiveEventStatus::Executed;
                            }
                        }
                    }
                }
                AccreteEvent::MoonsCoalesced(_, target_id, source_id, resulting_planet)
                | AccreteEvent::PlanetesimalsCoalesced(_, target_id, source_id, resulting_planet)
                | AccreteEvent::PlanetesimalCaptureMoon(
                    _,
                    target_id,
                    source_id,
                    resulting_planet,
                ) => {
                    state.cache_planets(&mut query, source_id, target_id);

                    if let Some((moon_entity, planet_entity)) = state.cached_planets {
                        let [moon, planet] = query
                            .get_many_mut([moon_entity, planet_entity])
                            .expect("Failed to retrieve cached planets by enitities");
                        let (_, moon_id, moon_position, mut moon_orbit, _, _, _) = moon;
                        let (_, planet_id, planet_position, mut planet_orbit, _, _, _) = planet;

                        let moon_data = state.planets.get(&moon_id.0).expect("Failed to find moon");
                        let planet_data = state
                            .planets
                            .get(&planet_id.0)
                            .expect("Failed to find planet");

                        let resulting_planet_a = Orbit::scaled_a(resulting_planet.a);

                        moon_orbit.update_orbit(
                            resulting_planet_a,
                            resulting_planet.e,
                            moon_data.mass,
                            primary_star.stellar_mass,
                        );

                        planet_orbit.update_orbit(
                            resulting_planet_a,
                            resulting_planet.e,
                            planet_data.mass,
                            primary_star.stellar_mass,
                        );

                        let planet_to_moon_distance = planet_position.0.distance(moon_position.0);
                        let approach_limit =
                            match resulting_planet.moons.iter().find(|m| m.id == moon_id.0) {
                                Some(moon) => Orbit::scaled_a(moon.a),
                                None => COLLISION_DISTANCE,
                            };

                        if planet_to_moon_distance <= approach_limit {
                            self.status = ActiveEventStatus::Approached;
                        }
                    }
                }
                AccreteEvent::PlanetesimalMoonToRing(_, moon_id, planet_id, resulting_ring) => {
                    state.cache_planets(&mut query, moon_id, planet_id);

                    if let Some((moon_entity, planet_entity)) = state.cached_planets {
                        let [moon, planet] = query
                            .get_many_mut([moon_entity, planet_entity])
                            .expect("Failed to retrieve cached planets by enitities");
                        let (moon_entity, _, _, _, mesh_handle, material_handle, _) = moon;
                        let (planet_entity, _, _, _, _, _, _) = planet;
                        RingModel::create_ring_mesh(
                            &mut commands,
                            planet_entity,
                            moon_entity,
                            mesh_handle,
                            material_handle,
                            resulting_ring,
                            &mut meshes,
                            &mut materials,
                        );
                        self.status = ActiveEventStatus::Executed;
                    }
                }
                _ => self.status = ActiveEventStatus::Executed,
            }
        }
    }

    fn approached(
        &mut self,
        mut commands: Commands,
        primary_star: Res<PrimaryStar>,
        mut state: ResMut<SimulationState>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut query: Query<(
            Entity,
            &PlanetId,
            &mut PlanetPosition,
            &mut Orbit,
            &Handle<Mesh>,
            &Handle<StandardMaterial>,
            &mut Visibility,
        )>,
    ) {
        if let Some(event) = &self.event {
            match event {
                AccreteEvent::MoonsCoalesced(_, _, _, resulting_planet)
                | AccreteEvent::PlanetesimalsCoalesced(_, _, _, resulting_planet) => {
                    if let Some((moon_entity, planet_entity)) = state.cached_planets {
                        let [moon, planet] = query
                            .get_many_mut([moon_entity, planet_entity])
                            .expect("Failed to retrieve cached planets by enitities");
                        let (moon_entity, moon_id, _, _, moon_mesh_handle, moon_material_handle, _) =
                            moon;
                        let (
                            _,
                            planet_id,
                            _,
                            mut planet_orbit,
                            planet_mesh_handle,
                            planet_material_handle,
                            _,
                        ) = planet;

                        commands.entity(moon_entity).despawn();
                        materials.remove(moon_material_handle);
                        meshes.remove(moon_mesh_handle);
                        PlanetModel::update_planet_resources(
                            planet_mesh_handle,
                            planet_material_handle,
                            &mut meshes,
                            &mut materials,
                            resulting_planet,
                        );

                        state.planets.remove(&moon_id.0);
                        state
                            .planets
                            .insert(planet_id.0.to_string(), resulting_planet.clone());

                        planet_orbit.update_orbit(
                            Orbit::scaled_a(resulting_planet.a),
                            resulting_planet.e,
                            resulting_planet.mass,
                            primary_star.stellar_mass,
                        );

                        self.status = ActiveEventStatus::Executed;
                    }
                }
                AccreteEvent::PlanetesimalCaptureMoon(_, _, _, resulting_planet) => {
                    if let Some((moon_entity, planet_entity)) = state.cached_planets {
                        let [moon, planet] = query
                            .get_many_mut([moon_entity, planet_entity])
                            .expect("Failed to retrieve cached planets by enitities");

                        let (
                            moon_entity,
                            moon_id,
                            moon_position,
                            mut moon_orbit,
                            moon_mesh_handle,
                            moon_material_handle,
                            _,
                        ) = moon;
                        let (
                            planet_entity,
                            _,
                            planet_position,
                            mut planet_orbit,
                            planet_mesh_handle,
                            planet_material_handle,
                            _,
                        ) = planet;

                        let moon_data = state.planets.get(&moon_id.0).expect("Failed to find moon");
                        let resulting_moon = resulting_planet
                            .moons
                            .iter()
                            .find(|m| m.id == moon_id.0)
                            .expect("Failed to find resulting moon");
                        let resulting_planet_a = Orbit::scaled_a(resulting_planet.a);
                        let distance = moon_position.0.distance(planet_position.0);

                        moon_orbit.update_orbit_immediate(
                            distance,
                            moon_data.e,
                            moon_data.mass,
                            primary_star.stellar_mass,
                        );
                        planet_orbit.update_orbit_immediate(
                            resulting_planet_a,
                            resulting_planet.e,
                            resulting_planet.mass,
                            primary_star.stellar_mass,
                        );
                        commands.entity(planet_entity).add_child(moon_entity);
                        PlanetModel::update_planet_resources(
                            moon_mesh_handle,
                            moon_material_handle,
                            &mut meshes,
                            &mut materials,
                            resulting_moon,
                        );
                        PlanetModel::update_planet_resources(
                            planet_mesh_handle,
                            planet_material_handle,
                            &mut meshes,
                            &mut materials,
                            resulting_planet,
                        );
                        self.status = ActiveEventStatus::Executed;
                    }
                }
                _ => (),
            }
        }
    }

    fn executed(&mut self, mut state: ResMut<SimulationState>) {
        if let Some(event) = &self.event {
            match event {
                AccreteEvent::MoonsCoalesced(_, _, _, _)
                | AccreteEvent::PlanetesimalMoonToRing(_, _, _, _)
                | AccreteEvent::PlanetesimalsCoalesced(_, _, _, _)
                | AccreteEvent::PlanetesimalCaptureMoon(_, _, _, _) => {
                    state.clear_cahed_planets();
                }
                _ => (),
            }
        }
        self.status = ActiveEventStatus::Done;
    }
}

#[allow(unused_mut)]
pub fn active_event_system(
    mut commands: Commands,
    primary_star: Res<PrimaryStar>,
    mut state: ResMut<SimulationState>,
    mut active_event: ResMut<ActiveEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        Entity,
        &PlanetId,
        &mut PlanetPosition,
        &mut Orbit,
        &Handle<Mesh>,
        &Handle<StandardMaterial>,
        &mut Visibility,
    )>,
) {
    match &active_event.status {
        ActiveEventStatus::Created => {
            active_event.created(commands, primary_star, state, meshes, materials, query)
        }
        ActiveEventStatus::Approached => {
            active_event.approached(commands, primary_star, state, meshes, materials, query)
        }
        ActiveEventStatus::Executed => active_event.executed(state),
        ActiveEventStatus::Done => (),
    }
}
