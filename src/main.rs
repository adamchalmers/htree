extern crate gif;
extern crate line_drawing;
extern crate rayon;

use fractals::HTree;
use geometry::Point;
use gifs::GifEncoder;
use std::io::Result;

mod fractals;
mod geometry;
mod gifs;

pub const IMGWID: usize = 1024;
pub const NUM_PIXELS: usize = IMGWID*IMGWID;
const NUM_FRAMES: usize = 300;
const TURN_SPEED: f64 = 0.05;
const MAX_LEVELS: i32 = 6;
const ANIMATION_DELAY: u16 = 4;

fn main() -> Result<()> {
    let width = IMGWID as i32;
    let mut encoder = GifEncoder::new(IMGWID, ANIMATION_DELAY)?;
    let mut d_levels = 1;
    let mut num_levels = 1;

    for i in 0..NUM_FRAMES {
        let gradient_change = TURN_SPEED * f64::from(i as i32);
        let mut h = HTree::new(
            Point {
                x: width / 2,
                y: width / 2,
            },
            width / 2,
            gradient_change.tan(),
        );

        if num_levels == MAX_LEVELS || num_levels == 0 {
            d_levels *= -1;
        }
        num_levels += d_levels;
        h = h.level_added(num_levels);
        encoder.add_frame(h.render(IMGWID))?;
    }

    Ok(())
}
