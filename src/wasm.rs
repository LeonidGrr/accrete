use crate::accrete::Accrete;
use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate(seed: u64) -> JsValue {
    set_panic_hook();
    let accrete = Accrete::new(seed);
    JsValue::from_serde(&accrete).expect("Failed to serialize Accrete instance")
}

#[wasm_bindgen]
pub fn planetary_system(accrete: JsValue) -> JsValue {
    set_panic_hook();
    let mut accrete: Accrete = accrete.into_serde().unwrap();
    let planetary_system = accrete.planetary_system();
    JsValue::from_serde(&planetary_system).unwrap()
}

#[wasm_bindgen]
pub fn planet(accrete: JsValue) -> JsValue {
    set_panic_hook();
    let mut accrete: Accrete = accrete.into_serde().unwrap();
    let planet = accrete.planet();
    JsValue::from_serde(&planet).unwrap()
}
