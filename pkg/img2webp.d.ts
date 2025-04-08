/* tslint:disable */
/* eslint-disable */
export function convert_png_to_webp(data: Uint8Array): Uint8Array;
export function convert_jpg_to_webp(data: Uint8Array): Uint8Array;
export function convert_jpeg_to_webp(data: Uint8Array): Uint8Array;
export function convert_bmp_to_webp(data: Uint8Array): Uint8Array;
export function detect_image_format(data: Uint8Array): string;
export function process_image_to_webp(data: Uint8Array): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly convert_png_to_webp: (a: number, b: number) => [number, number, number, number];
  readonly convert_jpeg_to_webp: (a: number, b: number) => [number, number, number, number];
  readonly convert_bmp_to_webp: (a: number, b: number) => [number, number, number, number];
  readonly detect_image_format: (a: number, b: number) => [number, number];
  readonly process_image_to_webp: (a: number, b: number) => [number, number, number, number];
  readonly convert_jpg_to_webp: (a: number, b: number) => [number, number, number, number];
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
