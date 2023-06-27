// Topic: Arrays & Slices
//
// Requirements:
// * 1. Print pairs of numbers and their sums as they are streamed from a data source
// * 2. If only one number is received, then print "Unpaired value: V",
//      where V is the value
// * 3. If no numbers are received, print "Data stream complete"

// * As the lifetime of this slice is 'static, it will not be dropped when the function exits
fn data() -> &'static [u64] {
    &[7, 6, 5, 4, 3, 3, 1]
}

// * 1. 2. 3.
fn process_chunk(chunk: &[u64]) {
    match chunk {
        [a, b] => println!("{a} + {b} = {:?}", (a + b)),
        [a] => println!("Unpaired value: {a:?}"),
        [] => println!("Data stream complete"),
        [..] => unreachable!("chunk size should be at most 2"),
    }
}

fn main() {
    let stream = data().chunks(2);

    for slice in stream {
        process_chunk(slice);
    }
}
