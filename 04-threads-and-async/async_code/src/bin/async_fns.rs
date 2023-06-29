async fn life() -> u32 {
    42
}

#[tokio::main]
pub async fn main() {
    let future = life();
    // ? Awaiting a future is done by calling .await on it.
    let value = future.await;
    println!("The answer to life is: {}", value);
    let value = life().await;
    println!("The answer to life is: {}", value);
}
