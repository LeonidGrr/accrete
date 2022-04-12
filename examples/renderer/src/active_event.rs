use accrete::events::*;
use bevy::prelude::*;
use crate::planet_model::{PlanetId, PlanetBarycenter, Orbit, PlanetPosition, udpate_planet_mesh_from_planetesimal, PlanetModel, MoonsIds};
use crate::simulation_state::SimulationState;
use crate::consts::{SCALE_FACTOR, COALESCE_DISTANCE, PLANET_RADIUS_SCALE_FACTOR};

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
        mut commands: Commands,
        time: Res<Time>,
        mut active_event: ResMut<ActiveEvent>,
        mut state: ResMut<SimulationState>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut query: Query<(&PlanetId, &PlanetPosition, &mut PlanetBarycenter, &mut Orbit, &mut MoonsIds, &Handle<Mesh>)>,
    ) {
        let mut resulting_status = active_event.status.clone();
        let mut mesh_to_update = None;

        if let Some(event) = &active_event.event {
            match event {
                AccreteEvent::PlanetesimalCreated(_, planet) => {
                    let passed_time = time.seconds_since_startup();
                    let mut planet_model = PlanetModel::from(planet);
                    planet_model.position.update_position(&planet_model.barycenter, &planet_model.orbit, passed_time);
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

                    resulting_status = ActiveEventStatus::Done;
                },
                AccreteEvent::PlanetesimalUpdated(_, planet) => {
                    for (planet_id, _, _, _, _, mesh_handle) in query.iter() {
                        if planet_id.0 == planet.id {
                            mesh_to_update = Some((mesh_handle, planet)); 
                            state.planets.insert(planet.id.to_owned(), planet.clone());
                            resulting_status = ActiveEventStatus::Done;
                        }
                    }
                }
                AccreteEvent::PlanetesimalsCoalesced(_, target_id, source_id, resulting_planet) => {
                    for (planet_id, _, _, mut orbit, _, _) in query.iter_mut() {
                        if target_id == &planet_id.0 || source_id == &planet_id.0 {
                            orbit.update_orbit(resulting_planet.a as f32 * SCALE_FACTOR);
                            if (resulting_planet.a as f32 - orbit.a).abs() < COALESCE_DISTANCE {
                                resulting_status = ActiveEventStatus::Approached;
                            }
                        }
                    }
                },
                AccreteEvent::PlanetesimalCaptureMoon(_, target_id, source_id, resulting_planet) => {
                    for (planet_id, _, _, mut orbit, _, _) in query.iter_mut() {
                        if target_id == &planet_id.0 || source_id == &planet_id.0 {
                            orbit.update_orbit(resulting_planet.a as f32 * SCALE_FACTOR);
                            if (resulting_planet.a as f32 - orbit.a).abs() < COALESCE_DISTANCE {
                                resulting_status = ActiveEventStatus::Approached;
                            }
                        }
                    }
                },
                _ => resulting_status = ActiveEventStatus::Done,
            }
        }

        if let Some((mesh_handle, planetesimal)) = mesh_to_update {
            udpate_planet_mesh_from_planetesimal(mesh_handle, meshes, planetesimal);
        }

        active_event.status = resulting_status;
    }

    fn approached(
        mut active_event: ResMut<ActiveEvent>,
        mut state: ResMut<SimulationState>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut query: Query<(&PlanetId, &PlanetPosition, &mut PlanetBarycenter, &mut Orbit, &mut MoonsIds, &Handle<Mesh>)>,
    ) {
        let mut mesh_to_remove = None;
        let mut mesh_to_update = None;
        let mut resulting_status = active_event.status.clone();
    
        if let Some(event) = &active_event.event {
            match event {
                AccreteEvent::PlanetesimalsCoalesced(_, target_id, source_id, resulting_planet) => {
                    let mut iter = query.iter_combinations_mut();
                    while let Some([(id, position, _, _, _, mesh_handle), (id2, position2, _, _, _, mesh_handle2)]) = iter.fetch_next() {
                        let first_is_source = &id.0 == source_id && &id2.0 == target_id;
                        let second_is_source = &id2.0 == source_id && &id.0 == target_id;
                        if first_is_source || second_is_source {
                            let distance = position.0.distance(position2.0);

                            if distance <= COALESCE_DISTANCE {
                                if first_is_source {
                                    mesh_to_remove = Some(mesh_handle);
                                    mesh_to_update = Some((mesh_handle2, resulting_planet));                                    
                                }

                                if second_is_source {
                                    mesh_to_remove = Some(mesh_handle2);
                                    mesh_to_update = Some((mesh_handle, resulting_planet)); 
                                }

                                state.planets.remove(source_id);
                                state.planets.insert(target_id.to_string(), resulting_planet.clone());
                                resulting_status = ActiveEventStatus::Executed;
                            }
                        }
                    }
                },
                AccreteEvent::PlanetesimalCaptureMoon(_, target_id, source_id, resulting_planet) => {
                    let mut iter = query.iter_combinations_mut();
                    while let Some([(id, position, mut barycenter, _, mut moonsIds, _), (id2, position2, mut barycenter2, _, mut moonsIds2, _)]) = iter.fetch_next() {
                        let first_is_source = &id.0 == source_id && &id2.0 == target_id;
                        let second_is_source = &id2.0 == source_id && &id.0 == target_id;
                        if first_is_source || second_is_source {
                            let distance = position.0.distance(position2.0);
                            // let resulting_moon = resulting_planet.moons.iter().find(|m| &m.id == source_id).expect("Failed to dinf resulting moon");

                            if distance <= COALESCE_DISTANCE {
                                if first_is_source {
                                    barycenter.id = Some(target_id.to_string());
                                    moonsIds2.0.insert(source_id.to_string());
                                }
                                if second_is_source {
                                    barycenter2.id = Some(target_id.to_string());
                                    moonsIds.0.insert(source_id.to_string());
                                }
                            }
                        }
                    }
                },
                _ => (),
            }
        }
        if let Some(mesh_handle) = mesh_to_remove {
            meshes.remove(mesh_handle);
        }

        if let Some((mesh_handle, planetesimal)) = mesh_to_update {
            udpate_planet_mesh_from_planetesimal(mesh_handle, meshes, planetesimal);
        }

        active_event.status = resulting_status;
    }
}

pub fn active_event_system(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<SimulationState>,
    mut active_event: ResMut<ActiveEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&PlanetId, &PlanetPosition, &mut PlanetBarycenter, &mut Orbit, &mut MoonsIds, &Handle<Mesh>)>,
) {
    match &active_event.status.clone() {
        ActiveEventStatus::Created => ActiveEvent::created(commands, time, active_event, state, meshes, materials, query),
        ActiveEventStatus::Approached => ActiveEvent::approached(active_event, state, meshes, query),
        ActiveEventStatus::Executed => (),
        ActiveEventStatus::Done => (),
    }
}
