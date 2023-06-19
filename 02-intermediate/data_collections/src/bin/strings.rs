struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name = {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            // NOTE .to_owned() or String::from() both created owned string data
            name: "apple".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("banana"),
            count: 3,
        },
        LineItem {
            name: "orange".to_owned(),
            count: 5,
        },
    ];

    for item in &receipt {
        // NOTE Passing a borrowed item since print_name expects a borrowed string
        print_name(&item.name);
        println!("{} x {}", item.count, &item.name);
    }
}
