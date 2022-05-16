use crate::consts::{COLLISION_DISTANCE, SIMULATION_SPEED_IMMEDIATE_THRESHOLD, UPDATE_A_LIMIT};
use crate::orbit::Orbit;
use crate::planet_model::{
    PlanetData, PlanetId, PlanetModel, PlanetPosition, SourcePlanet, TargetPlanet,
};
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

fn event_initialized_system(
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
        &mut Orbit,
        &Handle<Polyline>,
        &mut Visibility,
        &mut PlanetData,
        &Children,
    )>,
) {
    if matches!(&active_event.status, ActiveEventStatus::Initialized) {
        let mut next_status = active_event.status.clone();
        if let Some(event) = &active_event.event {
            match event {
                AccreteEvent::OuterBodyInjected(_, planet)
                | AccreteEvent::PlanetesimalCreated(_, planet) => {
                    PlanetModel::create_planet(
                        &mut commands,
                        &mut state,
                        &mut meshes,
                        &mut materials,
                        &mut polyline_materials,
                        &mut polylines,
                        planet,
                        &primary_star,
                    );
                    next_status = ActiveEventStatus::Executed;
                }
                AccreteEvent::PlanetesimalUpdated(_, planet) => {
                    for (entity, planet_id, _, _, _, _, _, _) in query.iter_mut() {
                        if planet_id.0 == planet.id {
                            commands.entity(entity).insert(TargetPlanet);
                            next_status = ActiveEventStatus::InProgress;
                        }
                    }
                }
                AccreteEvent::MoonsCoalesced(_, target_id, source_id, _)
                | AccreteEvent::PlanetesimalMoonToRing(_, target_id, source_id, _)
                | AccreteEvent::PlanetesimalsCoalesced(_, target_id, source_id, _)
                | AccreteEvent::PlanetesimalCaptureMoon(_, target_id, source_id, _) => {
                    let mut iter = query.iter_combinations_mut();
                    while let Some(
                        [(entity1, id1, _, _, _, _, _, _), (entity2, id2, _, _, _, _, _, _)],
                    ) = iter.fetch_next()
                    {
                        match (&id1.0, &id2.0) {
                            (id1, id2) if id1 == source_id && id2 == target_id => {
                                commands.entity(entity1).insert(SourcePlanet);
                                commands.entity(entity2).insert(TargetPlanet);
                            }
                            (id1, id2) if id2 == source_id && id1 == target_id => {
                                commands.entity(entity1).insert(TargetPlanet);
                                commands.entity(entity2).insert(SourcePlanet);
                            }
                            _ => (),
                        }
                    }
                }
                _ => next_status = ActiveEventStatus::Executed,
            }
        }
        active_event.status = next_status;
    }
}

fn planetesimal_update_in_progress_system(
    primary_star: Res<PrimaryStar>,
    state: Res<SimulationState>,
    mut active_event: ResMut<ActiveEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        Entity,
        &PlanetId,
        &mut PlanetPosition,
        &mut Orbit,
        &Handle<Polyline>,
        &mut Visibility,
        &mut PlanetData,
        &Children,
    ), With<TargetPlanet>>,
    mut child_query: Query<
        (&Handle<Mesh>, &Handle<StandardMaterial>, &mut Visibility),
        Without<PlanetId>,
    >,
) {
    if !query.is_empty() && matches!(active_event.status, ActiveEventStatus::InProgress) {
        if let Some(event) = &active_event.event {
            match event {
                AccreteEvent::PlanetesimalUpdated(_, planet) => {
                    let (
                        _,
                        planet_id,
                        _,
                        mut planet_orbit,
                        _,
                        mut visibility,
                        mut planet_data,
                        children,
                    ) = query.single_mut();

                    if planet_id.0 == planet.id {
                        let resulting_planet_a = Orbit::scaled_a(planet.a);
                        planet_orbit.update_orbit(
                            resulting_planet_a,
                            planet.e,
                            planet.mass,
                            primary_star.stellar_mass,
                        );

                        let immediate =
                            state.simulation_speed > SIMULATION_SPEED_IMMEDIATE_THRESHOLD;
                        if (resulting_planet_a - planet_orbit.a) < UPDATE_A_LIMIT || immediate {
                            PlanetModel::update_planet_model(
                                children,
                                &mut child_query,
                                &mut visibility,
                                &mut planet_data,
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
                            active_event.status = ActiveEventStatus::Executed;
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

fn coalescence_in_progress_system(
    mut commands: Commands,
    primary_star: Res<PrimaryStar>,
    state: Res<SimulationState>,
    mut active_event: ResMut<ActiveEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    mut query: Query<(
        Entity,
        &PlanetId,
        &mut PlanetPosition,
        &mut Orbit,
        &Handle<Polyline>,
        &mut Visibility,
        &mut PlanetData,
        &Children,
        Or<(With<SourcePlanet>, With<TargetPlanet>)>,
    )>,
    mut child_query: Query<
        (&Handle<Mesh>, &Handle<StandardMaterial>, &mut Visibility),
        Without<PlanetId>,
    >,
) {
    if !query.is_empty() && matches!(active_event.status, ActiveEventStatus::InProgress) {
        if let Some(event) = &active_event.event {
            match event {
                AccreteEvent::MoonsCoalesced(_, _, _, resulting_planet)
                | AccreteEvent::PlanetesimalsCoalesced(_, _, _, resulting_planet) => {
                    let resulting_planet_a = Orbit::scaled_a(resulting_planet.a);
                    let mut iter = query.iter_combinations_mut();
                    if let Some([source, target]) = iter.fetch_next() {
                        let (
                            _,
                            source_id,
                            source_position,
                            mut source_orbit,
                            _,
                            mut source_visibility,
                            mut source_data,
                            children,
                            _,
                        ) = source;

                        source_orbit.update_orbit(
                            resulting_planet_a,
                            resulting_planet.e,
                            source_data.0.mass,
                            primary_star.stellar_mass,
                        );

                        let (
                            _,
                            target_id,
                            target_position,
                            mut target_orbit,
                            _,
                            mut target_visibility,
                            mut target_data,
                            children,
                            _,
                        ) = target;

                        target_orbit.update_orbit(
                            resulting_planet_a,
                            resulting_planet.e,
                            target_data.0.mass,
                            primary_star.stellar_mass,
                        );

                        let planet_to_moon_distance = target_position.0.distance(source_position.0);
                        let approach_limit =
                            match resulting_planet.moons.iter().find(|m| m.id == source_id.0) {
                                Some(moon) => Orbit::scaled_a(moon.a),
                                None => COLLISION_DISTANCE,
                            };
                    }

                    //     if planet_to_moon_distance <= approach_limit || immediate {
                    //         next_status = ActiveEventStatus::InProgress;
                    //     }
                    // }

                    // if let Some((moon_entity, planet_entity)) = state.cached_planets {
                    //     let [moon, planet] = query
                    //         .get_many_mut([moon_entity, planet_entity])
                    //         .expect("Failed to retrieve cached planets by enitities");
                    //     let (moon_entity, _, _, _, moon_polyline_handle, _, _, moon_children) =
                    //         moon;
                    //     let (_, _, _, mut planet_orbit, _, mut visibility, mut planet_data, planet_children) =
                    //         planet;

                    //     PlanetModel::remove_planet_resources(
                    //         moon_children,
                    //         &mut child_query,
                    //         moon_entity,
                    //         &mut commands,
                    //         &mut meshes,
                    //         &mut materials,
                    //     );

                    //     Orbit::remove_orbital_lines(moon_polyline_handle, &mut polylines);

                    //     PlanetModel::update_planet_model(
                    //         planet_children,
                    //         &mut child_query,
                    //         &mut visibility,
                    //         &mut planet_data,
                    //         &mut meshes,
                    //         &mut materials,
                    //         resulting_planet,
                    //     );

                    //     planet_orbit.update_orbit_immediate(
                    //         Orbit::scaled_a(resulting_planet.a),
                    //         resulting_planet.e,
                    //         resulting_planet.mass,
                    //         primary_star.stellar_mass,
                    //     );

                    //     next_status = ActiveEventStatus::Executed;
                    // }
                }
                // AccreteEvent::PlanetesimalCaptureMoon(_, _, _, resulting_planet) => {

                // if let Some((moon_entity, planet_entity)) = state.cached_planets {
                //     let [moon, planet] = query
                //         .get_many_mut([moon_entity, planet_entity])
                //         .expect("Failed to retrieve cached planets by enitities");
                //     let (_, moon_id, moon_position, mut moon_orbit, _, _, moon_data, _) = moon;
                //     let (_, _, planet_position, mut planet_orbit, _, _, planet_data, _) = planet;

                //     let resulting_planet_a = Orbit::scaled_a(resulting_planet.a);

                //     moon_orbit.update_orbit(
                //         resulting_planet_a,
                //         resulting_planet.e,
                //         moon_data.0.mass,
                //         primary_star.stellar_mass,
                //     );

                //     planet_orbit.update_orbit(
                //         resulting_planet_a,
                //         resulting_planet.e,
                //         planet_data.0.mass,
                //         primary_star.stellar_mass,
                //     );

                //     let immediate = state.simulation_speed > SIMULATION_SPEED_IMMEDIATE_THRESHOLD;
                //     let planet_to_moon_distance = planet_position.0.distance(moon_position.0);
                //     let approach_limit =
                //         match resulting_planet.moons.iter().find(|m| m.id == moon_id.0) {
                //             Some(moon) => Orbit::scaled_a(moon.a),
                //             None => COLLISION_DISTANCE,
                //         };

                //     if planet_to_moon_distance <= approach_limit || immediate {
                //         next_status = ActiveEventStatus::InProgress;
                //     }
                // }

                // if let Some((moon_entity, planet_entity)) = state.cached_planets {
                //     let [moon, planet] = query
                //         .get_many_mut([moon_entity, planet_entity])
                //         .expect("Failed to retrieve cached planets by enitities");

                //     let (
                //         moon_entity,
                //         moon_id,
                //         _,
                //         mut moon_orbit,
                //         _,
                //         mut moon_visibility,
                //         mut moon_data,
                //         moon_children,
                //     ) = moon;
                //     let (
                //         planet_entity,
                //         _,
                //         _,
                //         mut planet_orbit,
                //         _,
                //         mut planet_visibility,
                //         mut planet_data,
                //         planet_children,
                //     ) = planet;

                //     // let moon_data = state.planets.get(&moon_id.0).expect("Failed to find moon");
                //     let resulting_moon = resulting_planet
                //         .moons
                //         .iter()
                //         .find(|m| m.id == moon_id.0)
                //         .expect("Failed to find resulting moon");
                //     let resulting_planet_a = Orbit::scaled_a(resulting_planet.a);

                //     moon_orbit.update_orbit_immediate(
                //         resulting_moon.a as f32,
                //         resulting_moon.e,
                //         resulting_moon.mass,
                //         primary_star.stellar_mass,
                //     );

                //     planet_orbit.update_orbit_immediate(
                //         resulting_planet_a,
                //         resulting_planet.e,
                //         resulting_planet.mass,
                //         primary_star.stellar_mass,
                //     );

                //     commands.entity(planet_entity).add_child(moon_entity);

                //     PlanetModel::update_planet_model(
                //         moon_children,
                //         &mut child_query,
                //         &mut moon_visibility,
                //         &mut moon_data,
                //         &mut meshes,
                //         &mut materials,
                //         resulting_moon,
                //     );

                //     PlanetModel::update_planet_model(
                //         planet_children,
                //         &mut child_query,
                //         &mut planet_visibility,
                //         &mut planet_data,
                //         &mut meshes,
                //         &mut materials,
                //         resulting_planet,
                //     );

                //     next_status = ActiveEventStatus::Executed;
                // }
                // }

                // AccreteEvent::PlanetesimalMoonToRing(_, target_id, source_id, _) => {
                // if let Some((moon_entity, planet_entity)) = state.cached_planets {
                //     let [moon, planet] = query
                //         .get_many_mut([moon_entity, planet_entity])
                //         .expect("Failed to retrieve cached planets by enitities");
                //     let (moon_entity, _, _, _, moon_polyline_handle, _, _, children) = moon;
                //     let (planet_entity, _, _, _, _, _, _, _) = planet;
                //     RingModel::create_ring_resources(
                //         &mut commands,
                //         planet_entity,
                //         resulting_ring,
                //         &mut meshes,
                //         &mut materials,
                //     );

                //     PlanetModel::remove_planet_resources(
                //         children,
                //         &mut child_query,
                //         moon_entity,
                //         &mut commands,
                //         &mut meshes,
                //         &mut materials,
                //     );

                //     Orbit::remove_orbital_lines(moon_polyline_handle, &mut polylines);
                //     active_event.status = ActiveEventStatus::Executed;
                // }
                // }
                _ => (),
            }
        }
    }
}

fn event_executed_system(
    mut commands: Commands,
    mut query: Query<(Entity, AnyOf<(&SourcePlanet, &TargetPlanet)>)>,
    mut active_event: ResMut<ActiveEvent>,
) {
    if matches!(active_event.status, ActiveEventStatus::Executed) {
        query.for_each(|(entity, (marker1, marker2))| {
            if marker1.is_some() {
                commands.entity(entity).remove::<SourcePlanet>();
            }
            if marker2.is_some() {
                commands.entity(entity).remove::<TargetPlanet>();
            }
        });
        active_event.status = ActiveEventStatus::Done;
    }
}

pub struct ActiveEventPlugin;

impl Plugin for ActiveEventPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveEvent::default())
            .add_system(event_initialized_system)
            .add_system(planetesimal_update_in_progress_system)
            .add_system(coalescence_in_progress_system)
            .add_system(event_executed_system);
    }
}
