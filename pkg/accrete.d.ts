/* tslint:disable */
/* eslint-disable */
/**
* @param {BigInt} seed
* @returns {any}
*/
export function generate(seed: BigInt): any;
/**
* @param {any} accrete
* @returns {any}
*/
export function planetary_system(accrete: any): any;
/**
* @param {any} accrete
* @returns {any}
*/
export function planet(accrete: any): any;
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
*/
  b: number;
/**
*/
  cloud_eccentricity: number;
/**
*/
  dust_density_coeff: number;
/**
*/
  k: number;
/**
*/
  planet_a: number;
/**
*/
  planet_e: number;
/**
*/
  planet_mass: number;
/**
*/
  post_accretion_intensity: number;
/**
*/
  stellar_luminosity: number;
/**
*/
  stellar_mass: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly generate: (a: number, b: number) => number;
  readonly planetary_system: (a: number) => number;
  readonly planet: (a: number) => number;
  readonly __wbg_accrete_free: (a: number) => void;
  readonly __wbg_get_accrete_stellar_mass: (a: number) => number;
  readonly __wbg_set_accrete_stellar_mass: (a: number, b: number) => void;
  readonly __wbg_get_accrete_dust_density_coeff: (a: number) => number;
  readonly __wbg_set_accrete_dust_density_coeff: (a: number, b: number) => void;
  readonly __wbg_get_accrete_k: (a: number) => number;
  readonly __wbg_set_accrete_k: (a: number, b: number) => void;
  readonly __wbg_get_accrete_cloud_eccentricity: (a: number) => number;
  readonly __wbg_set_accrete_cloud_eccentricity: (a: number, b: number) => void;
  readonly __wbg_get_accrete_b: (a: number) => number;
  readonly __wbg_set_accrete_b: (a: number, b: number) => void;
  readonly __wbg_get_accrete_post_accretion_intensity: (a: number) => number;
  readonly __wbg_set_accrete_post_accretion_intensity: (a: number, b: number) => void;
  readonly __wbg_get_accrete_planet_a: (a: number) => number;
  readonly __wbg_set_accrete_planet_a: (a: number, b: number) => void;
  readonly __wbg_get_accrete_planet_e: (a: number) => number;
  readonly __wbg_set_accrete_planet_e: (a: number, b: number) => void;
  readonly __wbg_get_accrete_planet_mass: (a: number) => number;
  readonly __wbg_set_accrete_planet_mass: (a: number, b: number) => void;
  readonly __wbg_get_accrete_stellar_luminosity: (a: number) => number;
  readonly __wbg_set_accrete_stellar_luminosity: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
