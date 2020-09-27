Accrete.rs
========================
Accrete is planetary system generation program using simulation of protoplanetary disk accretion.
While this particular version is not an accurate modelling tool, it can be used for procedural generation of plausible planetary system for game development or education.
During last 50 years this code was reimplemented many times with many improvements ([good overview here](https://github.com/zakski/accrete-starform-stargen), also brief history below).

## Features
- Planetary system generation from original Accrete.
- Moons and planetary rings generation.
- Extended stellar and planetary data.
- JSON output.

## Generate planetary system
### Default:
```rust
use accrete;

fn main() {
    let planets = accrete::run(None, None, None, None, None, None, None, false);
}
```
Simple way to variate output is to change stellar mass. This accrete implementation is capable of generating planetary system for any stellar mass, but better (most realistic) results achieved for main sequence star class with primary star mass of 0.6 - 1.3 solar masses.

### Configuration:
**planets_limit** - Limit number of planets.
*Default: None*

**stellar_mass** - Primary star mass in solar masses.
*Default: random f64 in a range of 0.6-1.3 (corresponds main sequence spectral classes of F-G-K)*

**dust_density_coeff** - "A" in Dole's paper, recommended range according to Dole's paper is 0.00125-0.0015, aslo noted that binary stars produced by increasing coeff of dust density in cloud (Formation of Planetary Systems by Aggregation: A Computer Simulation by Stephen H. Dole).
*Default: 0.0015*

**k** - The dust-to-gas ratio 50-100 (dust/gas = K), gas = hydrogen and helium, dust = other. Recommended range: 50.0-100.0
*Default: 50.0*

**cloud_eccentricity** - Initial dust cloud cloud_eccentricity. Recommended range: 0.15-0.25.
*Default: 0.20*

**b** - Crit_mass coeff is used as threshold for planet to become gas giant. Recommended range: 1.0e-5 - 1.2e-5
*Default: 1.2e-5*

**post_accretion_intensity** - Amount of random planetesimals that will bomb planets of created system after accretion.
*Default: 1000*

**to_json** - Output as JSON string. 
*Default: false*

## Brief history
>Accrete's origin dates back to the late 60's when Stephen H. Dole published "Formation of Planetary Systems by Aggregation: A Computer Simulation". 
>Almost a decade later Carl Sagan and Richard Isaacson refined Dole's model -- which shortly thereafter was also implemented in FORTRAN, and again elaborately and academically published by Martin Fogg in his paper "Extra-Solar Planetary Systems".
>The late 80's came and Matt Burdick brought this priceless program to the masses (via Turbo Pascal and C). Since then, many versions of Accrete have popped up around the internet, adding varying degrees of planetary specifics – the most notable (and ingenious) being Jim Burrow's implementation StarGen.(c)

## Papers
- [Dole, Stephen H., Formation of Planetary Systems by Aggregation: A Computer Simulation.. Santa Monica, CA: RAND Corporation, 1969.](https://www.rand.org/pubs/papers/P4226.html)
- [Dole, Stephen H., Habitable Planets for Man.](https://www.rand.org/content/dam/rand/pubs/commercial_books/2007/RAND_CB179-1.pdf)
- [Isaacman, R., Sagan, C. Cornell University, Ithaca, N.Y., Computer Simulations of Planetary Accretion Dynamics Sensitivity to Initial Condition.](https://ui.adsabs.harvard.edu/abs/1977Icar...31..510I/abstract)
- [Martyn J. Fogg, Extra Solar Planetary Systems A Microcomputer simulation.](https://www.academia.edu/4173808/Extra_Solar_Planetary_Systems_A_Microcomputer_Simulation)


## Acknowledgements
- Stephen H. Dole
- Carl Sagan
- Richard Isaacson
- Martin Fogg
- Matt Burdick
- Ian Burrell
- [Jim Burrows](http://www.eldacur.com/~brons/NerdCorner/StarGen/StarGen.html)
- [Zakski](https://github.com/zakski/accrete-starform-stargen)
- [Tmanderson](https://github.com/tmanderson/Accrete.js)
- [Kbingman](https://github.com/kbingman/accretejs)
- [Calebrob6](https://github.com/calebrob6/accrete)
- Many other good people!
