use crate::{
    consts::SCALE_FACTOR,
    planet_model::{Orbit, PlanetBarycenter, PlanetId, PlanetModel, PlanetPosition},
};
use accrete::{events::AccreteEvent, Planetesimal};
use bevy::{math::vec3, prelude::*};
use std::collections::HashMap;

// use crate::coalescence::CoalescenceOption;
// use crate::moon_capture::MoonCaptureOption;
// use crate::planet_model::PlanetModel;
// use accrete::events::AccreteEvent;
// use accrete::DustBand;
// use macroquad::prelude::*;
// use std::collections::HashMap;

// pub type PlanetModels = HashMap<String, PlanetModel>;

// pub struct SimulationState {
//     pub planet_models: PlanetModels,
//     pub coalescence: CoalescenceOption,
//     pub moon_capture: MoonCaptureOption,
//     pub dust_model: HashMap<String, DustBand>,
//     pub event_idx: usize,
//     pub event_lock: bool,
// }

// impl SimulationState {
//     pub fn new() -> Self {
//         SimulationState {
//             dt: 1.0,
//             event_idx: 0,
//             planet_models: HashMap::new(),
//             coalescence: CoalescenceOption::none(),
//             moon_capture: MoonCaptureOption::none(),
//             dust_model: HashMap::new(),
//             event_lock: false,
//         }
//     }

//     pub fn update_planets(&mut self, time: f64) {
//         for p in self.planet_models.values_mut() {
//             p.update_position(time);
//             p.update_a();
//         }
//     }

//     pub fn update_coalescences(&mut self) {
//         let SimulationState {
//             planet_models,
//             coalescence,
//             event_lock,
//             ..
//         } = self;
//         coalescence.update_status(planet_models, event_lock);
//     }

//     pub fn update_moon_capture(&mut self) {
//         let SimulationState {
//             planet_models,
//             moon_capture,
//             event_lock,
//             ..
//         } = self;
//         moon_capture.update_status(planet_models, event_lock);
//     }
// }

#[derive(Component)]
struct EventText;

#[derive(Debug, Default)]
pub struct SimulationState {
    pub event_idx: usize,
    pub event_lock: bool,
    pub planets: HashMap<String, Planetesimal>,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState {
            event_idx: 0,
            event_lock: false,
            planets: HashMap::new(),
        }
    }
}

fn event_handler_system(
    time: Res<Time>,
    mut state: ResMut<SimulationState>,
    mut commands: Commands,
    log: Res<Vec<AccreteEvent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let passed_time = time.seconds_since_startup();
    let current_event = &log[state.event_idx];
    if passed_time > (state.event_idx as f64 + 1.0)
        && state.event_idx < log.len() - 1
        && !state.event_lock
    {
        match current_event {
            AccreteEvent::PlanetarySystemSetup(_, _) => (),
            AccreteEvent::PlanetesimalCreated(_, planet) => {
                let Planetesimal { id, a, e, .. } = planet;
                let a = *a as f32 * SCALE_FACTOR;
                let planet_id = PlanetId(id.to_owned());
                let mut position = PlanetPosition(vec3(-(a - 0.001), 0.0, 0.0));
                let barycenter = PlanetBarycenter(vec3(0.0, 0.0, 0.0));
                let orbit = Orbit::new(a, *e as f32);

                position.update_position(&barycenter, &orbit, passed_time);
                state.planets.insert(id.to_owned(), planet.clone());
                commands
                    .spawn()
                    .insert_bundle(PlanetModel {
                        planet_id,
                        position,
                        barycenter,
                        orbit,
                    })
                    .insert_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                        transform: Transform::from_translation(position.0),
                        ..Default::default()
                    });
            }
            _ => (),
            // AccreteEvent::PlanetesimalUpdated(_, planet) => {
            //     if let Some(current_planet_model) = self.planet_models.get(&planet.id) {
            //         let mut next_planet_model = PlanetModel::new(planet.clone(), time);
            //         next_planet_model.position = current_planet_model.position;
            //         self.planet_models
            //             .insert(planet.id.clone(), next_planet_model);
            //     }
            // }
            // AccreteEvent::PlanetesimalToGasGiant(_, gas_giant) => {
            //     if let Some(current_planet_model) = self.planet_models.get(&gas_giant.id) {
            //         let mut next_planet_model = PlanetModel::new(gas_giant.clone(), time);
            //         next_planet_model.position = current_planet_model.position;
            //         self.planet_models
            //             .insert(gas_giant.id.clone(), next_planet_model);
            //     }
            // }
            // // AccreteEvent::DustBandsUpdated(_, _) => (),
            // AccreteEvent::PlanetesimalsCoalesced(_, source_planet_id, target_planet_id, result) => {
            //     self.coalescence = CoalescenceOption::new(
            //         source_planet_id,
            //         target_planet_id,
            //         result.clone(),
            //         time,
            //     );
            // }
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
            // _ => (),
        }
        state.event_idx += 1;
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
            .add_system(render_event_system);
    }
}
