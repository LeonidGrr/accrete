mod orbit;
mod planet;
mod render;
mod state;

use crate::render::Render;
use accrete::events::{AccreteEvent, EVENTS};
use accrete::Accrete;
use macroquad::prelude::*;
use state::State;

#[macroquad::main("Accrete")]
async fn main() {
    let mut accrete = Accrete::new(33);
    let system = accrete.planetary_system();

    let log = EVENTS.lock().unwrap();
    println!("Total {:#?} events.", log.len());
    let mut planetesimals = 0;
    for event in log.iter() {
        match event {
            AccreteEvent::PlanetarySystemSetup(s1, _) => println!("{:#?}", s1),
            AccreteEvent::PlanetesimalCreated(_, _) => planetesimals += 1,
            AccreteEvent::PlanetarySystemComplete(s1, _) => println!("{:#?}", s1),
            _ => (),
        }
    }

    println!("Planetesimals created: {:#?}", planetesimals);
    println!("Planets created: {:#?}", system.planets.len());

    let mut state = State {
        step: 1.0,
        event_idx: 0,
        current_event: &log[0],
        planets: vec![],
        dust: vec![],
    };

    loop {
        clear_background(DARKGRAY);
        set_camera(&Camera3D {
            position: vec3(0.0, 0.0, 450.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        let passed = get_time();
        if passed > state.step * (state.event_idx + 1) as f64 {
            state.event_idx += 1;
            state.current_event = &log[state.event_idx];
            state.event_handler();
        }

        for p in state.planets.iter_mut() {
            p.render();
            p.orbit.render();
        }

        system.primary_star.render();

        set_default_camera();
        draw_text(state.current_event.name(), 10.0, 20.0, 30.0, WHITE);

        next_frame().await;
    }
}
