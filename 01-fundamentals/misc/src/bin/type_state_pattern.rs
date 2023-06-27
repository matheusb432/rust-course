// NOTE Typestates are useful to properly transition between states of a struct
// * They can also be useful to disallow access to a missing resource

#[derive(Debug)]
struct Employee<TState> {
    name: String,
    state: TState,
}

impl<TState> Employee<TState> {
    // NOTE transition takes self, so it invalidates the previous state
    // * the transition is private so that proper state transitions are followed
    fn transition<TNextState>(self, next_state: TNextState) -> Employee<TNextState> {
        Employee {
            name: self.name,
            state: next_state,
        }
    }
}

struct Agreement;
struct Signature;
struct Training;
#[derive(Debug)]
struct FailedTraining {
    score: u8,
}
#[derive(Debug)]
struct OnboardingComplete {
    score: u8,
}

// NOTE Using the typestate pattern can restrict the state transitions to it's specific order
impl Employee<Agreement> {
    pub fn new(name: &str) -> Self {
        Employee {
            name: name.to_owned(),
            state: Agreement,
        }
    }
    pub fn read_agreement(self) -> Employee<Signature> {
        self.transition(Signature)
    }
}
impl Employee<Signature> {
    pub fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}

impl Employee<Training> {
    pub fn train(
        self,
        score: u8,
    ) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
        if score >= 7 {
            Ok(self.transition(OnboardingComplete { score }))
        } else {
            Err(self.transition(FailedTraining { score }))
        }
    }
}

fn main() {
    // NOTE due to the implementations, the struct must follow this logical order to transition to it's final state
    let onboarded_success = Employee::new("John").read_agreement().sign().train(9);
    let onboarded_fail = Employee::new("Doe").read_agreement().sign().train(6);

    dbg!(onboarded_success.ok().unwrap());
    dbg!(onboarded_fail.err().unwrap());
}
