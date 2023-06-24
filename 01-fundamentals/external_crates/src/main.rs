// NOTE using an external crate
use humantime::format_duration;
use std::time::Duration;

fn main() {
    let d = Duration::from_secs(9876);
    println!("{}", format_duration(d));
}
