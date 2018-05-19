extern crate image;

use image::gif::Encoder;
use image::gif::Frame;
use std::fs::File;

fn main() -> Result<(), HTreeError> {
    println!("Hello, world!");
    let size = 100;
    let bounds: (usize, usize) = (size, size);
    let mut pixels = vec![255; bounds.0 * bounds.1 * 4];
    let filename = "htree.gif";

    let _ = write_image(filename, &mut pixels, bounds)?;
    Ok(())
}

fn write_image(
    filename: &str, 
    pixels: &mut[u8], 
    bounds: (usize, usize)
) -> Result<(), HTreeError> {
    let output = File::create(filename)?;
    let frame = Frame::from_rgba(bounds.0 as u16, bounds.1 as u16, pixels);
    let encoder = Encoder::new(output);
    encoder.encode(frame)?;
    Ok(())
}

// Error handling

#[derive(Debug)]
enum HTreeError {
    IOError(std::io::Error),
    ImgError(image::ImageError),
}

impl From<std::io::Error> for HTreeError {
    fn from(err: std::io::Error) -> Self {
        HTreeError::IOError(err)
    }
}

impl From<image::ImageError> for HTreeError {
    fn from(err: image::ImageError) -> Self {
        HTreeError::ImgError(err)
    }
}