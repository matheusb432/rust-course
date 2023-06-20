use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();

    lockers.insert(
        1,
        Contents {
            content: "stuff".to_string(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "shirt".to_string(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "pencil".to_string(),
        },
    );

    // NOTE the order in which the lockers are printed is not guaranteed
    for (locker_number, contents) in lockers.iter() {
        println!("locker {} contains {}", locker_number, contents.content);
    }
}
