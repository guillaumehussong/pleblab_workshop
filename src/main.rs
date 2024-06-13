use bdk::{bitcoin::Network, database::MemoryDatabase, Wallet};

fn function_that_takes_a_string(s: &str) {
    println!("{}", s);
}

fn main() -> anyhow::Result<()>{
    println!("Hello, {}!", "Pleb");
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let descriptor = std::env::var("WALLET_DESCRIPTOR")?;

    let str_descriptor = "wpkh(tprv8ZgxMBicQKsPd8JL5Q7Z)";
    let string_desc = String::from(&descriptor);

    function_that_takes_a_string(str_descriptor);

    let wallet = Wallet::new(str_descriptor, None, Network::Testnet, MemoryDatabase::default())?;

    Ok(())
} 
