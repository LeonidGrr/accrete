use accrete::events::AccreteEvent;
use bevy::prelude::*;

#[derive(Debug, Default)]
struct State {
    pub event_idx: usize,
    event_lock: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            event_idx: 0,
            event_lock: false,
        }
    }

    pub fn event_step(&mut self, passed_time: f64, log: &Vec<AccreteEvent>) {
        if passed_time > (self.event_idx as f64 + 1.0)
            && self.event_idx < log.len() - 1
            && !self.event_lock
        {
            self.event_idx += 1;
        }
    }
}

#[derive(Component)]
struct EventText;

pub fn run(log: Vec<AccreteEvent>) {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(State::new())
        .insert_resource(log)
        .add_startup_system(setup_event_system)
        .add_startup_system(setup)
        // .add_system(update_planets_position_system)
        // .add_system(render_planets_system)
        .add_system(render_event_system)
        .add_system(event_handler_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

// fn update_planets_position_system() {}

// fn update_coalescences_system() {}

// fn update_moon_captures_system() {}

fn event_handler_system(time: Res<Time>, mut state: ResMut<State>, log: Res<Vec<AccreteEvent>>) {
    let passed_time = time.seconds_since_startup();
    state.event_step(passed_time, &log);
}

fn setup_event_system(mut commands: Commands, state: Res<State>, asset_server: Res<AssetServer>) {
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
                "hello\nbevy!",
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

fn render_event_system(state: Res<State>, mut query: Query<&mut Text, With<EventText>>, log: Res<Vec<AccreteEvent>>) {
    let event_idx = state.event_idx;
    let last_event = &log[event_idx];
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{} - {}", event_idx, last_event.name());
    }
}

// fn render_planets_system() {}
