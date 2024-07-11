use crate::Portal;
use crate::Position;

const DIRECTIONS: [(i16, i16); 4] = [(-1, 1), (-1, -1), (1, -1), (1, 1)];

fn get_all_equidistant(distance: i16, position: Position) -> Vec<Position> {
    let sacred_points: [Position; 4] = [
        position.add(&Position::new(distance, 0)),
        position.add(&Position::new(0, distance)),
        position.add(&Position::new(-distance, 0)),
        position.add(&Position::new(0, -distance)),
    ];
    let mut equidistant_points: Vec<Position> = Vec::new();

    for i in 0..DIRECTIONS.len() {
        for j in 0..distance {
            let point =
                sacred_points[i].add(&Position::new(DIRECTIONS[i].0 * j, DIRECTIONS[i].1 * j));
            equidistant_points.push(point);
        }
    }
    equidistant_points
}

pub struct Redirection {
    sorted_portals: Vec<Portal>,
    size: usize,
}

impl Redirection {
    pub fn determine(entry_portal: Portal, portals: Vec<Portal>) -> Redirection {
        let mut sorted_positions: Vec<Position> = Vec::new();
        let mut left_portals: Vec<Portal> = portals
            .into_iter()
            .filter(|portal| portal.position != entry_portal.position)
            .collect();
        let mut current_portal = entry_portal;
        for _ in 0..left_portals.len() {
            let left_positions: Vec<Position> =
                left_portals.iter().map(|p| p.position.clone()).collect();
            let (minimum_distance, closest_portals) =
                current_portal.position.find_closest(&left_positions);
            if closest_portals.len() == 1 {
                sorted_positions.push(left_positions[closest_portals[0]].clone());
            } else if closest_portals.len() == 2 {
                let equidistant_positions = get_all_equidistant(
                    minimum_distance,
                    left_positions[closest_portals[0]].clone(),
                );
            }
        }

        todo!()
    }
}
