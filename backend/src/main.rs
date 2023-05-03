pub mod messages;
pub mod middleware;
pub mod routers;
pub mod state;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    Ok(())
}
