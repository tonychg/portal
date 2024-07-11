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
}

/* #[test]
fn test_find_closest() {
    let positions = vec![
        Position::new(5, 5), // 10
        Position::new(9, 5), // 14
        Position::new(2, 4), // 6
    ];
    let origin = Position::new(0, 0);
    assert_eq!(origin.find_closest(&positions), &[2]);
}

#[test]
fn test_find_closest_with_multiple() {
    let positions = vec![
        Position::new(5, 5), // 10
        Position::new(9, 5), // 14
        Position::new(2, 4), // 6
        Position::new(4, 2), // 6
    ];
    let origin = Position::new(0, 0);
    assert_eq!(origin.find_closest(&positions), &[2, 3]);
} */
