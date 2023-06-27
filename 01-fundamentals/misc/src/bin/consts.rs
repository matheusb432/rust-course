// NOTE with constants it's possible to have cleaner code at no cost of performance since they are inlined by the compiler
const MAX_SPEED: i32 = 9000;

fn clamp_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        MAX_SPEED
    } else {
        speed
    }
}

fn main() {
    println!("max speed: {:?}", clamp_speed(15000));
}
