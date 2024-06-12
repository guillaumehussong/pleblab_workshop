fn main() -> anyhow::Result<()>{
    println!("Hello, {}!", "Pleb");
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let descriptor = std::env::var("WALLET_DESCRIPTOR")?;

    println!("Wallet descriptor: {}", descriptor);
    dbg!(descriptor);

    Ok(())
} 
