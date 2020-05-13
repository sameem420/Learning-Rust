enum Direction {
    Up,
    Down,
    Left,
    Right
}
fn main() {
    let player_direction:Direction = Direction::Left;
    match player_direction {
        Direction::Up => println!("We are heading Up"),
        Direction::Down => println!("We are heading Down"),
        Direction::Left => println!("We are heading Left"),
        Direction::Right => println!("We are heading Right"),
    }
}
