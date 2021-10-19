pub mod accrete;
pub mod consts;
mod enviro;
mod structs;
mod utils;

pub use crate::accrete::Accrete;
pub use structs::DustBand;
pub use structs::Planetesimal;
pub use structs::PrimaryStar;
pub use structs::Ring;
pub use structs::System;

#[cfg(test)]
mod tests {
    use crate::Accrete;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn run_with_default_config() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let accrete = Accrete::new(&mut rng);
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn run_with_o_spectral_class() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.stellar_mass = 60.0;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn run_with_b_spectral_class() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.stellar_mass = 18.0;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn run_with_a_spectral_class() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.stellar_mass = 2.1;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn run_with_f_spectral_class() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.stellar_mass = 1.3;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn run_with_g_spectral_class() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.stellar_mass = 1.0;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn run_with_k_spectral_class() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.stellar_mass = 0.8;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn run_with_m_spectral_class() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.stellar_mass = 0.3;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn run_with_brown_dwarf() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.stellar_mass = 0.1;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn run_with_rogue_planet() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.stellar_mass = 0.0005;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn high_density_dust() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.dust_density_coeff = 0.05;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn low_density_dust() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.dust_density_coeff = 0.00125;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn high_cloud_ecentricity() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.cloud_eccentricity = 0.5;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn low_cloud_ecentricity() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.cloud_eccentricity = 0.1;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn low_cloud_ecentricity_and_dust_density() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut accrete = Accrete::new(&mut rng);
        accrete.cloud_eccentricity = 0.05;
        accrete.dust_density_coeff = 0.035;
        accrete.planetary_system(&mut rng);
    }

    #[test]
    fn random_planet_default() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let accrete = Accrete::new(&mut rng);
        accrete.planet(&mut rng);
    }
}
