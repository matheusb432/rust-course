// Topic: Option combinators
//
// Requirements:
// * Use combinators as described in the functions:
//   part_1, part_2, and part_3
//
// Notes:
// * 1. Run `cargo test --bin a23` to check your program.
// * 2. Only edit the part_1, part_2, and part_3 functions.

fn part_1() -> bool {
    maybe_access("admin").is_some()
}

fn part_2() -> Option<Access> {
    maybe_access("root").or_else(|| root())
}

fn part_3() -> Access {
    maybe_access("Alice").unwrap_or_else(|| Access::Guest)
}

#[derive(Debug, Eq, PartialEq)]
enum Access {
    Admin,
    User,
    Guest,
}

fn maybe_access(name: &str) -> Option<Access> {
    match name {
        "admin" => Some(Access::Admin),
        "gary" => Some(Access::User),
        _ => None,
    }
}

fn root() -> Option<Access> {
    Some(Access::Admin)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_part_1() {
        assert_eq!(part_1(), true, "Admins have an access level");
    }

    #[test]
    fn check_part_2() {
        assert_eq!(
            part_2(),
            Some(Access::Admin),
            "Root users have Admin access"
        );
    }

    #[test]
    fn check_part_3() {
        assert_eq!(part_3(), Access::Guest, "Alice is a guest");
    }
}
