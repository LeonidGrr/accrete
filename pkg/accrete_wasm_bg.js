import * as wasm from './accrete_wasm_bg.wasm';

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

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

let heap_next = heap.length;

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

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}
/**
* @returns {any}
*/
export function config() {
    var ret = wasm.config();
    return takeObject(ret);
}

/**
* @param {any} accrete
* @returns {any}
*/
export function planetary_system_wasm(accrete) {
    var ret = wasm.planetary_system_wasm(addHeapObject(accrete));
    return takeObject(ret);
}

/**
* @param {any} accrete
* @returns {any}
*/
export function planet_wasm(accrete) {
    var ret = wasm.planet_wasm(addHeapObject(accrete));
    return takeObject(ret);
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    };
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
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

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_accrete_free(ptr);
    }
    /**
    * @returns {number}
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
    * @returns {number}
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
    * @returns {number}
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
    * @returns {number}
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
    * @returns {number}
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
    * @returns {number}
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
    * @returns {number}
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
    * @returns {number}
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
    * @returns {number}
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
    * @returns {number}
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

export const __wbindgen_json_serialize = function(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = JSON.stringify(obj === undefined ? null : obj);
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

export const __wbindgen_json_parse = function(arg0, arg1) {
    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export const __wbg_new_59cb74e423758ede = function() {
    var ret = new Error();
    return addHeapObject(ret);
};

export const __wbg_stack_558ba5917b466edd = function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

export const __wbg_getRandomValues_f5e14ab7ac8e995d = function(arg0, arg1, arg2) {
    getObject(arg0).getRandomValues(getArrayU8FromWasm0(arg1, arg2));
};

export const __wbg_randomFillSync_d5bd2d655fdf256a = function(arg0, arg1, arg2) {
    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
};

export const __wbg_self_1b7a39e3a92c949c = handleError(function() {
    var ret = self.self;
    return addHeapObject(ret);
});

export const __wbg_require_604837428532a733 = function(arg0, arg1) {
    var ret = require(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export const __wbg_crypto_968f1772287e2df0 = function(arg0) {
    var ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};

export const __wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};

export const __wbg_getRandomValues_a3d34b4fee3c2869 = function(arg0) {
    var ret = getObject(arg0).getRandomValues;
    return addHeapObject(ret);
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

