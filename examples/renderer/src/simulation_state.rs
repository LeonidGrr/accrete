use crate::active_event::{active_event_system, ActiveEventStatus, ActiveEvent};
use crate::consts::{PLANET_RADIUS_SCALE_FACTOR, EVENT_TIME_SCALE};
use crate::planet_model::{PlanetId, PlanetModel};
use accrete::{events::AccreteEvent, Planetesimal};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
struct EventText;

#[derive(Debug, Default)]
pub struct SimulationState {
    pub event_idx: usize,
    pub planets: HashMap<String, Planetesimal>,
    pub active_event: ActiveEvent,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState {
            event_idx: 0,
            planets: HashMap::new(),
            active_event: ActiveEvent::default(),
        }
    }

    pub fn is_open(&self, passed_time: f64, total_events: usize) -> bool {
        let SimulationState { event_idx, .. } = *self;
        event_idx < total_events - 1
            && passed_time > (event_idx as f64 * EVENT_TIME_SCALE)
            && (self.active_event.status == ActiveEventStatus::None || self.active_event.status == ActiveEventStatus::Done)
    }
}

fn event_handler_system(
    mut commands: Commands,
    time: Res<Time>,
    log: Res<Vec<AccreteEvent>>,
    mut state: ResMut<SimulationState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&PlanetId, &Handle<Mesh>)>,
) {
    let passed_time = time.seconds_since_startup();
    let current_event = &log[state.event_idx];
    if state.is_open(passed_time, log.len()) {
        match current_event {
            AccreteEvent::PlanetesimalCreated(_, planet) => {
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
            }
            AccreteEvent::PlanetesimalUpdated(_, planet) => {
                for (planet_id, mesh_handle) in query.iter() {
                    if planet_id.0 == planet.id {
                        if let Some(mesh) = meshes.get_mut(mesh_handle) {
                            let next_mesh = Mesh::from(shape::Icosphere {
                                radius: planet.radius as f32 * PLANET_RADIUS_SCALE_FACTOR,
                                subdivisions: 32,
                            });
                            mesh.clone_from(&next_mesh);

                            state.planets.insert(planet.id.to_owned(), planet.clone());
                        }
                    }
                }
            }
            AccreteEvent::PlanetesimalCaptureMoon(_, _, _, _) |
            AccreteEvent::PlanetesimalsCoalesced(_, _, _, _) => state.active_event = ActiveEvent::from(current_event),
            // // AccreteEvent::MoonsCoalesced(_, source_moon_id, target_moon_id, result) => {},
            // AccreteEvent::PlanetesimalCaptureMoon(_, planet_id, moon_id, result) => {
            //     self.moon_capture =
            //         MoonCaptureOption::new(planet_id, moon_id, result.clone(), time);
            // }
            // // AccreteEvent::PlanetesimalMoonToRing(name, _) => name,
            // AccreteEvent::PostAccretionStarted(_) => self.event_lock = true,
            // // AccreteEvent::OuterBodyInjected(name, _) => name,
            // // AccreteEvent::PlanetaryEnvironmentGenerated(name, _) => name,
            // // AccreteEvent::PlanetarySystemComplete(_, _) => (),
            _ => (),
        }
        if state.active_event.status == ActiveEventStatus::None || state.active_event.status == ActiveEventStatus::Done {
            state.event_idx += 1;
        }
    }
}

fn setup_event_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Accrete\nsimulation!",
                TextStyle {
                    font: asset_server.load("fonts/Cinzel-Regular.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(EventText);
}

fn render_event_system(
    state: Res<SimulationState>,
    mut query: Query<&mut Text, With<EventText>>,
    log: Res<Vec<AccreteEvent>>,
) {
    let event_idx = state.event_idx;
    let last_event = &log[event_idx];
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{} - {}", event_idx, last_event.name());
    }
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_event_system)
            .add_system(event_handler_system)
            .add_system(active_event_system)
            .add_system(render_event_system);
    }
}
