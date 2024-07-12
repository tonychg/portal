use portal::{Portal, Position, Redirection};

fn determine_redirection(portals: &[Portal]) {
    Portal::debug(10, 10, &portals);
    let redirection = Redirection::determine(0, &portals).expect("Unexpected error");
    Portal::debug(10, 10, &redirection.sorted_portals);
}

fn main() {
    println!("DEBUG: Simple redirection");
    determine_redirection(&[
        Portal::new(Position::new(5, 5)),
        Portal::new(Position::new(7, 6)),
        Portal::new(Position::new(6, 6)),
        Portal::new(Position::new(8, 5)),
    ]);
    println!("DEBUG: Double equidistant redirection");
    determine_redirection(&[
        Portal::new(Position::new(5, 5)),
        Portal::new(Position::new(7, 6)),
        Portal::new(Position::new(3, 6)),
        Portal::new(Position::new(8, 8)),
    ]);
    println!("DEBUG: Triple equidistant redirection");
    determine_redirection(&[
        Portal::new(Position::new(5, 5)),
        Portal::new(Position::new(7, 6)),
        Portal::new(Position::new(3, 6)),
        Portal::new(Position::new(8, 5)),
    ]);
}
