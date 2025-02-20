use std::io::Cursor;
use image::{DynamicImage, ImageReader, Pixel};
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::imageops::{grayscale, resize, FilterType};
use tracing::log::debug;

pub type ImageResult = Result<(Vec<u8>, String), Box<dyn std::error::Error + Send + Sync>>;

pub fn crush(bytes: Vec<u8>, depth: u8) -> ImageResult {
    debug!("bits: {depth}");
    let data = Cursor::new(bytes);
    let img = ImageReader::new(data).with_guessed_format()?.decode()?;
    let mut rgba = img.to_rgba8();

    rgba.pixels_mut().for_each(|p| {
        p.apply(|ch| {
            (ch >> (8 - depth)) * (255 / (2u16.pow(depth as u32) - 1) as u8)
        });
    });

    let img = DynamicImage::from(rgba);
    let mut buf: Vec<u8> = vec![];
    let encoder = PngEncoder::new(&mut buf);
    img.write_with_encoder(encoder)?;
    Ok((buf, format!("{depth} bits")))
}

pub fn compress(bytes: Vec<u8>, quality: u8) -> ImageResult {
    let qual = f32::floor(((quality as f32 / 100.0) * 50.0)) as u8;
    let mut buf: Vec<u8> = vec![];
    let data = Cursor::new(bytes);
    let img = ImageReader::new(data).with_guessed_format()?.decode()?.to_rgb8();

    let encoder = JpegEncoder::new_with_quality(&mut buf, qual);
    img.write_with_encoder(encoder)?;
    Ok((buf, format!("quality {qual}")))
}

pub fn mask(bytes: Vec<u8>, mask: Vec<u8>) -> ImageResult {
    let data = Cursor::new(bytes);
    let mut img = ImageReader::new(data).with_guessed_format()?.decode()?.to_rgba8();

    let mask_data = Cursor::new(mask);
    let mask = ImageReader::new(mask_data).with_guessed_format()?.decode()?;
    let mask = grayscale(&mask);
    
    let scaled_mask = resize(&mask, img.dimensions().0, img.dimensions().1, FilterType::Lanczos3);
    img.enumerate_pixels_mut().for_each(|(x, y, p)| {
        let alpha = &mut p.channels_mut()[3];
        let mask = scaled_mask.get_pixel(x, y);
        *alpha = mask[0];
    });
    
    let mut buf: Vec<u8> = vec![];
    let encoder = PngEncoder::new(&mut buf);
    img.write_with_encoder(encoder)?;
    Ok((buf, "hash unknown".to_string()))
}