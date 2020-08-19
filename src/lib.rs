mod accrete;
mod astro;
mod consts;
mod dole_params;
mod dust;
mod planetismal;
mod asteroid_belt;

use crate::accrete::Accrete;
use rand::prelude::*;

#[allow(dead_code)]
fn run() {
    let mut rng = rand::thread_rng();
    let gen = Accrete::new(true, true);
    let s = gen.distribute_planets(Some(rng.gen_range(0.3, 1.2)), None);

    println!("Generated star system:");
    println!("{} planets", s.0.len());
    println!("Stellar mass: {}", s.1);
    println!("Stellar luminosity: {}", s.2);

    for (i, planet) in s.0.iter().enumerate() {
        println!("Planet: {}", i + 1);
        println!("Mass: {}", planet.get_earth_mass());
        println!("Axis: {}", planet.axis);
        println!("Gas giant?: {}", planet.gas_giant);
        println!("Moons:");
        for (j, moon) in planet.moons.iter().enumerate() {
            println!("          Moon: {}", j+ 1);
            println!("          Mass: {}", moon.get_earth_mass());
            println!("          Axis: {}", moon.axis);
            println!("          Gas giant?: {}", moon.gas_giant);
            println!("          -----------");
        }
        println!("------------------------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        run();
    }
}
