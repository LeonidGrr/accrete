use accrete::events::AccreteEvent;
use bevy::prelude::*;

use crate::simulation_state::SimulationState;

#[derive(Component)]
struct InfoText;

fn setup_event_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let mut text = Text::default();

    let event_number = TextSection {
        value: "Accrete simulation!\n".to_string(),
        style: TextStyle {
            font: asset_server.load("fonts/Cinzel-Regular.ttf"),
            font_size: 24.0,
            color: Color::WHITE,
        },
    };
    text.sections.push(event_number);

    let event_text = TextSection {
        value: "".to_string(),
        style: TextStyle {
            font: asset_server.load("fonts/Cinzel-Regular.ttf"),
            font_size: 24.0,
            color: Color::WHITE,
        },
    };
    text.sections.push(event_text);

    let step_text = TextSection {
        value: "Current step: 0".to_string(),
        style: TextStyle {
            font: asset_server.load("fonts/Cinzel-Regular.ttf"),
            font_size: 24.0,
            color: Color::WHITE,
        },
    };
    text.sections.push(step_text);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text,
            ..default()
        })
        .insert(InfoText);
}

fn render_info_system(
    state: Res<SimulationState>,
    mut query_event_text: Query<&mut Text, With<InfoText>>,
    log: Res<Vec<AccreteEvent>>,
) {
    let event_idx = state.event_idx;
    if event_idx > 0 {
        let last_event = &log[event_idx - 1];
        for mut text in query_event_text.iter_mut() {
            text.sections[0].value = format!("Event {}\n", event_idx);
            text.sections[1].value = format!("{}\n", last_event.name());
            text.sections[2].value = format!("Current step: {}", state.current_step);
        }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_event_system)
            .add_system(render_info_system);
    }
}
