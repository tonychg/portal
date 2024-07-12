use portal::Position;

#[test]
fn test_find_closest() {
    let positions = vec![
        Position::new(5, 5), // 10
        Position::new(9, 5), // 14
        Position::new(2, 4), // 6
    ];
    let origin = Position::new(0, 0);
    assert_eq!(origin.find_closest(&positions), (6, vec![2]));
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
    assert_eq!(origin.find_closest(&positions), (6, vec![2, 3]));
}
