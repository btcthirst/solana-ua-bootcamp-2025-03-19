use dotenvy;
use std::env;
use solana_sdk::{signature::Keypair, signer::Signer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    _ = dotenvy::dotenv().is_ok();
    
    let secret_key = env::var("SECRET_KEY")?;
    println!("Secret Key: {:?}", str_to_u8_array(secret_key.as_str()));
    
    let keypair: Keypair = Keypair::from_bytes(&str_to_u8_array(secret_key.as_str()))?;
    
    let public_key = keypair.pubkey().to_string();
    println!("Public Key: {}", &public_key); 
    Ok(())
}

fn str_to_u8_array(data_str: &str) -> [u8; 64] {
    let data_parts: Vec<&str> = data_str[1..data_str.len() - 1].split(",").collect();

    // Parse each string to a u8
    let data: Vec<u8> = data_parts.iter()
        .map(|part| part.trim().parse::<u8>().unwrap())
        .collect();
    // Convert Vec<u8> to [u8; 64]
    // Ensure the length is 64
    let res: [u8; 64] = data.try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!("Expected a Vec of length 64 but it was {}", v.len())
    });
    
    res
}