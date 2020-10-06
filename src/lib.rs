mod consts;
mod enviro;
mod structs;
mod utils;

pub mod accrete;
pub mod wasm;

#[cfg(test)]
mod tests {
    use crate::accrete::*;

    #[test]
    fn run_with_default_config() {
        let accrete = Accrete::new();
        accrete.planetary_system();
    }

    #[test]
    fn run_with_o_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 60.0;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_b_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 18.0;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_a_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 2.1;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_f_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 1.3;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_g_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 1.0;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_k_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 0.8;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_m_spectral_class() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 0.3;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_brown_dwarf() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 0.1;
        accrete.planetary_system();
    }

    #[test]
    fn run_with_rogue_planet() {
        let mut accrete = Accrete::new();
        accrete.stellar_mass = 0.0005;
        accrete.planetary_system();
    }

    // // "Even small increases in A result in large increases in the total mass of the systems produced; increasing A also decreases the average number of planets per system. As may be seen in Figure 17, for A = 0.003 and 0.006 the planetary system has become a binary star sys-tem, the body near 9 a.u. having grown large enough to be considered a red dwarf star. Observationally, the two stars of smallest mass now known are members of a binary system designated L726-8; each star has a mass estimated at about 0.04Ms (about 40 times the mass of Jupiter) or 13,000M e. The lower theoretical limit to the mass of a star is believed to be near 0.02Ms. It will be noticed that the binary star systems still contain numerous planetary bodies. As A is increased still more the systems become multiple-star systems and the number of planetary companions diminishes. Actually, the results at the higher values of A should be considered only suggestive of the general trend, since the total mass of the "planetary" bodies is now becoming fairly high with respect to that of the central body, so that the original simplifying assumptions, which were adequate when the total planetary mass was well below 0.01Ms, no longer apply so satisfactorily. The gravitational attractions of the several large masses for each other can no longer be considered to have negligible effects on the secular stability of the systems. This is pushing the ACRETE program somewhat beyond its original intent (to create planetary systems similar to the solar system). However, it would be readily possible to modify the program slightly to provide more rigorously for cases in which some of the planetary bodies grow to stellar mass. In any event, the general trend is clear. Simply increasing the value assigned to one parameter makes it possible to generate widely spaced binary and multiple-star systems."

    #[test]
    fn high_density_dust() {
        let mut accrete = Accrete::new();
        accrete.dust_density_coeff = 0.05;
        accrete.planetary_system();
    }

    #[test]
    fn low_density_dust() {
        let mut accrete = Accrete::new();
        accrete.dust_density_coeff = 0.00125;
        accrete.planetary_system();
    }

    #[test]
    fn high_cloud_ecentricity() {
        let mut accrete = Accrete::new();
        accrete.cloud_eccentricity = 0.5;
        accrete.planetary_system();
    }

    #[test]
    fn low_cloud_ecentricity() {
        let mut accrete = Accrete::new();
        accrete.cloud_eccentricity = 0.1;
        accrete.planetary_system();
    }

    #[test]
    fn low_cloud_ecentricity_and_dust_density() {
        let mut accrete = Accrete::new();
        accrete.cloud_eccentricity = 0.05;
        accrete.dust_density_coeff = 0.035;
        accrete.planetary_system();
    }

    #[test]
    fn random_planet_default() {
        let accrete = Accrete::new();
        accrete.planet();
    }
}
