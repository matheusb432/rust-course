enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    // ? if let
    let maybe_user = Some("Jerry");

    match maybe_user {
        Some(user) => println!("user={}!", user),
        None => println!("no user"),
    }

    // NOTE equivalent to the above match statement, useful for matching on two cases
    if let Some(user) = maybe_user {
        println!("user={}!", user);
    } else {
        println!("no user");
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("color is red");
    } else {
        println!("color is not red");
    }
    // ?

    // ? while let
    let mut data = Some(3);

    while let Some(i) = data {
        println!("loop");
        data = None;
    }
    let numbers = vec![1, 2, 3];
    let mut number_iter = numbers.iter();
    // NOTE will iterate as long as there is Some<T> value
    while let Some(num) = number_iter.next() {
        println!("num={:?}", num);
    }
}
