use image::{DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;
use webp::{Encoder, WebPMemory};
use crate::format::{identify_format, ImageType};
use std::error::Error;

pub fn png_to_webp(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::load_from_memory_with_format(&data, image::ImageFormat::Png)?;
    encode_to_webp(img)
}

pub fn jpg_to_webp(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::load_from_memory_with_format(&data, image::ImageFormat::Jpeg)?;
    encode_to_webp(img)
}

pub fn jpeg_to_webp(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    jpg_to_webp(data)
}

pub fn bmp_to_webp(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::load_from_memory_with_format(&data, image::ImageFormat::Bmp)?;
    encode_to_webp(img)
}

pub fn process_to_webp(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let format = identify_format(data.clone());
    match format {
        ImageType::Webp => Ok(data),
        ImageType::Png => png_to_webp(data),
        ImageType::Jpg => jpg_to_webp(data),
        ImageType::Jpeg => jpeg_to_webp(data),
        ImageType::Bmp => bmp_to_webp(data),
        ImageType::Unsupported => Err("Unsupported image format".into()),
    }
}

fn encode_to_webp(img: DynamicImage) -> Result<Vec<u8>, Box<dyn Error>> {
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let chunks: Vec<_> = rgba.chunks(4 * width as usize).collect();
    
    let encoded_chunks: Vec<_> = chunks.par_iter().map(|chunk| {
        let encoder = Encoder::from_rgba(chunk, width, 1);
        encoder.encode(75.0).to_vec()
    }).collect();
    
    let mut result = Vec::new();
    for chunk in encoded_chunks {
        result.extend(chunk);
    }
    Ok(result)
}