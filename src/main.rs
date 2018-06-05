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

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Dir {
    H, // Horizontal
    V, // Vertical
}

impl Dir {
    fn other(&self) -> Dir {
        return match self {
            Dir::H => Dir::V,
            Dir::V => Dir::H,
        };
    }
}

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
    fn dir(self) -> Dir {
        if self.p.x == self.q.x {
            return Dir::V
        }
        Dir::H
    }

    fn len(self) -> i32 {
        match self.dir() {
            Dir::H => (self.q.x - self.p.x).abs(),
            Dir::V => (self.q.y - self.p.y).abs(),
        }
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
            fn new_with_center(p: Point, dir: Dir, len: i32) -> Line {
                match dir {
                    Dir::H => Line {
                        p: Point { x: p.x - len/2, y: p.y },
                        q: Point { x: p.x + len/2, y: p.y },
                    },
                    Dir::V => Line {
                        p: Point { x: p.x, y: p.y - len/2 },
                        q: Point { x: p.x, y: p.y + len/2 },
                    },
                }
            }
        }

        let sqrt2 = 2_f64.sqrt();
        let new_len = ((line.len() as f64) / sqrt2) as i32;

        vec![Line::new_with_center(line.p, line.dir().other(), new_len),
             Line::new_with_center(line.q, line.dir().other(), new_len)]
    }

    fn render(&self) -> [u8; IMGPX] {
        let points = self
            .older.union(&self.newer)
            .flat_map(|l| l.points_along());

        let mut canvas: [u8; IMGPX] = [0; IMGPX];
        for p in points {
            let pixel = ((p.y * IMGWID as i32) + p.x) as usize;
            canvas[pixel] = 1;
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
