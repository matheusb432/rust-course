use dotenvy::dotenv;
use std::env;

fn main() {
    // NOTE dotenvy will load .env file in the current directory or in the parent directory
    dotenv().ok();

    match env::var("PORT") {
        Ok(port) => println!("Port is {}", port),
        Err(_) => println!("No PORT variable found"),
    };
    let some_var = env::var("SOME_VAR").unwrap_or_else(|_| "default value".to_string());

    println!("SOME_VAR is {}", some_var);
}
