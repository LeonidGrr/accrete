use accrete::event_store::EVENT_STORE;
use accrete::Accrete;

fn main() {
    let mut accrete = Accrete::new(33);
    accrete.stellar_mass = 2.0;
    let system = accrete.planetary_system();

    println!("{:#?}", EVENT_STORE.lock().unwrap());
}
