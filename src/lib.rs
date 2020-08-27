mod accrete;
mod astro;
mod consts;
mod dole_params;
mod dust;
mod planetismal;
mod asteroid_belt;

use serde_json::json;

pub enum AccreteOutput {
    Tuple(accrete::Accrete),
    Json(String),
}

/// Run planetary system generator
pub fn run(
    with_moons: bool,
    with_rings: bool,
    _with_belts: bool,
    to_json: bool,
) -> AccreteOutput {
    let mut system = accrete::Accrete::new(None, None, None, None, None, None, None, None, None, with_moons, with_rings);
    system.distribute_planets();
    
    println!("{}, {}", system.stellar_mass, system.stellar_luminosity);
    for (i, p) in system.planets.iter().enumerate() {
        println!("Planet {}. {:#?}", i + 1, p);
        println!("------------------------------------");
    }

    if to_json {
        let accrete::Accrete {
            planets,
            stellar_mass,
            stellar_luminosity,
            ..
        } = system;
        let s = json!({
            "stellar_mass": stellar_mass,
            "stellar_luminosity": stellar_luminosity,
            "planets": planets,
        }).to_string();
    
        println!("{}", s);
        return AccreteOutput::Json(s);
    }

    AccreteOutput::Tuple(system)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_with_all_options_disabled() {
        run(false, false, false, false);
    }
    // #[test]
    // fn run_with_all_options_enabled_with_default_star() {
    //     run(true, true, true, false);
    // }
    // #[test]
    // fn run_with_json_output_with_default_star() {
    //     run(true, true, true, true);
    // }
    // #[test]
    // fn run_with_all_options_disabled_with_sun_mass_and_lumosity() {
    //     run(false, false, false, false);
    // }
}
