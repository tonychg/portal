use std::cmp;
use std::cmp::Ordering;

use crate::Portal;
use crate::Position;

const DIRECTIONS: [(i16, i16); 4] = [(-1, 1), (-1, -1), (1, -1), (1, 1)];

#[derive(thiserror::Error, Debug)]
pub enum RedirectionError {
    #[error("Fail to determine redirection")]
    Failed,
    #[error("Unexpected triple equidistant result")]
    UnexpectedTripleEquidistant,
}

#[derive(Debug, Clone)]
pub struct Redirection {
    pub sorted_portals: Vec<Portal>,
}

impl Redirection {
    pub fn new(sorted_portals: Vec<Portal>) -> Redirection {
        Redirection { sorted_portals }
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
        let entry_portal_position = &portals[entry_portal].position;
        let p1_position = &portals[p1].position;
        let p2_position = &portals[p2].position;
        let distance = entry_portal_position.distance(&portals[p1].position);
        let equidistant_positions = Self::get_all_equidistant(distance, entry_portal_position);
        let p1_index = equidistant_positions
            .iter()
            .position(|p| p == p1_position)
            .unwrap();
        let p2_index = equidistant_positions
            .iter()
            .position(|p| p == p2_position)
            .unwrap();
        let circle_half_length = 2 * distance;
        if p1_index.abs_diff(p2) as i16 == circle_half_length {
            if (4 * circle_half_length - p1_index as i16) < 8 {
                p1
            } else {
                p2
            }
        } else {
            let circle_half_length = 4 * distance;
            let dist_p1_p2 = if p1_index > p2_index {
                p1_index - p2_index
            } else {
                p2_index - p1_index
            };
            if dist_p1_p2 as i16 > circle_half_length {
                p2
            } else {
                p1
            }
        }
    }

    // Find the closest portal when there are three equidistant portals
    fn find_closest_when_three_equidistant(
        entry_portal: usize,
        p1: usize,
        p2: usize,
        p3: usize,
        portals: &[Portal],
    ) -> Result<usize, RedirectionError> {
        let entry_portal_position = &portals[entry_portal].position;
        let distance = entry_portal_position.distance(&portals[p1].position);
        let equidistant_positions = Self::get_all_equidistant(distance, entry_portal_position);
        let mut indexes: Vec<(usize, i16)> = [p1, p2, p3]
            .iter()
            .map(|p| {
                let position = &portals[*p].position;
                (
                    *p,
                    equidistant_positions
                        .iter()
                        .position(|pos| pos == position)
                        .unwrap() as i16,
                )
            })
            .collect();
        indexes.sort_by_key(|p| p.1);
        let circle_length = distance * 4;
        let circle_half_length = distance * 2;
        let distance_p1_p2 = cmp::min(
            indexes[1].1.abs_diff(indexes[0].1) as i16,
            circle_length - indexes[1].1.abs_diff(indexes[0].1) as i16,
        );
        let distance_p2_p3 = cmp::min(
            indexes[2].1.abs_diff(indexes[1].1) as i16,
            circle_length - indexes[2].1.abs_diff(indexes[1].1) as i16,
        );
        let distance_p1_p3 = cmp::min(
            indexes[2].1.abs_diff(indexes[0].1) as i16,
            circle_length - indexes[2].1.abs_diff(indexes[0].1) as i16,
        );
        let maximum_distance = *[distance_p1_p2, distance_p2_p3, distance_p1_p3]
            .iter()
            .max()
            .unwrap();
        let mut contained = false;
        let mut contained_outer1 = None;
        let mut contained_outer2 = None;
        if maximum_distance == distance_p1_p2 && distance_p1_p3 + distance_p2_p3 == distance_p1_p2 {
            contained = true;
            contained_outer1 = Some(p1);
            contained_outer2 = Some(p2);
        }
        if maximum_distance == distance_p2_p3 && distance_p1_p3 + distance_p1_p3 == distance_p2_p3 {
            contained = true;
            contained_outer1 = Some(p2);
            contained_outer2 = Some(p3);
        }
        if maximum_distance == distance_p1_p3 && distance_p1_p2 + distance_p2_p3 == distance_p1_p3 {
            contained = true;
            contained_outer1 = Some(p1);
            contained_outer2 = Some(p3);
        }
        if !contained {
            for index in indexes.iter() {
                if index.1 == 0 {
                    return Ok(index.0);
                }
            }
            Ok(indexes[2].0)
        } else {
            match (contained_outer1, contained_outer2) {
                (Some(contained_outer1), Some(contained_outer2)) => {
                    let dist_outer = contained_outer2.abs_diff(contained_outer1);
                    if dist_outer as i16 > circle_half_length {
                        Ok(contained_outer2)
                    } else {
                        Ok(contained_outer1)
                    }
                }
                _ => Err(RedirectionError::UnexpectedTripleEquidistant),
            }
        }
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
                )?,
                _ => return Err(RedirectionError::Failed),
            };
            state.push(closest_portal);
            entry_portal = closest_portal;
        }
        Ok(Redirection::new(
            state.iter().map(|i| portals[*i].clone()).collect(),
        ))
    }
}
