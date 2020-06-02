import * as wasm from './wams_gif_bg.wasm';

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

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

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getObject(idx) { return heap[idx]; }

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
/**
* @param {Uint8Array} data
* @returns {any}
*/
export function decode(data) {
    var ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.decode(ptr0, len0);
    return takeObject(ret);
}

/**
*/
export function greet() {
    wasm.greet();
}

/**
*/
export class Dimension {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_dimension_free(ptr);
    }
    /**
    * @returns {number}
    */
    get width() {
        var ret = wasm.__wbg_get_dimension_width(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set width(arg0) {
        wasm.__wbg_set_dimension_width(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get heihgt() {
        var ret = wasm.__wbg_get_dimension_heihgt(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set heihgt(arg0) {
        wasm.__wbg_set_dimension_heihgt(this.ptr, arg0);
    }
}

export const __wbg_log_b1e953047d00f8ab = function(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
};

export const __wbg_alert_752897bd5b706a3b = function(arg0, arg1) {
    alert(getStringFromWasm0(arg0, arg1));
};

export const __wbindgen_json_parse = function(arg0, arg1) {
    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

