enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let direction = Direction::Down;
    match direction {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
    }
}
