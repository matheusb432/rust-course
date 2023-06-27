fn main() {
    #[derive(Debug)]
    struct Uppercase(String);

    // NOTE Implementing the From trait allows for a type to define how to create itself from another type,
    // * providing a very simple mechanism for converting between several types.
    impl From<String> for Uppercase {
        fn from(data: String) -> Self {
            Uppercase(data.to_uppercase())
        }
    }
    // * Implementing it for different types
    impl From<&str> for Uppercase {
        fn from(data: &str) -> Self {
            Uppercase(data.to_uppercase())
        }
    }

    let upper = Uppercase::from("lowercase");
    dbg!(upper);
    let upper: Uppercase = "lowercase".into();
    dbg!(upper);

    #[derive(Debug)]
    enum KeyPress {
        Down,
        Up,
    }

    struct KeyEvent {
        keycode: u16,
        state: KeyPress,
    }

    #[derive(Debug)]
    enum InputEvent {
        Key(u16, KeyPress),
        Mouse,
    }

    impl From<KeyEvent> for InputEvent {
        fn from(ev: KeyEvent) -> Self {
            InputEvent::Key(ev.keycode, ev.state)
        }
    }

    let input_ev = InputEvent::from(KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    });
    dbg!(input_ev);
    let key_ev = KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    };
    let input_ev: InputEvent = key_ev.into();
    dbg!(input_ev);

    use thiserror::Error;

    #[derive(Error, Debug)]
    enum NetworkError {
        #[error("connection timed out")]
        Timeout,
    }

    #[derive(Error, Debug)]
    enum DatabaseError {
        #[error("eror querying database")]
        QueryFailure,
    }

    #[derive(Error, Debug)]
    enum ApiError {
        #[error("network error: {0}")]
        Network(NetworkError),
        #[error("database error: {0}")]
        // NOTE the `thiserror` crate already provides the `#[from]` macro to implement the from block
        Database(#[from] DatabaseError),
    }

    // NOTE Implementing error creators from a different error
    // * Not actually necessary if using `thiserror` crate with the #[from] macro
    impl From<NetworkError> for ApiError {
        fn from(value: NetworkError) -> Self {
            Self::Network(value)
        }
    }

    fn do_stuff() -> Result<(), ApiError> {
        // NOTE This will convert NetworkError to ApiError by using the .from() impl
        Err(NetworkError::Timeout)?
    }

    println!("{:?}", do_stuff().err());
}
