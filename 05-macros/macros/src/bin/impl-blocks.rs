#![allow(dead_code, unused_variables)]

#[derive(Clone, Copy)]
struct Volume(usize);

trait ReagentContainer {
    fn max_volume(&self) -> Volume;
    fn current_volume(&self) -> Volume;
}

// NOTE Macro that implements a trait for a struct
macro_rules! impl_reagent_container {
    ($container:ty : $volume:literal) => {
        impl ReagentContainer for $container {
            fn max_volume(&self) -> Volume {
                Volume($volume)
            }

            fn current_volume(&self) -> Volume {
                self.current_volume
            }
        }
    };
}

struct TallFlask {
    current_volume: Volume,
}

struct TestTube {
    current_volume: Volume,
}

struct Pipette {
    current_volume: Volume,
}

struct OtherTube {
    current_volume: Volume,
    max_volume: Volume,
}

impl ReagentContainer for OtherTube {
    fn max_volume(&self) -> Volume {
        self.max_volume
    }

    fn current_volume(&self) -> Volume {
        self.current_volume
    }
}

// ? Calling the macro will implement the trait for the structs
impl_reagent_container!(TallFlask: 32);
impl_reagent_container!(TestTube: 10);
impl_reagent_container!(Pipette: 4);

fn main() {
    let test_tube = TestTube {
        current_volume: Volume(5),
    };

    let tall_flask = TallFlask {
        current_volume: Volume(10),
    };

    let pipette = Pipette {
        current_volume: Volume(2),
    };

    println!(
        "test_tube: {}/{}",
        test_tube.current_volume().0,
        test_tube.max_volume().0
    );

    println!(
        "tall_flask: {}/{}",
        tall_flask.current_volume().0,
        tall_flask.max_volume().0
    );

    println!(
        "pipette: {}/{}",
        pipette.current_volume().0,
        pipette.max_volume().0
    );
}
