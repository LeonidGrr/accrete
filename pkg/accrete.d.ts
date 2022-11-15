/* tslint:disable */
/* eslint-disable */
/**
* Generate planetary system from seed and primary star mass
* @param {bigint} seed
* @param {number} stellar_mass
* @returns {any}
*/
export function planetary_system(seed: bigint, stellar_mass: number): any;
/**
* Generate random planet from seed and primary star mass
* @param {bigint} seed
* @param {number} stellar_mass
* @returns {any}
*/
export function planet(seed: bigint, stellar_mass: number): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly planetary_system: (a: number, b: number) => number;
  readonly planet: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
