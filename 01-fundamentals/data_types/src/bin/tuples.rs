fn main() {
    let coord = (2, 3);

    // NOTE tuples are accessed by index
    println!("x -> {:?} & y -> {:?}", coord.0, coord.1);

    let (x, y) = (5, 6);

    println!("x -> {:?} & y -> {:?}", x, y);

    let (name, age) = ("John", 30);

    println!("name -> {:?} & age -> {:?}", name, age);

    // NOTE bad practice, tuples should be concise (3 values max ideally), structs should be used instead
    let favorites = ("red", 14, "TX", "pizza");

    let state = favorites.2;
    let food = favorites.3;
}
