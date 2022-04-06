// mod coalescence;
mod consts;
// mod macroquad_render;
mod bevy_render;
// mod moon_capture;
// mod orbit;
// mod planet_model;
// mod render;
// mod state;

use accrete::events::{AccreteEvent, EVENTS};
use accrete::Accrete;
use bevy_render::run;

fn main() {
    let mut accrete = Accrete::new(2);
    let system = accrete.planetary_system();

    let log = EVENTS.lock().expect("Failed to lock EVENTS mutex");
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

    // macroquad_render::macroquad_render();
    run(log.to_vec());
}
