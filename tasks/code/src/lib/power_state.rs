// * a20.rs

// NOTE Setting constants for the power states, more memory efficient than using strings
pub const OFF: &'static str = "off";
pub const SLEEP: &'static str = "sleep";
pub const REBOOT: &'static str = "reboot";
pub const SHUTDOWN: &'static str = "shutdown";
pub const HIBERNATE: &'static str = "hibernate";

// * 2.
pub enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    pub fn new(value: &str) -> Option<Self> {
        Self::new_lowercase(value.to_lowercase().as_str())
    }

    pub fn res_from_str(value: &str) -> Result<Self, String> {
        Self::res_from_lowercase_str(value.to_lowercase().as_str())
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
    fn new_lowercase(value: &str) -> Option<Self> {
        let power_state = match value {
            OFF => PowerState::Off,
            SLEEP => PowerState::Sleep,
            REBOOT => PowerState::Reboot,
            SHUTDOWN => PowerState::Shutdown,
            HIBERNATE => PowerState::Hibernate,
            // NOTE If the value is not one of the power states, return an error
            _ => {
                return None;
            }
        };

        Some(power_state)
    }

    fn res_from_lowercase_str(value: &str) -> Result<Self, String> {
        match Self::new(value) {
            Some(power_state) => Ok(power_state),
            None => Err("Not a valid power state!".to_owned()),
        }
    }
}
