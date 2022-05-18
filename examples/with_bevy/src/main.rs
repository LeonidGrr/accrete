mod active_event;
mod camera;
mod consts;
mod dust_model;
mod orbit;
mod planet_model;
mod rendering;
mod ring_model;
mod simulation_state;
mod stats;
mod surface;
mod ui;

use accrete::events::EVENTS;
use accrete::Accrete;
use rendering::run_simulation;
use stats::print_accrete_stats;

fn main() {
    let mut accrete = Accrete::new(2);
    accrete.post_accretion_intensity = 10;
    let system = accrete.planetary_system();
    let log = EVENTS.lock().expect("Failed to lock EVENTS mutex");
    print_accrete_stats(&system, &log);
    run_simulation(log.to_vec(), system.primary_star);
}
