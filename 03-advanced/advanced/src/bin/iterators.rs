fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    // NOTE .collect() transforms the iterator into a collection
    // NOTE Rust can infer the type of the data structure, but not the data structure itself, so `: Vec<_>` is necessary
    let plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();
    dbg!(plus_one);

    let new_numbers: Vec<_> = numbers.iter().filter(|num| num <= &&3).collect();
    dbg!(new_numbers);

    let maybe_number = numbers.iter().find(|num| num == &&3);
    dbg!(maybe_number);

    let count = numbers.iter().count();
    dbg!(count);

    let last = numbers.iter().last();
    dbg!(last);

    let min = numbers.iter().min();
    dbg!(min);
    let max = numbers.iter().max();
    dbg!(max);
    let take: Vec<&i32> = numbers.iter().take(3).collect();
    dbg!(take);
}
