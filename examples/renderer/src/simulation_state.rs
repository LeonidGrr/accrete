use crate::active_event::{active_event_system, ActiveEvent, ActiveEventStatus};
use crate::consts::EVENT_TIME_SCALE;
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
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState {
            event_idx: 0,
            planets: HashMap::new(),
        }
    }

    pub fn is_open(
        &self,
        active_event: &Res<ActiveEvent>,
        passed_time: f64,
        total_events: usize,
    ) -> bool {
        let SimulationState { event_idx, .. } = *self;
        event_idx < total_events - 1
            && passed_time > (event_idx as f64 * EVENT_TIME_SCALE)
            && active_event.status == ActiveEventStatus::Done
    }
}

fn event_handler_system(
    mut commands: Commands,
    time: Res<Time>,
    active_event: Res<ActiveEvent>,
    log: Res<Vec<AccreteEvent>>,
    mut state: ResMut<SimulationState>,
) {
    let passed_time = time.seconds_since_startup();
    let current_event = &log[state.event_idx];
    if state.is_open(&active_event, passed_time, log.len()) {
        commands.insert_resource(ActiveEvent::from(current_event));
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
    if event_idx > 0 {
        let last_event = &log[event_idx - 1];
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{} - {}", event_idx, last_event.name());
        }
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
