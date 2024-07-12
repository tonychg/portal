use portal::{Position, Redirection};

fn main() {
    let position = Position::new(5, 5);
    let distance = 3;
    let mut equidistant = Redirection::get_all_equidistant(distance, &position);
    equidistant.insert(0, position);
    Position::debug(10, 10, equidistant);
}
