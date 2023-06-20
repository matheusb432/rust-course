fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    println!("Greetings");
    let sum = 2 + 2;
    let value = 10 - 5;
    let division = 10 / 2;
    let mult = 5 * 5;

    let five = sub(5, 3);
    let rem = 7 % 3;
    let rem2 = 6 % 4;

    println!("sum: {sum:?}");
    println!("value: {value:?}");
    println!("division: {division:?}");
    println!("mult: {mult:?}");
    println!("five: {five:?}");
    println!("rem: {rem:?}");
    println!("rem2: {rem2:?}");
}
