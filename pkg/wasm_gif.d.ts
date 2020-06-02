/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} data 
* @returns {any} 
*/
export function decode(data: Uint8Array): any;
/**
* @param {any} data 
* @param {number} mode 
* @param {number | undefined} times 
* @returns {Uint8Array} 
*/
export function encode(data: any, mode: number, times?: number): Uint8Array;
/**
*/
export enum GifPlayMode {
  REPEAT,
  SINGLE,
}
