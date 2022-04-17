// mod coalescence;
mod active_event;
mod consts;
mod planet_model;
mod rendering;
mod simulation_state;

use accrete::events::{AccreteEvent, EVENTS};
use accrete::Accrete;
use rendering::run;

fn main() {
    let mut accrete = Accrete::new(2);
    let system = accrete.planetary_system();

    let log = EVENTS.lock().expect("Failed to lock EVENTS mutex");
    println!("Total {:#?} events.", log.len());

    let mut planetesimals = 0;
    let mut moon_capture = 0;
    let mut planetesimals_coalesce = 0;
    let mut moons_coalesce = 0;

    for event in log.iter() {
        match event {
            AccreteEvent::PlanetesimalsCoalesced(_, _, _, _) => planetesimals_coalesce += 1,
            AccreteEvent::MoonsCoalesced(_, _, _, _) => moons_coalesce += 1,
            AccreteEvent::PlanetarySystemSetup(s1, _) => println!("{:#?}", s1),
            AccreteEvent::PlanetesimalCreated(_, _) => planetesimals += 1,
            AccreteEvent::PlanetesimalCaptureMoon(_, _, _, _) => moon_capture += 1,
            AccreteEvent::PlanetarySystemComplete(s1, _) => println!("{:#?}", s1),
            _ => (),
        }
    }

    println!("Planetesimals coalesce: {:#?}", planetesimals_coalesce);
    println!("Moons coalesce: {:#?}", moons_coalesce);
    println!("Planetesimals created: {:#?}", planetesimals);
    println!("Planets moon_capture: {:#?}", moon_capture);
    println!("Planets created: {:#?}", system.planets.len());

    run(log.to_vec());
}
