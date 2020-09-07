Accrete.rs
========================
Accrete is planetary system generation program.
During last 50 years this code was reimplemented many times with many improvements ([good overview here](https://github.com/zakski/accrete-starform-stargen), also brief history below).
While this particular version is not supposed to be used as accurate scientific modelling tool, it can be used for procedural generation of plausible planetary system.
This Rust port of accrete hopefully will include most of old features of this wonderful program and maybe even some extended functionality.


## Example
```rust
use accrete;

// To return JSON instead of struct just pass true to run function;
fn main() {
    let planets = accrete::run(false);
}
```

## Brief history
>Accrete's origin dates back to the late 60's when Stephen H. Dole published "Formation of Planetary Systems by Aggregation: A Computer Simulation". 
>Almost a decade later Carl Sagan and Richard Isaacson refined Dole's model -- which shortly thereafter was also implemented in FORTRAN, and again elaborately and academically published by Martin Fogg in his paper "Extra-Solar Planetary Systems".
>The late 80's came and Matt Burdick brought this priceless program to the masses (via Turbo Pascal and C). Since then, many versions of Accrete have popped up around the internet, adding varying degrees of planetary specifics – the most notable (and ingenious) being Jim Burrow's implementation StarGen.(c)

## Papers

- [Dole, Stephen H., Formation of Planetary Systems by Aggregation: A Computer Simulation.. Santa Monica, CA: RAND Corporation, 1969.](https://www.rand.org/pubs/papers/P4226.html)
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
