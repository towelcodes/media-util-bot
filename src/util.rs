use std::io::Cursor;
use image::{DynamicImage, ImageReader};
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use tracing::log::debug;

pub fn crush(bytes: Vec<u8>, percentage: i64) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>{
    let depth = f32::floor((percentage as f32 / 100.0) * 255.0) as u8;
    debug!("percentage: {percentage}");
    debug!("bits: {depth}");
    let data = Cursor::new(bytes);
    let img = ImageReader::new(data).with_guessed_format()?.decode()?;
    let mut rgba = img.to_rgba8();

    for pixel in rgba.pixels_mut() {
        for i in 0..3 { // iterate through channels
            let chan = pixel[i];
            pixel[i] = ((chan * depth / 255) * (255 / depth)) as u8;
        }
    }

    let img = DynamicImage::from(rgba);
    let mut buf: Vec<u8> = Vec::new();
    let cursor = Cursor::new(&mut buf);
    let encoder = PngEncoder::new(cursor);
    img.write_with_encoder(encoder)?;
    Ok(buf.to_owned())
}

pub fn compress(bytes: Vec<u8>, quality: u8) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> { 
    let qual = f32::floor(((quality as f32 / 100.0) * 50.0)) as u8;
    let mut buf: Vec<u8> = Vec::new();
    let data = Cursor::new(bytes);
    let img = ImageReader::new(data).with_guessed_format()?.decode()?;
    
    let encoder = JpegEncoder::new_with_quality(&mut buf, qual); 
    img.write_with_encoder(encoder)?;
    Ok(buf)
}