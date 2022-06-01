use crate::accrete::Accrete;
use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::*;


/// Generate planetary system from seed and primary star mass
#[wasm_bindgen]
pub fn planetary_system(seed: u64, stellar_mass: f64) -> JsValue {
    set_panic_hook();
    let mut accrete = Accrete::new(seed);
    accrete.stellar_mass = stellar_mass;
    let planetary_system = accrete.planetary_system();
    JsValue::from_serde(&planetary_system).unwrap()
}

/// Generate random planet from seed and primary star mass
#[wasm_bindgen]
pub fn planet(seed: u64, stellar_mass: f64) -> JsValue {
    set_panic_hook();
    let mut accrete = Accrete::new(seed);
    accrete.stellar_mass = stellar_mass;
    let planet = accrete.planet();
    JsValue::from_serde(&planet).unwrap()
}
