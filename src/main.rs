enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
}

fn main() {
    let direction = Direction::NorthEast;
    steer(direction);
}

fn steer(dir: Direction) {
    match dir {
        Direction::East => println!("Heading East"),
        Direction::West => println!("Heading West"),
        Direction::North => println!("Heading North"),
        Direction::South => println!("Heading South"),
        _ => println!("Heading somewhere"),
    }
}
