use accrete::events::AccreteEvent;
use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPlugin};

use crate::{active_event::ActiveEvent, simulation_state::SimulationState};

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
    active_event: Res<ActiveEvent>,
    mut query_event_text: Query<&mut Text, With<InfoText>>,
) {
    let event_name = match &active_event.event {
        Some(e) => e.name(),
        None => "",
    };
    let event_idx = state.event_idx;

    if event_idx > 0 {
        for mut text in query_event_text.iter_mut() {
            text.sections[0].value = format!("Event {}\n", event_idx);
            text.sections[1].value = format!("{} - {:?}\n", event_name, active_event.status);
            text.sections[2].value = format!("Current step: {}", state.current_step);
        }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_startup_system(setup_event_system)
            .add_startup_system(setup_custom_fonts_system)
            .add_system(render_info_system)
            .add_system(render_settings_system)
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

fn setup_custom_fonts_system(mut egui_context: ResMut<EguiContext>) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Cinzel-Regular.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    egui_context.ctx_mut().set_fonts(fonts);
}

fn render_settings_system(
    log: Res<Vec<AccreteEvent>>,
    mut state: ResMut<SimulationState>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Simulation settings").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Speed");
            ui.add(egui::Slider::new(&mut state.simulation_speed, 0.0..=100.0));
        });
        let progress = (state.event_idx + 1) as f32 / log.len() as f32;
        ui.horizontal(|ui| {
            ui.label("Progress");
            ui.add(
                egui::ProgressBar::new(progress).text(format!("{} %", (progress * 100.0) as usize)),
            );
        });
    });
}
