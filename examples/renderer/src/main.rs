mod coalescence;
mod consts;
mod moon_capture;
mod orbit;
mod planet_model;
mod render;
mod state;

use crate::render::Render;
use accrete::events::{AccreteEvent, EVENTS};
use macroquad::ui::{hash, root_ui, widgets};
use accrete::Accrete;
use macroquad::prelude::*;
use state::State;

#[macroquad::main("Accrete")]
async fn main() {
    let mut accrete = Accrete::new(2);
    let system = accrete.planetary_system();

    let log = EVENTS.lock().unwrap();
    println!("Total {:#?} events.", log.len());
    let mut planetesimals = 0;
    let mut coalescences = 0;
    for event in log.iter() {
        match event {
            AccreteEvent::PlanetarySystemSetup(s1, _) => println!("{:#?}", s1),
            AccreteEvent::PlanetesimalCreated(_, _) => planetesimals += 1,
            AccreteEvent::PlanetesimalsCoalesced(_, _, _, _) => coalescences += 1,
            AccreteEvent::PlanetarySystemComplete(s1, _) => println!("{:#?}", s1),
            _ => (),
        }
    }

    println!("Planetesimals created: {:#?}", planetesimals);
    println!("Planets coalesced: {:#?}", coalescences);
    println!("Planets created: {:#?}", system.planets.len());

    let mut state = State::new();

    loop {
        clear_background(DARKGRAY);
        set_camera(&Camera3D {
            position: vec3(0.0, 0.0, 450.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        let passed = get_time();
        state.update_planets(passed);
        state.update_coalescences();
        state.update_moon_capture();

        system.primary_star.render();
        state.render();

        root_ui().window(hash!(), Vec2::new(20., 20.), Vec2::new(450., 200.), |ui| {
            let (mouse_x, mouse_y) = mouse_position();
            ui.label(None, &format!("Mouse position: {} {}", mouse_x, mouse_y));

            let (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
            ui.label(None, &format!("Mouse wheel x: {}", mouse_wheel_x));
            ui.label(None, &format!("Mouse wheel y: {}", mouse_wheel_y));

            widgets::Group::new(hash!(), Vec2::new(200., 90.))
                .position(Vec2::new(240., 92.))
                .ui(ui, |ui| {
                    ui.label(None, "Pressed mouse keys");

                    if is_mouse_button_down(MouseButton::Left) {
                        ui.label(None, "Left");

                        if state.event_idx < log.len() - 1 && !state.event_lock {
                            state.event_handler(&log[state.event_idx], passed);
                            state.event_idx += 1;
                        }
                    }
                });
        });

        set_default_camera();

        if state.event_idx > 0 {
            let last_event = &log[state.event_idx - 1];
            draw_text(last_event.name(), 10.0, 20.0, 30.0, WHITE);
        }

        next_frame().await;
    }
}
