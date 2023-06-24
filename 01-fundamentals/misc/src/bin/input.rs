use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();

    // NOTE passing a mutable reference to the buffer so that the input can be read
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut all_input = vec![];
    let mut times_input = 0;

    while times_input < 2 {
        match get_input() {
            Ok(input) => {
                all_input.push(input);
                times_input += 1;
            }
            Err(err) => panic!("error: {err:?}"),
        }
    }

    for input in all_input {
        println!(
            "Original: {:?}\nCapitalized: {:?}",
            input,
            input.to_uppercase()
        )
    }
}
