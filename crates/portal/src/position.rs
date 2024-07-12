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
}
