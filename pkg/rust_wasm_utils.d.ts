/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
* @param {string} key
* @param {string} plaintext
* @returns {Uint8Array}
*/
export function aesEncrypt(key: string, plaintext: string): Uint8Array;
/**
* @param {string} key
* @param {Uint8Array} ciphertext
* @returns {string}
*/
export function aesDecrypt(key: string, ciphertext: Uint8Array): string;
/**
* @param {Uint8Array} data
* @param {number} dst_width
* @param {number} dst_height
* @returns {Uint8Array}
*/
export function imageResize(data: Uint8Array, dst_width: number, dst_height: number): Uint8Array;
