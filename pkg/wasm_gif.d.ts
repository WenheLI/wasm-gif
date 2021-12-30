/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} data
* @returns {DecodeFrames}
*/
export function decode(data: Uint8Array): DecodeFrames;
/**
* @param {number} width
* @param {number} height
* @param {Uint8Array} frames
* @param {Uint16Array} delays
* @param {Uint8Array} global_palette
* @param {number} mode
* @param {number | undefined} times
* @returns {Uint8Array}
*/
export function encode(width: number, height: number, frames: Uint8Array, delays: Uint16Array, global_palette: Uint8Array, mode: number, times?: number): Uint8Array;
/**
*/
export enum GifPlayMode {
  REPEAT,
  SINGLE,
}
/**
*/
export class DecodeFrames {
  free(): void;
/**
* @returns {Uint8Array}
*/
  readonly data: Uint8Array;
/**
* @returns {Uint16Array}
*/
  readonly delays: Uint16Array;
/**
*/
  frameCount: number;
/**
*/
  height: number;
/**
* @returns {Uint8Array}
*/
  readonly palette: Uint8Array;
/**
*/
  width: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_decodeframes_free: (a: number) => void;
  readonly __wbg_get_decodeframes_width: (a: number) => number;
  readonly __wbg_set_decodeframes_width: (a: number, b: number) => void;
  readonly __wbg_get_decodeframes_height: (a: number) => number;
  readonly __wbg_set_decodeframes_height: (a: number, b: number) => void;
  readonly __wbg_get_decodeframes_frameCount: (a: number) => number;
  readonly __wbg_set_decodeframes_frameCount: (a: number, b: number) => void;
  readonly decodeframes_data: (a: number, b: number) => void;
  readonly decodeframes_delays: (a: number, b: number) => void;
  readonly decodeframes_palette: (a: number, b: number) => void;
  readonly decode: (a: number, b: number) => number;
  readonly encode: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
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
