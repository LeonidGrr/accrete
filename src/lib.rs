mod accrete;
mod astro;
mod consts;
mod dole_params;
mod dust;
mod planetismal;
use crate::accrete::Accrete;

fn run() {
    let gen = Accrete::new(0.58);
    let s = gen.distribute_planets(Some(0.9), None);
    println!("Generated star system:");
    println!("{} planets", s.0.len());
    println!("Stellar mass: {}", s.1);
    println!("Stellar luminosity: {}", s.2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        run();
    }
}
