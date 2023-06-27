fn main() {
    enum Status {
        Error(i32),
        Info,
        Warn,
    }

    let status = Status::Error(19);

    let res = match status {
        // NOTE Matching on a specific value by binding some value
        Status::Error(s @ 3) => println!("error three"),
        // * Checks if s is 5 or 6
        Status::Error(s @ 5..=6) => println!("error five or six: {}", s),
        // * Checks if s is between 4 and 10
        Status::Error(s @ 4..=10) => println!("error 4 through 10 {}", s),
        Status::Error(s @ 18 | s @ 19) => println!("error 18 or 19 {}", s),
        Status::Error(s) => println!("error {}", s),
        Status::Info => println!("info"),
        Status::Warn => println!("warn"),
    };

    dbg!(res);

    enum Species {
        Finch,
        Hawk,
        Parrot,
    }
    struct Bird {
        age: usize,
        species: Species,
    }

    let bird = Bird {
        age: 17,
        species: Species::Finch,
    };

    let res = match bird {
        // NOTE Matching only on specific props of a struct, the `..` syntax ignores the rest of the props
        Bird { age: 4, .. } => println!("4 yr old bird"),
        Bird {
            age: 4..=10 | 15..=20,
            ..
        } => println!("4-10 or 15-20 year old bird"),
        Bird {
            species: Species::Finch,
            ..
        } => println!("finch!"),
        Bird { .. } => println!("other bird"),
    };

    dbg!(res);

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum Difficulty {
        Easy,
        Normal,
        Hard,
    }

    let stage = 5;
    let diff = Difficulty::Normal;
    let res = match stage {
        s if (s == 5 && diff == Difficulty::Easy) => println!("easy mode stage 5"),
        s if diff == Difficulty::Normal => println!("normal mode, stage: {}", s),
        s @ 10 | s @ 15 => println!("stage 10 or 15"),
        s => println!("other stage {}", s),
    };

    dbg!(res);

    struct Vehicle {
        km: usize,
        year: usize,
    }

    let car = Vehicle {
        km: 80_000,
        year: 2023,
    };

    let res = match car {
        // NOTE Matching on structs with guards
        Vehicle { km, year } if km == 0 && year == 2023 => println!("new & made in 2023"),
        Vehicle { km, .. } if km <= 50_000 => println!("under 50k km"),
        Vehicle { km, .. } if km >= 80_000 => println!("at least 80k km"),
        Vehicle { year, .. } if year == 2023 => println!("made in 2023"),
        Vehicle { .. } => println!("other vehicle"),
    };

    dbg!(res)
}
