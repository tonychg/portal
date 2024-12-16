use portal::{Portal, Position, Redirection};

#[test]
fn test_get_all_equidistant() {
    // .  .  .  .  .  .  .  .  .  .  .
    // .  .  .  .  .  .  .  .  .  .  .
    // .  .  .  .  . 10  .  .  .  .  .
    // .  .  .  .  9  . 11  .  .  .  .
    // .  .  .  8  .  .  . 12  .  .  .
    // .  .  7  .  .  0  .  .  1  .  .
    // .  .  .  6  .  .  .  2  .  .  .
    // .  .  .  .  5  .  3  .  .  .  .
    // .  .  .  .  .  4  .  .  .  .  .
    // .  .  .  .  .  .  .  .  .  .  .
    // .  .  .  .  .  .  .  .  .  .  .
    let position = Position::new(5, 5);
    let distance = 3;
    assert_eq!(
        Redirection::get_all_equidistant(distance, &position),
        vec![
            Position::new(8, 5),
            Position::new(7, 6),
            Position::new(6, 7),
            Position::new(5, 8),
            Position::new(4, 7),
            Position::new(3, 6),
            Position::new(2, 5),
            Position::new(3, 4),
            Position::new(4, 3),
            Position::new(5, 2),
            Position::new(6, 3),
            Position::new(7, 4)
        ]
    );
}

#[test]
fn test_find_closest() {
    let portals = vec![
        Portal::new(Position::new(0, 0)),
        Portal::new(Position::new(5, 5)), // 10
        Portal::new(Position::new(9, 5)), // 14
        Portal::new(Position::new(2, 4)), // 6
    ];
    assert_eq!(Redirection::find_closest(0, &portals, &[]), vec![3]);
}

#[test]
fn test_find_closest_with_multiple() {
    let portals = vec![
        Portal::new(Position::new(0, 0)),
        Portal::new(Position::new(9, 5)), // 14
        Portal::new(Position::new(2, 4)), // 6
        Portal::new(Position::new(4, 2)), // 6
    ];
    assert_eq!(Redirection::find_closest(0, &portals, &[]), vec![2, 3]);
}
