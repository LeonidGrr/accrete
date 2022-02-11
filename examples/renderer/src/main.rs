use accrete::events::{AccreteEvent, EVENTS};
use accrete::Accrete;

// TODO
fn main() {
    let mut accrete = Accrete::new(33);
    accrete.stellar_mass = 2.0;
    accrete.planetary_system();

    let log = EVENTS.lock().unwrap();
    println!("{:#?}", log.len());

    for event in log.iter() {
        match event {
            AccreteEvent::PlanetarySystemSetup(s1, _) => println!("{:#?}", s1),
            AccreteEvent::PlanetarySystemComplete(s1, _) => println!("{:#?}", s1),
            _ => (),
        }
    }
}
