
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);
/**
* @param {BigInt} seed
* @returns {any}
*/
export function generate(seed) {
    uint64CvtShim[0] = seed;
    const low0 = u32CvtShim[0];
    const high0 = u32CvtShim[1];
    var ret = wasm.generate(low0, high0);
    return takeObject(ret);
}

/**
* @param {any} accrete
* @returns {any}
*/
export function planetary_system(accrete) {
    var ret = wasm.planetary_system(addHeapObject(accrete));
    return takeObject(ret);
}

/**
* @param {any} accrete
* @returns {any}
*/
export function planet(accrete) {
    var ret = wasm.planet(addHeapObject(accrete));
    return takeObject(ret);
}

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

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_accrete_free(ptr);
    }
    /**
    */
    get stellar_mass() {
        var ret = wasm.__wbg_get_accrete_stellar_mass(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set stellar_mass(arg0) {
        wasm.__wbg_set_accrete_stellar_mass(this.ptr, arg0);
    }
    /**
    */
    get dust_density_coeff() {
        var ret = wasm.__wbg_get_accrete_dust_density_coeff(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set dust_density_coeff(arg0) {
        wasm.__wbg_set_accrete_dust_density_coeff(this.ptr, arg0);
    }
    /**
    */
    get k() {
        var ret = wasm.__wbg_get_accrete_k(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set k(arg0) {
        wasm.__wbg_set_accrete_k(this.ptr, arg0);
    }
    /**
    */
    get cloud_eccentricity() {
        var ret = wasm.__wbg_get_accrete_cloud_eccentricity(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set cloud_eccentricity(arg0) {
        wasm.__wbg_set_accrete_cloud_eccentricity(this.ptr, arg0);
    }
    /**
    */
    get b() {
        var ret = wasm.__wbg_get_accrete_b(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set b(arg0) {
        wasm.__wbg_set_accrete_b(this.ptr, arg0);
    }
    /**
    */
    get post_accretion_intensity() {
        var ret = wasm.__wbg_get_accrete_post_accretion_intensity(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set post_accretion_intensity(arg0) {
        wasm.__wbg_set_accrete_post_accretion_intensity(this.ptr, arg0);
    }
    /**
    */
    get planet_a() {
        var ret = wasm.__wbg_get_accrete_planet_a(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set planet_a(arg0) {
        wasm.__wbg_set_accrete_planet_a(this.ptr, arg0);
    }
    /**
    */
    get planet_e() {
        var ret = wasm.__wbg_get_accrete_planet_e(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set planet_e(arg0) {
        wasm.__wbg_set_accrete_planet_e(this.ptr, arg0);
    }
    /**
    */
    get planet_mass() {
        var ret = wasm.__wbg_get_accrete_planet_mass(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set planet_mass(arg0) {
        wasm.__wbg_set_accrete_planet_mass(this.ptr, arg0);
    }
    /**
    */
    get stellar_luminosity() {
        var ret = wasm.__wbg_get_accrete_stellar_luminosity(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set stellar_luminosity(arg0) {
        wasm.__wbg_set_accrete_stellar_luminosity(this.ptr, arg0);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('accrete_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_json_parse = function(arg0, arg1) {
        var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_json_serialize = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = JSON.stringify(obj === undefined ? null : obj);
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_new_693216e109162396 = function() {
        var ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

