mod accrete;
mod astro;
mod consts;
mod dole_params;
mod dust;
mod planetismal;
mod asteroid_belt;

use serde_json::json;
use rand::prelude::*;

pub enum AccreteOutput {
    Tuple((Vec<planetismal::Planetismal>, f64, f64)),
    Json(String),
}

/// Run planetary system generator
pub fn run(
    with_moons: bool,
    with_rings: bool,
    _with_belts: bool,
    to_json: bool,
    stellar_mass: Option<f64>,
    stellar_luminosity: Option<f64>,
) -> AccreteOutput {
    let mut rng = rand::thread_rng();
    let stellar_mass = stellar_mass.unwrap_or(rng.gen_range(0.3, 1.2));
    let stellar_luminosity = stellar_luminosity.unwrap_or(astro::luminosity(stellar_mass));
    
    // let age = 
    // let main_seq_life = 
    // let r_ecosphere =
    // let r_greenhouse = 



    let gen = accrete::Accrete::new(with_moons, with_rings);
    let system = gen.distribute_planets(stellar_mass, stellar_luminosity);
    
    println!("{}, {}", stellar_mass, stellar_luminosity);
    for (i, p) in system.0.iter().enumerate() {
        println!("Planet {}. {:#?}", i + 1, p);
        println!("------------------------------------");
    }

    if to_json {
        let (planets, stellar_mass, stellar_luminosity) = system;
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
        run(false, false, false, false, None, None);
    }
    // #[test]
    // fn run_with_all_options_enabled_with_default_star() {
    //     run(true, true, true, false,None, None);
    // }
    // #[test]
    // fn run_with_json_output_with_default_star() {
    //     run(true, true, true, true, None, None);
    // }
    // #[test]
    // fn run_with_all_options_disabled_with_sun_mass_and_lumosity() {
    //     run(false, false, false, false,Some(1.0), Some(1.0));
    // }
}
