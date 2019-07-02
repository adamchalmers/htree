use super::geometry::{Line, Point};
use std::collections::HashSet;
use std::iter;

#[derive(Debug)]
pub struct HTree {
    older: HashSet<Line>,
    newer: HashSet<Line>,
    gradient_change: f64,
}

impl HTree {
    pub fn new(p: Point, length: i32, gradient_change: f64) -> HTree {
        let mut start: HashSet<Line> = HashSet::new();
        start.insert(Line::new_with_center(p, gradient_change, f64::from(length)));

        HTree {
            older: HashSet::new(),
            newer: start,
            gradient_change,
        }
    }

    // Add one more level to the fractal.
    pub fn level_added(&self, n: i32) -> HTree {
        let mut older: HashSet<Line> = self.older.clone();
        let mut newer: HashSet<Line> = self.newer.clone();

        for _ in 0..n {
            // Lines generated in the previous level_added (`newer` lines) are now old.
            older = older.union(&newer).map(|x| x.to_owned()).collect();
            // Make two new lines from each previous level_added's lines.
            newer = newer
                .iter()
                .flat_map(|l| HTree::two_new(*l, self.gradient_change))
                .collect()
        }

        HTree {
            older,
            newer,
            gradient_change: self.gradient_change,
        }
    }

    // Use the H-Tree rules to generate two new lines from this one.
    fn two_new(line: Line, gradient_change: f64) -> Vec<Line> {
        let new_len = f64::from(line.len()) / 2_f64.sqrt();
        vec![
            Line::new_with_center(line.p, gradient_change - 1.0 / line.gradient(), new_len),
            Line::new_with_center(line.q, gradient_change - 1.0 / line.gradient(), new_len),
        ]
    }

    pub fn render(&self, img_width: usize) -> Vec<u8> {
        let pixels = self
            .older
            .union(&self.newer)
            .flat_map(|l| l.points_along())
            .filter(|p| p.is_inside(0, img_width as i32))
            .map(|p| ((p.y * img_width as i32) + p.x) as usize);

        let num_pixels = img_width * img_width;
        let mut canvas: Vec<u8> = iter::repeat(0).take(num_pixels).collect();
        for p in pixels {
            canvas[p] = 1;
        }
        canvas
    }
}
