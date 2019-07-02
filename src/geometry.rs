use std::fmt;
use line_drawing::Bresenham;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    pub fn is_inside(&self, min: i32, max: i32) -> bool {
        self.x >= min && self.x < max && self.y >= min && self.y < max
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Line {
    pub p: Point,
    pub q: Point,
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-{:?}", self.p, self.q)
    }
}

impl Line {
    pub fn new_with_center(p: Point, m: f64, len: f64) -> Line {

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

    pub fn gradient(self) -> f64 {
        let rise = (self.q.y - self.p.y) as f64;
        let run = (self.q.x - self.p.x) as f64;
        rise/run
    }

    pub fn len(self) -> i32 {
        let x = (self.q.x - self.p.x).abs();
        let y = (self.q.y - self.p.y).abs();
        ((x.pow(2) + y.pow(2)) as f64).sqrt() as i32

    }

    pub fn points_along(&self) -> impl Iterator<Item = Point> {
        let pairs = Bresenham::new((self.p.x, self.p.y), (self.q.x, self.q.y));
        pairs.map(|(x, y)| Point { x: x, y: y })
    }
}
