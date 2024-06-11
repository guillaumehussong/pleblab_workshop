fn main() {
    println!("Hello, {}!", "World");
    dotenv::from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR").unwrap();

    println!("Wallet descriptor: {}", descriptor);
    dbg!(descriptor);
} 
