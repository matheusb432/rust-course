struct Temperature {
    degrees_f: f64,
}

// NOTE Adding an implementation of the struct, will make it possible to call defined functions via the struct
impl Temperature {
    // NOTE Can be thought of as a factory method, Self is the struct type
    fn freezing() -> Self {
        Temperature { degrees_f: 32.0 }
    }

    fn boiling() -> Self {
        Temperature { degrees_f: 212.0 }
    }

    // NOTE Accepting a reference to self to access the struct
    fn show_temp(&self) {
        println!("The temperature is {}°F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.9 };
    // NOTE calling the implementation function via the struct
    Temperature::show_temp(&hot);
    // NOTE Also possible to call it via the variable when it accepts a reference to self, much like extension methods
    hot.show_temp();

    // NOTE instantiating a struct via the implementation function
    let cold = Temperature::freezing();
    let boiling = Temperature::boiling();

    cold.show_temp();
    boiling.show_temp();
}
