mod active_event;
mod consts;
mod dust_model;
mod orbit;
mod planet_model;
mod rendering;
mod ring_model;
mod simulation_state;
mod surface;
mod ui;
mod stats;

use accrete::events::EVENTS;
use accrete::Accrete;
use stats::print_accrete_stats;
use rendering::run_simulation;

fn main() {
    let mut accrete = Accrete::new(2);
    accrete.post_accretion_intensity = 10;
    let system = accrete.planetary_system();
    let log = EVENTS.lock().expect("Failed to lock EVENTS mutex");
    print_accrete_stats(&system, &log);
    run_simulation(log.to_vec(), system.primary_star);
}
