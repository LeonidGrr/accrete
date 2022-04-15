use crate::consts::{COALESCE_DISTANCE_RATE, PLANET_RADIUS_SCALE_FACTOR, SCALE_FACTOR};
use crate::planet_model::{
    udpate_planet_mesh_from_planetesimal, Orbit, PlanetId, PlanetModel, PlanetPosition,
};
use crate::simulation_state::SimulationState;
use accrete::events::*;
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
        time: Res<Time>,
        mut state: ResMut<SimulationState>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut query: Query<(
            Entity,
            &PlanetId,
            &PlanetPosition,
            &mut Orbit,
            &Handle<Mesh>,
        )>,
    ) {
        let mut resulting_status = self.status.clone();
        let mut mesh_to_update = None;

        if let Some(event) = &self.event {
            match event {
                AccreteEvent::PlanetesimalCreated(_, planet) => {
                    let passed_time = time.seconds_since_startup();
                    let mut planet_model = PlanetModel::from(planet);
                    planet_model
                        .position
                        .update_position(&planet_model.orbit, passed_time);
                    state.planets.insert(planet.id.to_owned(), planet.clone());

                    commands
                        .spawn()
                        .insert_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius: planet.radius as f32 * PLANET_RADIUS_SCALE_FACTOR,
                                subdivisions: 32,
                            })),
                            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                            transform: Transform::from_translation(planet_model.position.0),
                            ..Default::default()
                        })
                        .insert_bundle(planet_model);

                    resulting_status = ActiveEventStatus::Executed;
                }
                AccreteEvent::PlanetesimalUpdated(_, planet) => {
                    for (_, planet_id, _, _, mesh_handle) in query.iter() {
                        if planet_id.0 == planet.id {
                            mesh_to_update = Some((mesh_handle, planet));
                            state.planets.insert(planet.id.to_owned(), planet.clone());
                            resulting_status = ActiveEventStatus::Executed;
                        }
                    }
                }
                AccreteEvent::PlanetesimalsCoalesced(_, target_id, source_id, resulting_planet)
                | AccreteEvent::PlanetesimalCaptureMoon(
                    _,
                    target_id,
                    source_id,
                    resulting_planet,
                ) => {
                    state.cache_planets(&mut query, source_id, target_id);

                    if let Some((moon_entity, planet_entity)) = state.cached_planets {
                        println!("got planets from cache");
                        {
                            let (_, _, _, mut moon_orbit, _) = query
                                .get_mut(moon_entity)
                                .expect("Failed to get moon components by entity");
                            moon_orbit
                                .update_orbit(resulting_planet.a as f32 * SCALE_FACTOR, false);
                        }
                        {
                            let (_, _, _, mut planet_orbit, _) = query
                                .get_mut(planet_entity)
                                .expect("Failed to get moon components by entity");
                            planet_orbit
                                .update_orbit(resulting_planet.a as f32 * SCALE_FACTOR, false);
                        }
                        {
                            let (_, _, _, moon_orbit, _) = query
                                .get(moon_entity)
                                .expect("Failed to get moon components by entity");
                            let (_, _, _, planet_orbit, _) = query
                                .get(planet_entity)
                                .expect("Failed to get moon components by entity");
                            if (resulting_planet.a as f32) < (moon_orbit.a * COALESCE_DISTANCE_RATE)
                                && (resulting_planet.a as f32)
                                    < (planet_orbit.a * COALESCE_DISTANCE_RATE)
                            {
                                resulting_status = ActiveEventStatus::Approached;
                            }
                        }
                    }
                }
                _ => resulting_status = ActiveEventStatus::Executed,
            }
        }

        if let Some((mesh_handle, planetesimal)) = mesh_to_update {
            udpate_planet_mesh_from_planetesimal(mesh_handle, &mut meshes, planetesimal);
        }

        self.status = resulting_status;
    }

    fn approached(
        &mut self,
        mut commands: Commands,
        mut state: ResMut<SimulationState>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut query: Query<(
            Entity,
            &PlanetId,
            &PlanetPosition,
            &mut Orbit,
            &Handle<Mesh>,
        )>,
    ) {
        let mut mesh_to_remove = None;
        let mut meshes_to_update = vec![];
        let mut resulting_status = self.status.clone();

        if let Some(event) = &self.event {
            match event {
                AccreteEvent::PlanetesimalsCoalesced(_, _, _, resulting_planet) => {
                    if let Some((moon_entity, planet_entity)) = state.cached_planets {
                        let (_, moon_id, moon_position, moon_orbit, moon_mesh_handle) = query
                            .get(moon_entity)
                            .expect("Failed to get moon components by entity");
                        let (_, planet_id, planet_position, planet_orbit, planet_mesh_handle) =
                            query
                                .get(planet_entity)
                                .expect("Failed to get moon components by entity");
                        let distance = moon_position.0.distance(planet_position.0);
                        let minimal_distance =
                            (moon_orbit.a - planet_orbit.a).abs() * COALESCE_DISTANCE_RATE;

                        if distance <= minimal_distance {
                            mesh_to_remove = Some(moon_mesh_handle);
                            meshes_to_update.push((planet_mesh_handle, resulting_planet));

                            state.planets.remove(&moon_id.0);
                            state
                                .planets
                                .insert(planet_id.0.to_string(), resulting_planet.clone());
                            resulting_status = ActiveEventStatus::Executed;
                        }
                    }
                }
                AccreteEvent::PlanetesimalCaptureMoon(_, _, _, resulting_planet) => {
                    if let Some((moon_entity, planet_entity)) = state.cached_planets {
                        let (planet_entity, _, planet_position, _, planet_mesh_handle) = query
                            .get(planet_entity)
                            .expect("Failed to get moon components by entity");
                        let (moon_entity, moon_id, moon_position, mut moon_orbit, moon_mesh_handle) =
                            query
                                .get_mut(moon_entity)
                                .expect("Failed to get moon components by entity");

                        let distance = moon_position.0.distance(planet_position.0);
                        let resulting_moon = resulting_planet
                            .moons
                            .iter()
                            .find(|m| m.id == moon_id.0)
                            .expect("Failed to find resulting moon");
                        let resulting_moon_a = resulting_moon.a as f32 * SCALE_FACTOR;

                        if distance <= moon_orbit.a * COALESCE_DISTANCE_RATE {
                            moon_orbit.update_orbit(distance, true);
                            commands.entity(planet_entity).add_child(moon_entity);
                        }

                        if moon_orbit.a > resulting_moon_a {
                            moon_orbit.update_orbit(resulting_moon_a, false);
                        } else {
                            meshes_to_update.push((moon_mesh_handle, resulting_moon));
                            meshes_to_update.push((planet_mesh_handle, resulting_planet));
                            resulting_status = ActiveEventStatus::Executed;
                        }
                    }
                }
                _ => (),
            }
        }
        if let Some(mesh_handle) = mesh_to_remove {
            meshes.remove(mesh_handle);
        }

        meshes_to_update
            .iter()
            .for_each(|(mesh_handle, planetesimal)| {
                udpate_planet_mesh_from_planetesimal(mesh_handle, &mut meshes, planetesimal);
            });

        self.status = resulting_status;
    }

    fn executed(&mut self, mut state: ResMut<SimulationState>) {
        if let Some(event) = &self.event {
            match event {
                AccreteEvent::PlanetesimalsCoalesced(_, _, _, _)
                | AccreteEvent::PlanetesimalCaptureMoon(_, _, _, _) => {
                    println!("Clear cache");
                    state.clear_cahed_planets();
                },
                _ => (),
            }
        }
        self.status = ActiveEventStatus::Done;
    }
}

#[allow(unused_mut)]
pub fn active_event_system(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<SimulationState>,
    mut active_event: ResMut<ActiveEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        Entity,
        &PlanetId,
        &PlanetPosition,
        &mut Orbit,
        &Handle<Mesh>,
    )>,
) {
    match &active_event.status {
        ActiveEventStatus::Created => {
            active_event.created(commands, time, state, meshes, materials, query)
        }
        ActiveEventStatus::Approached => active_event.approached(commands, state, meshes, query),
        ActiveEventStatus::Executed => active_event.executed(state),
        ActiveEventStatus::Done => (),
    }
}
