/* tslint:disable */
/* eslint-disable */
/**
* @returns {any}
*/
export function config(): any;
/**
* @param {any} accrete
* @returns {any}
*/
export function planetary_system_wasm(accrete: any): any;
/**
* @param {any} accrete
* @returns {any}
*/
export function planet_wasm(accrete: any): any;
/**
* ### Configuration:
*
* **stellar_mass** - Primary star mass in solar masses.
* *Default: random f64 in a range of 0.6-1.3 (corresponds main sequence spectral classes of F-G-K)*
*
* **dust_density_coeff** - "A" in Dole's paper, recommended range according to Dole's paper is 0.00125-0.0015, aslo noted that binary stars produced by increasing coeff of dust density in cloud (Formation of Planetary Systems by Aggregation: A Computer Simulation by Stephen H. Dole).
* *Default: 0.0015*
*
* **k** - The dust-to-gas ratio 50-100 (dust/gas = K), gas = hydrogen and helium, dust = other. Recommended range: 50.0-100.0
* *Default: 50.0*
*
* **cloud_eccentricity** - Initial dust cloud cloud_eccentricity. High eccentricity reduce number of planets. Recommended range: 0.15-0.25.
* *Default: 0.20*
*
* **b** - Crit_mass coeff is used as threshold for planet to become gas giant. Recommended range: 1.0e-5 - 1.2e-5
* *Default: 1.2e-5*
*
* **post_accretion_intensity** - Amount of random planetesimals that will bomb planets of created system after accretion.
* *Default: 1000*
*
* Parameters specific for standalone planet generation
* **planet_a** - Planet orbital radius in AU.
* *Default: random f64 in a range of 0.3-50.0*
*
* **planet_e** - Planet eccentricity
* *Default: f64 from random_eccentricity function*
*
* **planet_mass** - Planet mass in Earth masses.
* *Default: Random f64 in a range 3.3467202125167E-10 - 500.0*
*
* **stellar_luminosity** - Primary star luminosity.
* *Default: 1.0*
*/
export class Accrete {
  free(): void;
/**
* @returns {number}
*/
  b: number;
/**
* @returns {number}
*/
  cloud_eccentricity: number;
/**
* @returns {number}
*/
  dust_density_coeff: number;
/**
* @returns {number}
*/
  k: number;
/**
* @returns {number}
*/
  planet_a: number;
/**
* @returns {number}
*/
  planet_e: number;
/**
* @returns {number}
*/
  planet_mass: number;
/**
* @returns {number}
*/
  post_accretion_intensity: number;
/**
* @returns {number}
*/
  stellar_luminosity: number;
/**
* @returns {number}
*/
  stellar_mass: number;
}
