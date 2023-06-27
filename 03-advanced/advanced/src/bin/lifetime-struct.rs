#[derive(Debug)]
struct Cards {
    inner: Vec<IdCard>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum City {
    FooTown,
    BarLand,
    BazVillage,
}

#[derive(Debug)]
struct IdCard {
    name: String,
    age: u8,
    city: City,
}

impl IdCard {
    pub fn new(name: &str, age: u8, city: City) -> Self {
        Self {
            name: name.to_owned(),
            age,
            city,
        }
    }
}

fn new_ids() -> Cards {
    Cards {
        inner: vec![
            IdCard::new("Bob", 20, City::FooTown),
            IdCard::new("Frank", 30, City::BarLand),
            IdCard::new("Sally", 40, City::BazVillage),
            IdCard::new("Alice", 50, City::FooTown),
        ],
    }
}

#[derive(Debug)]
struct YoungPeople<'a> {
    // NOTE A vector of borrowed IdCard structs
    inner: Vec<&'a IdCard>,
}

// NOTE Implementation for a struct with a lifetime parameter
impl<'a> YoungPeople<'a> {
    pub fn new(cards: &'a Cards) -> Self {
        Self {
            inner: cards.inner.iter().filter(|id| id.age <= 30).collect(),
        }
    }

    pub fn in_foo_town(&self) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .filter(|id| id.city == City::FooTown)
                .map(|id| *id)
                .collect(),
        }
    }
}

fn main() {
    let people = new_ids();
    // NOTE not necessary to specify the lifetime annotations when using the struct
    let young = YoungPeople::new(&people);

    println!("ids:");
    for id in people.inner.iter() {
        println!("{:?}", id);
    }

    println!("\nyoung:");
    for id in young.inner.iter() {
        println!("{:?}", id);
    }

    println!("\nyoung in foo town:");
    for id in young.in_foo_town().inner.iter() {
        println!("{:?}", id);
    }
}
