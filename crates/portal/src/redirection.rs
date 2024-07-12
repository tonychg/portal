use std::cmp::Ordering;

use crate::Portal;
use crate::Position;

const DIRECTIONS: [(i16, i16); 4] = [(-1, 1), (-1, -1), (1, -1), (1, 1)];

#[derive(thiserror::Error, Debug)]
pub enum RedirectionError {
    #[error("Fail to determine redirection")]
    Failed,
}

#[derive(Debug, Clone)]
pub struct Redirection {
    pub sorted_portals: Vec<Portal>,
    size: usize,
}

impl Redirection {
    pub fn new(sorted_portals: Vec<Portal>, size: usize) -> Redirection {
        Redirection {
            sorted_portals,
            size,
        }
    }

    pub fn get_all_equidistant(distance: i16, position: &Position) -> Vec<Position> {
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

    // Find the closest portal when there are two equidistant portals
    fn find_closest_when_two_equidistant(
        entry_portal: usize,
        p1: usize,
        p2: usize,
        portals: &[Portal],
    ) -> usize {
        p1
    }

    // Find the closest portal when there are three equidistant portals
    fn find_closest_when_three_equidistant(
        entry_portal: usize,
        p1: usize,
        p2: usize,
        p3: usize,
        portals: &[Portal],
    ) -> usize {
        p1
    }

    // Find the closest portal from the entry portal
    pub fn find_closest(entry_portal: usize, portals: &[Portal], state: &[usize]) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let entry_portal_position = &portals[entry_portal].position;
        let mut minimum_distance = i16::MAX;

        for (i, portal) in portals.iter().enumerate() {
            // Ignores entry portal and portals that have already been processed
            if i == entry_portal || state.contains(&i) {
                continue;
            }
            let distance = entry_portal_position.distance(&portal.position);
            match distance.cmp(&minimum_distance) {
                Ordering::Less => {
                    minimum_distance = distance;
                    result.clear();
                    result.push(i);
                }
                Ordering::Equal => result.push(i),
                Ordering::Greater => (),
            }
        }
        result
    }

    // Determine the redirection from an entry portal to other portals (MAX: 3)
    // In case of equidisant portals the closest portal is determine by the number
    // of equidistant portals.
    pub fn determine(
        selected_portal: usize,
        portals: &[Portal],
    ) -> Result<Redirection, RedirectionError> {
        let mut state: Vec<usize> = vec![selected_portal];
        let mut entry_portal = selected_portal;

        for _ in 0..portals.len() - 1 {
            let closest_portals: Vec<usize> = Self::find_closest(entry_portal, portals, &state);
            println!(
                "DEBUG: Closest portal of {:?} : {:?}",
                entry_portal, closest_portals
            );
            let closest_portal = match closest_portals.len() {
                1 => closest_portals[0],
                2 => Self::find_closest_when_two_equidistant(
                    entry_portal,
                    closest_portals[0],
                    closest_portals[1],
                    portals,
                ),
                3 => Self::find_closest_when_three_equidistant(
                    entry_portal,
                    closest_portals[0],
                    closest_portals[1],
                    closest_portals[2],
                    portals,
                ),
                _ => return Err(RedirectionError::Failed),
            };
            state.push(closest_portal);
            entry_portal = closest_portal;
        }
        Ok(Redirection::new(
            state.iter().map(|i| portals[*i].clone()).collect(),
            0,
        ))
    }
}
