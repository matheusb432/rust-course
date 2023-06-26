trait Fall {
    fn hit_ground(&self);
}

struct Vase;
impl Fall for Vase {
    fn hit_ground(&self) {
        println!("vase broke");
    }
}

struct Cat;
impl Fall for Cat {
    fn hit_ground(&self) {
        println!("cat landed on its feet");
    }
}

#[derive(Debug)]
struct Package {
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Self {
        Self { weight: weight }
    }
}

// NOTE The Default built-in trait adds a default constructor method for a struct
impl Default for Package {
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}

// NOTE receiving a parameter that implements the Fall trait
fn fall(thing: impl Fall) {
    thing.hit_ground();
}

fn main() {
    fall(Vase {});
    fall(Cat {});
    dbg!(Package::default());
}
