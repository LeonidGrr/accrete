mod coalescence;
mod consts;
mod moon_capture;
mod orbit;
mod planet_model;
mod render;
mod state;

use crate::render::Render;
use accrete::events::{AccreteEvent, EVENTS};
use accrete::Accrete;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
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
    let mut timestamp = get_time();

    loop {
        clear_background(DARKGRAY);
        set_camera(&Camera3D {
            position: vec3(0.0, 0.0, 450.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        let passed_time = get_time();

        state.update_planets(passed_time);
        state.update_coalescences();
        state.update_moon_capture();

        system.primary_star.render();
        state.render();

        root_ui().window(
            hash!(),
            Vec2::new(20.0, 20.0),
            Vec2::new(200.0, 80.0),
            |ui| {
                let (mouse_x, mouse_y) = mouse_position();
                ui.label(None, &format!("Mouse position: {} {}", mouse_x, mouse_y));

                let (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
                ui.label(None, &format!("Mouse wheel x: {}", mouse_wheel_x));
                ui.label(None, &format!("Mouse wheel y: {}", mouse_wheel_y));

                if is_mouse_button_down(MouseButton::Left) {
                    if passed_time > (timestamp + 1.0)
                        && state.event_idx < log.len() - 1
                        && !state.event_lock
                    {
                        timestamp = passed_time;
                        state.event_handler(&log[state.event_idx], passed_time);
                        state.event_idx += 1;
                    }
                }
            },
        );

        if state.event_idx < log.len() - 1 && !state.event_lock {
            timestamp = passed_time;
            state.event_handler(&log[state.event_idx], passed_time);
            state.event_idx += 1;
        }

        set_default_camera();

        if state.event_idx > 0 {
            let last_event = &log[state.event_idx - 1];
            draw_text(
                format!("{} - {}", state.event_idx - 1, last_event.name()).as_str(),
                10.0,
                20.0,
                30.0,
                WHITE,
            );
        }

        next_frame().await;
    }
}
