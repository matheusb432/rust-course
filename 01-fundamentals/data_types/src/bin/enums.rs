// NOTE: Enums are types which have a few definite values
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    println!("Hello, world!");
    // NOTE acessing the enum values with `::`
    let go = Direction::Down;
    match go {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }
}
