enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let current_direction = Direction::North;

    match current_direction {
        Direction::North => println!("Facing North"),
        Direction::South => println!("Facing South"),
        Direction::East => println!("Facing East"),
        Direction::West => println!("Facing West"),
    }
}