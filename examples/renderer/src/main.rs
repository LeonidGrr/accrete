mod coalescence;
mod orbit;
mod planet_model;
mod render;
mod state;

use crate::render::Render;
use accrete::events::{AccreteEvent, EVENTS};
use accrete::Accrete;
use macroquad::prelude::*;
use state::State;

#[macroquad::main("Accrete")]
async fn main() {
    let mut accrete = Accrete::new(1);
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

        system.primary_star.render();
        for p in state.planet_models.iter_mut() {
            p.render();
            // p.orbit.render();
        }

        let current_event = &log[state.event_idx];
        if state.event_idx < log.len() - 1 && !state.event_lock {
            state.event_idx += 1;
            state.event_handler(current_event, passed);
        }

        set_default_camera();

        if state.event_idx > 0 {
            let last_event = &log[state.event_idx - 1];
            draw_text(last_event.name(), 10.0, 20.0, 30.0, WHITE);
        }

        next_frame().await;
    }
}
