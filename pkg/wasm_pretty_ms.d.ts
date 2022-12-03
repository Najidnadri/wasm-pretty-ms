/* tslint:disable */
/* eslint-disable */
/**
* @param {any} milliseconds
* @param {any} option
* @returns {any}
*/
export function prettyMilliseconds(milliseconds: any, option: any): any;
/**
*/
export class ParsedMs {
  free(): void;
/**
* @param {number} milliseconds
*/
  constructor(milliseconds: number);
/**
*/
  days: number;
/**
*/
  hours: number;
/**
*/
  microseconds: number;
/**
*/
  milliseconds: number;
/**
*/
  minutes: number;
/**
*/
  nanoseconds: number;
/**
*/
  seconds: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly prettyMilliseconds: (a: number, b: number) => number;
  readonly __wbg_parsedms_free: (a: number) => void;
  readonly __wbg_get_parsedms_days: (a: number) => number;
  readonly __wbg_set_parsedms_days: (a: number, b: number) => void;
  readonly __wbg_get_parsedms_hours: (a: number) => number;
  readonly __wbg_set_parsedms_hours: (a: number, b: number) => void;
  readonly __wbg_get_parsedms_minutes: (a: number) => number;
  readonly __wbg_set_parsedms_minutes: (a: number, b: number) => void;
  readonly __wbg_get_parsedms_seconds: (a: number) => number;
  readonly __wbg_set_parsedms_seconds: (a: number, b: number) => void;
  readonly __wbg_get_parsedms_milliseconds: (a: number) => number;
  readonly __wbg_set_parsedms_milliseconds: (a: number, b: number) => void;
  readonly __wbg_get_parsedms_microseconds: (a: number) => number;
  readonly __wbg_set_parsedms_microseconds: (a: number, b: number) => void;
  readonly __wbg_get_parsedms_nanoseconds: (a: number) => number;
  readonly __wbg_set_parsedms_nanoseconds: (a: number, b: number) => void;
  readonly parsedms_new: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
