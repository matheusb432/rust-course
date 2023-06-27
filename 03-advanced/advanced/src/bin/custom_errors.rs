use chrono::{DateTime, Duration, Utc};
use thiserror::Error;

struct SubwayPass {
    id: usize,
    funds: isize,
    expires: DateTime<Utc>,
}

// NOTE Creating a custom error with the thiserror crate
#[derive(Error, Debug)]
enum PassError {
    #[error("Pass expired")]
    PassExpired,
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(isize),
    #[error("Card read error: {0}")]
    ReadError(String),
}

fn swipe_card() -> Result<SubwayPass, PassError> {
    Ok(SubwayPass {
        id: 0,
        funds: 200,
        expires: Utc::now() + Duration::weeks(52),
    })
}

fn use_pass(pass: &mut SubwayPass, cost: isize) -> Result<(), PassError> {
    if Utc::now() > pass.expires {
        // NOTE returning a custom error
        return Err(PassError::PassExpired);
    }

    if pass.funds < cost {
        return Err(PassError::InsufficientFunds(pass.funds));
    }

    pass.funds -= cost;
    Ok(())
}
fn main() {
    // NOTE and_then() executes on the Ok() variant of the Result
    let pass_status = swipe_card().and_then(|mut pass| use_pass(&mut pass, 3));
    match pass_status {
        Ok(_) => println!("Pass accepted!"),
        Err(e) => println!("Error: {}", e),
    }
}
