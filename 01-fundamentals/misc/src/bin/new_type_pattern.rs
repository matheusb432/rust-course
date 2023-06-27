#[derive(Debug, Copy, Clone)]
struct NeverZero(i32);

impl NeverZero {
    // NOTE using the new type pattern to guarantee that a struct will have valid data
    pub fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            Err("cannot be 0".to_owned())
        } else {
            Ok(NeverZero(i))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    a / b.0
}

fn main() {
    let a = 10;
    let b = NeverZero::new(0).unwrap_or_else(|err| {
        println!("Error: {:?}", err);
        NeverZero::new(1).unwrap()
    });

    println!("{} / {} = {}", a, b.0, divide(a, b));
}
