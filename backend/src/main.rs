use dotenvy::dotenv;

mod database;

fn main() {
    // Load environment variables
    let _ = dotenv();

    println!("Hello, world!");
}
