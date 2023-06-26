trait CheckIn {
    fn check_in(&self);
    fn process(&self) {
        // NOTE Default process() implementation
        println!("Default check-in");
    }
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("Pilot checking in");
    }
    fn process(&self) {
        println!("Processing pilot check-in");
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("Passenger checking in");
    }
    fn process(&self) {
        println!("Processing passenger check-in");
    }
}

struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("Cargo checking in");
    }
}

// NOTE The generic type parameter T is constrained to types that implement the CheckIn trait
fn print_check_in<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
    println!();
}

fn main() {
    print_check_in(Pilot {});
    print_check_in(Passenger {});
    print_check_in(Cargo {});
}
