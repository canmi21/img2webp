use wasm_bindgen::prelude::*;

mod convert;
mod format;

use convert::*;
use format::*;

#[wasm_bindgen]
pub fn convert_png_to_webp(data: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    png_to_webp(data).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn convert_jpg_to_webp(data: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    jpg_to_webp(data).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn convert_jpeg_to_webp(data: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    jpeg_to_webp(data).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn convert_bmp_to_webp(data: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    bmp_to_webp(data).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn detect_image_format(data: Vec<u8>) -> String {
    identify_format(data).to_string()
}

#[wasm_bindgen]
pub fn process_image_to_webp(data: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    process_to_webp(data).map_err(|e| JsValue::from_str(&e.to_string()))
}