use accrete::events::AccreteEvent;
use accrete::System;

pub fn print_accrete_stats(system: &System, log: &[AccreteEvent]) {
    println!("Total {:#?} events.", log.len());

    let mut planetesimals = 0;
    let mut moon_capture = 0;
    let mut moons_to_rings = 0;
    let mut planetesimals_coalesce = 0;
    let mut moons_coalesce = 0;

    for event in log.iter() {
        match event {
            AccreteEvent::PlanetesimalsCoalesced(_, _, _, _) => planetesimals_coalesce += 1,
            AccreteEvent::MoonsCoalesced(_, _, _, _) => moons_coalesce += 1,
            AccreteEvent::PlanetesimalCreated(_, _) => planetesimals += 1,
            AccreteEvent::PlanetesimalCaptureMoon(_, _, _, _) => moon_capture += 1,
            AccreteEvent::PlanetesimalMoonToRing(_, _, _, _) => moons_to_rings += 1,
            _ => (),
        }
    }

    println!("Planetesimals coalesce: {:#?}", planetesimals_coalesce);
    println!("Moons coalesce: {:#?}", moons_coalesce);
    println!("Planetesimals created: {:#?}", planetesimals);
    println!("Planetesimals coalesce: {:#?}", planetesimals_coalesce);
    println!("Planets moons captured: {:#?}", moon_capture);
    println!("Moons coalesce: {:#?}", moons_coalesce);
    println!("Moons turned to rings: {:#?}", moons_to_rings);
    println!("Planets created: {:#?}", system.planets.len());
}
