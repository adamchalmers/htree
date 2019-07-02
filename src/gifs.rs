use gif::{Encoder, Frame, Repeat, SetParameter};
use std::borrow::Cow;
use std::fs::File;
use std::io::Result;

const IMGPX: usize = IMGWID * IMGWID;
const FILENAME: &str = "htree.gif";
const IMGWID: usize = 256;

pub struct GifEncoder {
    encoder: Encoder<File>,
    img_size: usize,
    delay: u16,
}

impl GifEncoder {

    pub fn new(img_size: usize, delay: u16) -> Result<GifEncoder> {
        let color_map = &[
            0xFF, 0xFF, 0xFF, // Background RGB
            0xFF, 0xAA, 0,
        ]; // Foreground RGB

        let f = File::create(FILENAME)?;
        let mut encoder = Encoder::new(f, img_size as u16, img_size as u16, color_map)?;
        encoder.set(Repeat::Infinite)?;
        Ok(GifEncoder {
            encoder: encoder,
            img_size: img_size,
            delay: delay,
        })
    }

    pub fn add_frame(&mut self, bitmap: [u8; IMGPX]) -> Result<()> {
        let mut frame = Frame::default();
        frame.width = self.img_size as u16;
        frame.height = self.img_size as u16;
        frame.buffer = Cow::Borrowed(&bitmap);
        frame.delay = self.delay;

        self.encoder.write_frame(&frame)?;
        Ok(())
    }
}