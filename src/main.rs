extern crate gif;
extern crate line_drawing;

use gif::{Encoder, Frame, Repeat, SetParameter};
use line_drawing::Bresenham;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;

const IMGWID: usize = 256;
const IMGPX: usize = IMGWID * IMGWID;
const FILENAME: &str = "htree.gif";

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
    t: i32,
    older: HashSet<Line>,
    newer: HashSet<Line>,
}

impl HTree {
    fn new(l: Line) -> HTree {
        let mut start: HashSet<Line> = HashSet::new();
        start.insert(l);

        HTree {
            t: 0,
            older: HashSet::new(),
            newer: start,
        }
    }

    // Add one more level to the fractal.
    fn tick(&self) -> HTree {
        HTree {
            t: self.t + 1,
            // Lines generated in the previous tick (`newer` lines) are now old.
            older: 
                self.older.union(&self.newer)
                .map(|x| x.to_owned())
                .collect(),
            // Make two new lines from each previous tick's lines.
            newer: self.newer.iter().flat_map(|l| HTree::two_new(*l)).collect(),
        }
    }

    // Use the H-Tree rules to generate two new lines from this one.
    // Each new line will be perpendicular to the current line and half its height.
    // The original line will bisect each of the two new lines.
    fn two_new(line: Line) -> Vec<Line> {

        impl Line {
            fn sprout(p: Point, dir: Dir, len: i32) -> Line {
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

        vec!
            [ Line::sprout(line.p, line.dir().other(), new_len)
            , Line::sprout(line.q, line.dir().other(), new_len)
        ]
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

fn write_image(
    filename: &str,
    bitmaps: Vec<[u8; IMGPX]>,
    img_size: usize,
) -> Result<(), HTreeError> {
    let mut file = File::create(filename)?;
    let color_map = 
        &[ 0xFF, 0xFF, 0xFF // Background RGB
         , 0xFF, 0xAA, 0    // Foreground RGB
         ];
    let mut encoder = Encoder::new(&mut file, img_size as u16, img_size as u16, color_map)?;
    encoder.set(Repeat::Infinite).unwrap();

    for bitmap in bitmaps {
        let mut frame = Frame::default();
        frame.width = img_size as u16;
        frame.height = img_size as u16;
        frame.buffer = Cow::Borrowed(&bitmap);
        encoder.write_frame(&frame).unwrap();
    }

    Ok(())
}

// -------------------------------------------------------------------------
// RUNNING

fn main() -> Result<(), HTreeError> {

    let width = IMGWID as f64;
    let mut h = HTree::new(Line { 
        p: Point { x: (width/2.0) as i32, y: (width*0.25) as i32 }, 
        q: Point { x: (width/2.0) as i32, y: (width*0.75) as i32 } 
    });

    let mut bitmaps: Vec<[u8; IMGPX]> = Vec::new();
    for _ in 0..10 {
        bitmaps.push(h.render());
        h = h.tick();
    }
    let _ = write_image(FILENAME, bitmaps, IMGWID)?;

    Ok(())
}

#[derive(Debug)]
enum HTreeError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for HTreeError {
    fn from(err: std::io::Error) -> Self {
        HTreeError::IOError(err)
    }
}
