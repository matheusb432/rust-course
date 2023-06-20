// Topic: Map combinator
//
// Requirements:
// * 1. Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

impl User {
    fn new(id: i32, name: &str) -> Self {
        Self {
            user_id: id,
            name: name.to_owned(),
        }
    }
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let maybe_user_by_name = |name| find_user(name).map(|id| User::new(id, name));
    let on_not_found = || "not_found".to_owned();
    let format_user = |u: User| format!("Found user: {u:?}");

    // NOTE Mapping an iterable
    let users = ["sam", "matt", "joe", "katie"].map(maybe_user_by_name);
    for maybe_user in users {
        // NOTE possible to pass the closures instead of defining them inline
        let message = maybe_user.map_or_else(on_not_found, format_user);
        // NOTE map_or_else() accepts a closure to handle the None case as the first argument
        // let message = user.map_or_else(|| "not found".to_owned(), |u| format!("Found user: {u:?}"));

        println!("{message:?}");
    }
}
