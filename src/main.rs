extern crate gif;
extern crate line_drawing;

use gif::{Encoder, Frame, Repeat, SetParameter};
use line_drawing::Bresenham;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::Result;

const IMGWID: usize = 256;
const IMGPX: usize = IMGWID * IMGWID;
const FILENAME: &str = "htree.gif";
const NUM_FRAMES: i32 = 10;

// -------------------------------------------------------------------------
// GEOMETRY

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    fn is_inside(&self, min: i32, max: i32) -> bool {
        self.x >= min && self.x < max && self.y >= min && self.y < max
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Line {
    p: Point,
    q: Point,
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-{:?}", self.p, self.q)
    }
}

impl Line {
    fn gradient(self) -> f64 {
        let rise = (self.q.y - self.p.y) as f64;
        let run = (self.q.x - self.p.x) as f64;
        rise/run
    }

    fn len(self) -> i32 {
        let x = (self.q.x - self.p.x).abs();
        let y = (self.q.y - self.p.y).abs();
        ((x.pow(2) + y.pow(2)) as f64).sqrt() as i32

    }

    fn points_along(&self) -> impl Iterator<Item = Point> {
        let pairs = Bresenham::new((self.p.x, self.p.y), (self.q.x, self.q.y));
        pairs.map(|(x, y)| Point { x: x, y: y })
    }
}

// -------------------------------------------------------------------------
// HTREE FRACTALS

#[derive(Debug)]
struct HTree {
    older: HashSet<Line>,
    newer: HashSet<Line>,
}

impl HTree {
    fn new(l: Line) -> HTree {
        let mut start: HashSet<Line> = HashSet::new();
        start.insert(l);

        HTree {
            older: HashSet::new(),
            newer: start,
        }
    }

    // Add one more level to the fractal.
    fn level_added(&self) -> HTree {
        HTree {
            // Lines generated in the previous level_added (`newer` lines) are now old.
            older: 
                self.older.union(&self.newer)
                .map(|x| x.to_owned())
                .collect(),
            // Make two new lines from each previous level_added's lines.
            newer: self.newer.iter().flat_map(|l| HTree::two_new(*l)).collect(),
        }
    }

    // Use the H-Tree rules to generate two new lines from this one.
    fn two_new(line: Line) -> Vec<Line> {

        impl Line {
            fn new_with_center(p: Point, m: f64, len: f64) -> Line {

                // Special case for vertical lines to avoid dividing by zero
                if m.is_infinite() {
                    let l = len.round() as i32;
                    return Line {
                        p: Point { x: p.x, y: p.y - l/2 },
                        q: Point { x: p.x, y: p.y + l/2 },
                    }
                }


                let x = p.x as f64;
                let y = p.y as f64;
                let l = len/2.0;
                let l2_m2p1_ = (l.powi(2) * (m.powi(2) + 1.0)).sqrt();
                let m2xpx = (m.powi(2) * x) + x;
                let m2ypy = (m.powi(2) * y) + y;
                let m2p1 = m.powi(2) + 1.0;

                let a = l2_m2p1_ + m2xpx;
                let b = (m*l2_m2p1_) + m2ypy;
                let c = (-1.0*l2_m2p1_) + m2xpx;
                let d = (-1.0*m*l2_m2p1_) + m2ypy;

                fn divide(p: f64, q: f64) -> i32 {
                    (p/q).round() as i32
                }

                Line { 
                    p: Point { 
                        x: divide(a, m2p1), 
                        y: divide(b, m2p1)
                    }, 
                    q: Point { 
                        x: divide(c, m2p1),
                        y: divide(d, m2p1)
                    } 
                }
            }
        }

        let new_len = (line.len() as f64) / 2_f64.sqrt();

        vec![Line::new_with_center(line.p, -1.0/line.gradient(), new_len),
             Line::new_with_center(line.q, -1.0/line.gradient(), new_len)]
    }

    fn render(&self) -> [u8; IMGPX] {
        let pixels = self
            .older.union(&self.newer)
            .flat_map(|l| l.points_along())
            .filter(|p| p.is_inside(0, IMGWID as i32))
            .map(|p| ((p.y * IMGWID as i32) + p.x) as usize);

        let mut canvas: [u8; IMGPX] = [0; IMGPX];
        for p in pixels {
            canvas[p] = 1;
        }
        canvas
    }
}

// -------------------------------------------------------------------------
// GIFS

struct GifEncoder {
    encoder: Encoder<File>,
    img_size: usize,
}

impl GifEncoder {

    fn new(img_size: usize) -> Result<GifEncoder> {
        let color_map =
        &[ 0xFF, 0xFF, 0xFF, // Background RGB
           0xFF, 0xAA, 0 ];  // Foreground RGB

        let f = File::create(FILENAME)?;
        let mut encoder = Encoder::new(f, img_size as u16, img_size as u16, color_map)?;
        encoder.set(Repeat::Infinite)?;
        Ok(GifEncoder {
            encoder: encoder,
            img_size: img_size,
        })
    }

    fn add_frame(&mut self, bitmap: [u8; IMGPX]) -> Result<()> {
        let mut frame = Frame::default();
        frame.width = self.img_size as u16;
        frame.height = self.img_size as u16;
        frame.buffer = Cow::Borrowed(&bitmap);

        self.encoder.write_frame(&frame)?;
        Ok(())
    }
}


// -------------------------------------------------------------------------
// RUNNING

fn main() -> Result<()> {
    let width = IMGWID as f64;
    let mut h = HTree::new(Line { 
        p: Point { x: (width/2.0) as i32, y: (width*0.25) as i32 }, 
        q: Point { x: (width/2.0) as i32, y: (width*0.75) as i32 } 
    });

    let mut encoder = GifEncoder::new(IMGWID)?;

    for _ in 0..NUM_FRAMES {
        encoder.add_frame(h.render())?;
        h = h.level_added();
    }

    Ok(())
}
