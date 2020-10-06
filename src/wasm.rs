use crate::accrete::*;
use crate::structs::planetesimal::Planetesimal;
use crate::structs::system::System;

use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn config() -> JsValue {
    set_panic_hook();
    let accrete = Accrete::default();
    JsValue::from_serde(&accrete).unwrap()
}

#[wasm_bindgen]
pub fn planetary_system_wasm(accrete: JsValue) -> JsValue {
    set_panic_hook();
    let Accrete {
        stellar_mass,
        dust_density_coeff,
        k,
        cloud_eccentricity,
        b,
        post_accretion_intensity,
        ..
    } = accrete.into_serde().unwrap();

    let mut planetary_system =
        System::set_initial_conditions(stellar_mass, dust_density_coeff, k, cloud_eccentricity, b);

    planetary_system.distribute_planetary_masses();
    planetary_system.post_accretion(post_accretion_intensity);
    planetary_system.process_planets();

    JsValue::from_serde(&planetary_system).unwrap()
}

#[wasm_bindgen]
pub fn planet_wasm(accrete: JsValue) -> JsValue {
    set_panic_hook();
    let Accrete {
        stellar_mass,
        stellar_luminosity,
        planet_a,
        planet_e,
        planet_mass,
        post_accretion_intensity,
        ..
    } = accrete.into_serde().unwrap();

    let planet = Planetesimal::random_planet(
        stellar_luminosity,
        stellar_mass,
        planet_a,
        planet_e,
        planet_mass,
        post_accretion_intensity,
    );

    JsValue::from_serde(&planet).unwrap()
}
