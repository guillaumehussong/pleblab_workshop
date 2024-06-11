fn main() {
    println!("Hello, {}!", "World");
    dotenv::from_filename(".env").ok();

    let result_descriptor = std::env::var("WALLET_DESCRIPTOR");

    let descriptor = match result_descriptor {
            Ok(descriptor) => descriptor,
            Err(_) => panic!("Error"),
    };

    println!("Wallet descriptor: {}", descriptor);
    dbg!(descriptor);
} 
