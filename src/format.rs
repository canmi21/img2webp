use image::ImageFormat;

#[derive(Debug)]
pub enum ImageType {
    Png,
    Jpg,
    Jpeg,
    Bmp,
    Webp,
    Unsupported,
}

impl ToString for ImageType {
    fn to_string(&self) -> String {
        match self {
            ImageType::Png => ".png".to_string(),
            ImageType::Jpg => ".jpg".to_string(),
            ImageType::Jpeg => ".jpeg".to_string(),
            ImageType::Bmp => ".bmp".to_string(),
            ImageType::Webp => ".webp".to_string(),
            ImageType::Unsupported => "unsupported".to_string(),
        }
    }
}

pub fn identify_format(data: Vec<u8>) -> ImageType {
    match image::guess_format(&data) {
        Ok(ImageFormat::Png) => ImageType::Png,
        Ok(ImageFormat::Jpeg) => ImageType::Jpeg,
        Ok(ImageFormat::Bmp) => ImageType::Bmp,
        Ok(ImageFormat::WebP) => ImageType::Webp,
        _ => ImageType::Unsupported,
    }
}