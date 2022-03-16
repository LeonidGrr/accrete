
mod planet;
mod state;
mod orbit;

use accrete::events::{AccreteEvent, EVENTS};
use accrete::PrimaryStar;
use accrete::Accrete;
use macroquad::prelude::*;
use state::State;

#[macroquad::main("Accrete")]
async fn main() {
    let mut accrete = Accrete::new(33);
    let system = accrete.planetary_system();

    let log = EVENTS.lock().unwrap();
    println!("Total {:#?} events.", log.len());

    for event in log.iter() {
        match event {
            AccreteEvent::PlanetarySystemSetup(s1, _) => println!("{:#?}", s1),
            AccreteEvent::PlanetarySystemComplete(s1, _) => println!("{:#?}", s1),
            _ => (),
        }
    }

    let outer_a = match system.planets.last() {
        Some(p) => p.a as f32 * 3.3,
        None => 200.0,
    };

    let mut state = State {
        step: 1.0,
        event_idx: 0,
        current_event: &log[0],
        planets: vec![],
        scale_factor: screen_height() / outer_a,
    };
    let screen = (screen_width() / 2.0, screen_height() / 2.0);

    loop {
        clear_background(DARKGRAY);
        let passed = get_time();
        if passed > state.step * (state.event_idx + 1) as f64 {
            state.event_idx += 1;
            state.current_event = &log[state.event_idx];
            state.event_handler();
        }
    
        for p in state.planets.iter_mut() {
            p.render(&screen);
            p.orbit.render(&screen);      
        }

        render_star(&system.primary_star, &screen);

        draw_text(state.current_event.name(), 20.0, 20.0, 30.0, WHITE);

        next_frame().await;
    }
}

fn render_star(star: &PrimaryStar, screen: &(f32, f32)) {
    let [r, g, b] = star.color;
    let color = Color::new(r as f32, g as f32, b as f32, 1.0);
    draw_circle(screen.0, screen.1, 10.0, color);
}