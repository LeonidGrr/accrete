use accrete::event_store::{EVENT_STORE, AccreteEvent};
use accrete::Accrete;

fn main() {
    let mut accrete = Accrete::new(33);
    accrete.stellar_mass = 2.0;
    accrete.planetary_system();

    let log = EVENT_STORE.lock().unwrap();
    println!("{:#?}", log.len());

    for event in log.iter() {
        match event {
            AccreteEvent::PlanetarySystemSetup(s1, _) => println!("{:#?}", s1),
            AccreteEvent::PlanetarySystemComplete(s1, _) => println!("{:#?}", s1),
            _ => (),
        }
    }
}
