use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    x: i16,
    y: i16,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Position {
        Position { x, y }
    }

    pub fn distance(&self, other: &Position) -> i16 {
        self.x.abs_diff(other.x) as i16 + self.y.abs_diff(other.y) as i16
    }

    pub fn add(&self, other: &Position) -> Position {
        Position::new(self.x + other.x, self.y + other.y)
    }

    pub fn find_closest(&self, positions: &[Position]) -> (i16, Vec<usize>) {
        let mut result: Vec<usize> = Vec::new();
        let mut minimum_distance = self.distance(&positions[0]);

        for (index, pos) in positions[1..].iter().enumerate() {
            let distance = self.distance(pos);
            match distance.cmp(&minimum_distance) {
                Ordering::Less => {
                    minimum_distance = distance;
                    result.clear();
                    result.push(index + 1);
                }
                Ordering::Equal => result.push(index + 1),
                Ordering::Greater => (),
            }
        }
        (minimum_distance, result)
    }

    // Debug function to print positions in a grid with their index.
    pub fn debug(width: i16, height: i16, positions: Vec<Position>) {
        for y in 0..=height {
            for x in 0..=width {
                match positions.iter().position(|r| *r == Position::new(x, y)) {
                    Some(index) => print!("{:2} ", index),
                    None => print!(" . "),
                }
            }
            println!();
        }
    }
}
