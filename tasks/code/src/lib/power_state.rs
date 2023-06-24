// * a20.rs

// NOTE Setting constants for the power states, more memory efficient than using strings
const OFF: &'static str = "off";
const SLEEP: &'static str = "sleep";
const REBOOT: &'static str = "reboot";
const SHUTDOWN: &'static str = "shutdown";
const HIBERNATE: &'static str = "hibernate";

// * 2.
pub enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    pub fn from_str(value: &str) -> Result<Self, String> {
        Self::from_lowercase_str(value.to_lowercase().as_str())
    }

    pub fn print(&self) {
        // * 3.
        match self {
            PowerState::Off => println!("Turning off..."),
            PowerState::Sleep => println!("Going to sleep..."),
            PowerState::Reboot => println!("Rebooting..."),
            PowerState::Shutdown => println!("Shutting down..."),
            PowerState::Hibernate => println!("Hibernating..."),
        }
    }

    // NOTE Handling the conversion from a string to a power state enum value
    fn from_lowercase_str(value: &str) -> Result<Self, String> {
        let power_state = match value {
            OFF => PowerState::Off,
            SLEEP => PowerState::Sleep,
            REBOOT => PowerState::Reboot,
            SHUTDOWN => PowerState::Shutdown,
            HIBERNATE => PowerState::Hibernate,
            // NOTE If the value is not one of the power states, return an error
            _ => {
                // * 4.
                return Err("Not a valid power state!".to_owned());
            }
        };

        Ok(power_state)
    }
}
