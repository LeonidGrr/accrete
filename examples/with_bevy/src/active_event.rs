use crate::consts::{COLLISION_DISTANCE, UPDATE_A_LIMIT};
use crate::orbit::{Orbit, OrbitalParameters};
use crate::planet_model::{PlanetId, PlanetModel, PlanetPosition};
use crate::ring_model::RingModel;
use crate::simulation_state::SimulationState;
use accrete::{events::*, PrimaryStar};
use bevy::prelude::*;
use bevy_polyline::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveEventStatus {
    Initialized,
    InProgress,
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
            status: ActiveEventStatus::Initialized,
        }
    }
}

impl ActiveEvent {
    fn initialized(
        &mut self,
        mut commands: Commands,
        primary_star: Res<PrimaryStar>,
        mut state: ResMut<SimulationState>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
        mut polylines: ResMut<Assets<Polyline>>,
        mut query: Query<(
            Entity,
            &PlanetId,
            &mut PlanetPosition,
            &mut OrbitalParameters,
            &Handle<Polyline>,
            &Handle<Mesh>,
            &Handle<StandardMaterial>,
            &mut Visibility,
        )>,
    ) {
        if let Some(event) = &self.event {
            match event {
                AccreteEvent::OuterBodyInjected(_, planet)
                | AccreteEvent::PlanetesimalCreated(_, planet) => {
                    PlanetModel::create_planet_resourses(
                        &mut commands,
                        &mut state,
                        &mut meshes,
                        &mut materials,
                        &mut polyline_materials,
                        &mut polylines,
                        planet,
                        &primary_star,
                    );
                    self.status = ActiveEventStatus::Executed;
                }
                AccreteEvent::PlanetesimalUpdated(_, planet) => {
                    for (
                        _,
                        planet_id,
                        _,
                        mut planet_orbit,
                        polyline_handle,
                        mesh_handle,
                        material_handle,
                        mut visibility,
                    ) in query.iter_mut()
                    {
                        if planet_id.0 == planet.id {
                            let resulting_planet_a = OrbitalParameters::scaled_a(planet.a);
                            planet_orbit.update_orbit(
                                resulting_planet_a,
                                planet.e,
                                planet.mass,
                                primary_star.stellar_mass,
                            );

                            let immediate = state.simulation_speed > 10.0;
                            if (resulting_planet_a - planet_orbit.a) < UPDATE_A_LIMIT || immediate {
                                PlanetModel::update_planet_resources(
                                    mesh_handle,
                                    material_handle,
                                    &mut visibility,
                                    &mut state,
                                    &mut meshes,
                                    &mut materials,
                                    planet,
                                );
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
                        let (
                            _,
                            moon_id,
                            moon_position,
                            mut moon_orbit,
                            moon_polyline_handle,
                            _,
                            _,
                            _,
                        ) = moon;
                        let (
                            _,
                            planet_id,
                            planet_position,
                            mut planet_orbit,
                            planet_polyline_handle,
                            _,
                            _,
                            _,
                        ) = planet;

                        let moon_data = state.planets.get(&moon_id.0).expect("Failed to find moon");
                        let planet_data = state
                            .planets
                            .get(&planet_id.0)
                            .expect("Failed to find planet");
                        let resulting_planet_a = OrbitalParameters::scaled_a(resulting_planet.a);

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

                        let immediate = state.simulation_speed > 10.0;
                        let planet_to_moon_distance = planet_position.0.distance(moon_position.0);
                        let approach_limit =
                            match resulting_planet.moons.iter().find(|m| m.id == moon_id.0) {
                                Some(moon) => OrbitalParameters::scaled_a(moon.a),
                                None => COLLISION_DISTANCE,
                            };

                        if planet_to_moon_distance <= approach_limit || immediate {
                            self.status = ActiveEventStatus::InProgress;
                        }
                    }
                }
                AccreteEvent::PlanetesimalMoonToRing(_, moon_id, planet_id, resulting_ring) => {
                    state.cache_planets(&mut query, moon_id, planet_id);

                    if let Some((moon_entity, planet_entity)) = state.cached_planets {
                        let [moon, planet] = query
                            .get_many_mut([moon_entity, planet_entity])
                            .expect("Failed to retrieve cached planets by enitities");
                        let (
                            moon_entity,
                            moon_id,
                            _,
                            _,
                            moon_polyline_handle,
                            moon_mesh_handle,
                            moon_material_handle,
                            _,
                        ) = moon;
                        let (planet_entity, _, _, _, _, _, _, _) = planet;
                        RingModel::create_ring_resources(
                            &mut commands,
                            planet_entity,
                            resulting_ring,
                            &mut meshes,
                            &mut materials,
                        );
                        PlanetModel::remove_planet_resources(
                            moon_entity,
                            moon_id,
                            moon_mesh_handle,
                            moon_material_handle,
                            &mut commands,
                            &mut state,
                            &mut meshes,
                            &mut materials,
                        );
                        Orbit::remove_orbital_lines_resources(moon_polyline_handle, &mut polylines);
                        self.status = ActiveEventStatus::Executed;
                    }
                }
                _ => self.status = ActiveEventStatus::Executed,
            }
        }
    }

    fn in_progress(
        &mut self,
        mut commands: Commands,
        primary_star: Res<PrimaryStar>,
        mut state: ResMut<SimulationState>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut polylines: ResMut<Assets<Polyline>>,
        mut query: Query<(
            Entity,
            &PlanetId,
            &mut PlanetPosition,
            &mut OrbitalParameters,
            &Handle<Polyline>,
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
                        let (
                            moon_entity,
                            moon_id,
                            _,
                            _,
                            moon_polyline_handle,
                            moon_mesh_handle,
                            moon_material_handle,
                            _,
                        ) = moon;
                        let (
                            _,
                            _,
                            _,
                            mut planet_orbit,
                            planet_polyline_handle,
                            planet_mesh_handle,
                            planet_material_handle,
                            mut visibility,
                        ) = planet;

                        PlanetModel::remove_planet_resources(
                            moon_entity,
                            moon_id,
                            moon_mesh_handle,
                            moon_material_handle,
                            &mut commands,
                            &mut state,
                            &mut meshes,
                            &mut materials,
                        );
                        Orbit::remove_orbital_lines_resources(moon_polyline_handle, &mut polylines);

                        PlanetModel::update_planet_resources(
                            planet_mesh_handle,
                            planet_material_handle,
                            &mut visibility,
                            &mut state,
                            &mut meshes,
                            &mut materials,
                            resulting_planet,
                        );

                        planet_orbit.update_orbit_immediate(
                            OrbitalParameters::scaled_a(resulting_planet.a),
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
                            moon_polyline_handle,
                            moon_mesh_handle,
                            moon_material_handle,
                            mut moon_visibility,
                        ) = moon;
                        let (
                            planet_entity,
                            _,
                            planet_position,
                            mut planet_orbit,
                            planet_polyline_handle,
                            planet_mesh_handle,
                            planet_material_handle,
                            mut planet_visibility,
                        ) = planet;

                        let moon_data = state.planets.get(&moon_id.0).expect("Failed to find moon");
                        let resulting_moon = resulting_planet
                            .moons
                            .iter()
                            .find(|m| m.id == moon_id.0)
                            .expect("Failed to find resulting moon");
                        let resulting_planet_a = OrbitalParameters::scaled_a(resulting_planet.a);
                        let distance = moon_position.0.distance(planet_position.0);

                        moon_orbit.update_orbit_immediate(
                            resulting_moon.a as f32,
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

                        // let polyline = polylines.get(moon_polyline_handle).expect("Failed to get moon polyline resource");
                        // commands.entity(planet_entity).add_child(polyline);

                        PlanetModel::update_planet_resources(
                            moon_mesh_handle,
                            moon_material_handle,
                            &mut moon_visibility,
                            &mut state,
                            &mut meshes,
                            &mut materials,
                            resulting_moon,
                        );
                        PlanetModel::update_planet_resources(
                            planet_mesh_handle,
                            planet_material_handle,
                            &mut planet_visibility,
                            &mut state,
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
                    state.clear_cached_planets();
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
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    mut query: Query<(
        Entity,
        &PlanetId,
        &mut PlanetPosition,
        &mut OrbitalParameters,
        &Handle<Polyline>,
        &Handle<Mesh>,
        &Handle<StandardMaterial>,
        &mut Visibility,
    )>,
) {
    match &active_event.status {
        ActiveEventStatus::Initialized => active_event.initialized(
            commands,
            primary_star,
            state,
            meshes,
            materials,
            polyline_materials,
            polylines,
            query,
        ),
        ActiveEventStatus::InProgress => active_event.in_progress(
            commands,
            primary_star,
            state,
            meshes,
            materials,
            polylines,
            query,
        ),
        ActiveEventStatus::Executed => active_event.executed(state),
        ActiveEventStatus::Done => (),
    }
}
