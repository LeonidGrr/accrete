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
    use std::fs;
    use std::io::{Error, Write};

    const GENERATE_FIXTURES: bool = false;

    fn write_to_file(data: &str, path: &str) -> Result<(), Error> {
        let mut output = fs::File::create(path)?;
        write!(output, "{}", data)
    }

    fn read_file(path: &str) -> String {
        fs::read_to_string(path).expect("Failed to read fixture")
    }

    fn get_fixture(path: &str, accrete: &mut Accrete) -> String {
        if GENERATE_FIXTURES {
            write_to_file(&format!("{:?}", accrete.planetary_system()), path)
                .expect("Failed to write fixture");
        }
        let fixture = read_file(path);
        fixture
    }

    #[test]
    fn run_with_default_config() {
        let mut accrete = Accrete::new(Default::default());
        let path = "./src/fixtures/default";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn run_with_o_spectral_class() {
        let mut accrete = Accrete::new(Default::default());
        accrete.stellar_mass = 60.0;
        accrete.planetary_system();
        let path = "./src/fixtures/o_spectral_class";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn run_with_b_spectral_class() {
        let mut accrete = Accrete::new(Default::default());
        accrete.stellar_mass = 18.0;
        accrete.planetary_system();
        let path = "./src/fixtures/b_spectral_class";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn run_with_a_spectral_class() {
        let mut accrete = Accrete::new(Default::default());
        accrete.stellar_mass = 2.1;
        accrete.planetary_system();
        let path = "./src/fixtures/a_spectral_class";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn run_with_f_spectral_class() {
        let mut accrete = Accrete::new(Default::default());
        accrete.stellar_mass = 1.3;
        accrete.planetary_system();
        let path = "./src/fixtures/f_spectral_class";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn run_with_g_spectral_class() {
        let mut accrete = Accrete::new(Default::default());
        accrete.stellar_mass = 1.0;
        accrete.planetary_system();
        let path = "./src/fixtures/g_spectral_class";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn run_with_k_spectral_class() {
        let mut accrete = Accrete::new(Default::default());
        accrete.stellar_mass = 0.8;
        accrete.planetary_system();
        let path = "./src/fixtures/k_spectral_class";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn run_with_m_spectral_class() {
        let mut accrete = Accrete::new(Default::default());
        accrete.stellar_mass = 0.3;
        accrete.planetary_system();
        let path = "./src/fixtures/m_spectral_class";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn run_with_brown_dwarf() {
        let mut accrete = Accrete::new(Default::default());
        accrete.stellar_mass = 0.1;
        accrete.planetary_system();
        let path = "./src/fixtures/brown_dwarf";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn run_with_rogue_planet() {
        let mut accrete = Accrete::new(Default::default());
        accrete.stellar_mass = 0.0005;
        accrete.planetary_system();
        let path = "./src/fixtures/rogue_planet";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn high_density_dust() {
        let mut accrete = Accrete::new(Default::default());
        accrete.dust_density_coeff = 0.05;
        accrete.planetary_system();
        let path = "./src/fixtures/high_density_dust";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn low_density_dust() {
        let mut accrete = Accrete::new(Default::default());
        accrete.dust_density_coeff = 0.00125;
        accrete.planetary_system();
        let path = "./src/fixtures/low_density_dust";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn high_cloud_ecentricity() {
        let mut accrete = Accrete::new(Default::default());
        accrete.cloud_eccentricity = 0.5;
        accrete.planetary_system();
        let path = "./src/fixtures/high_cloud_ecentricity";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn low_cloud_ecentricity() {
        let mut accrete = Accrete::new(Default::default());
        accrete.cloud_eccentricity = 0.1;
        accrete.planetary_system();
        let path = "./src/fixtures/low_cloud_ecentricity";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn low_cloud_ecentricity_and_dust_density() {
        let mut accrete = Accrete::new(Default::default());
        accrete.cloud_eccentricity = 0.05;
        accrete.dust_density_coeff = 0.035;
        accrete.planetary_system();
        let path = "./src/fixtures/low_cloud_ecentricity_and_dust_density";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }

    #[test]
    fn random_planet_default() {
        let mut accrete = Accrete::new(Default::default());
        accrete.planet();
        let path = "./src/fixtures/random_planet_default";
        let fixture = get_fixture(path, &mut accrete);
        let system = format!("{:?}", accrete.planetary_system());
        assert_eq!(system, fixture);
    }
}
