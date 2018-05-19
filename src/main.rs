extern crate gif;

use gif::{Frame, Encoder, Repeat, SetParameter};
use std::fs::File;
use std::borrow::Cow;

fn main() -> Result<(), HTreeError> {
    let size = 6;
    let bounds: (usize, usize) = (size, size);
    let filename = "htree.gif";

    let p1 = &mut[
        0, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 0,
        0, 1, 1, 0, 0, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0,
    ];
    let p2 = &mut[
        0, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 0,
        0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0,
    ];
    let bitmaps: Vec<&mut [u8]> = vec![p1, p2];

    let _ = write_image(filename, bitmaps, bounds)?;
    Ok(())
}

fn write_image(
    filename: &str, 
    bitmaps: Vec<&mut [u8]>, 
    bounds: (usize, usize)
) -> Result<(), HTreeError> {

    let mut file = File::create(filename)?;
    let color_map = &[0xFF, 0xFF, 0xFF, 0xFF, 0xAA, 0]; // Background RGB, then foreground RGB
    let mut encoder = Encoder::new(&mut file, bounds.0 as u16, bounds.1 as u16, color_map)?;
    encoder.set(Repeat::Infinite).unwrap();

    for bitmap in bitmaps {
        let mut frame = Frame::default();
        frame.width = bounds.0 as u16;
        frame.height = bounds.1 as u16;
        frame.buffer = Cow::Borrowed(&*bitmap);
        encoder.write_frame(&frame).unwrap();
    }

    Ok(())
}

// Error handling

#[derive(Debug)]
enum HTreeError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for HTreeError {
    fn from(err: std::io::Error) -> Self {
        HTreeError::IOError(err)
    }
}