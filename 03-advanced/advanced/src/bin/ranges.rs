fn main() {
    // NOTE 1..=3 gets 1, 2, 3, includes the last value
    let range = 1..=3;
    // NOTE 1..4 gets 1, 2, 3, does not include the last value
    let range_2 = 1..4;

    for num in 1..4 {
        println!("{:?}", num);
    }

    for ch in 'a'..='f' {
        println!("{:?}", ch);
    }
}
