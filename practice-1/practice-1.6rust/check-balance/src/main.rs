use dotenvy::dotenv;
use solana_sdk::{ native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer};
use solana_client::rpc_client::RpcClient;
use std::env;
fn main() {
    let secret_key: String = load_secrets("SECRET_KEY").unwrap();
    let keypair: Keypair = Keypair::from_bytes(&str_to_u8_array(&secret_key)).unwrap();
    println!("Wallet address: {:?}", keypair.pubkey());

    check_balance(&keypair.pubkey().to_string());
}

fn check_balance(key: &str) {
    let client = RpcClient::new("https://api.devnet.solana.com");
    
    let token_account: Pubkey = Pubkey::from_str_const(key);

    let balance = client.get_balance(&token_account).unwrap();
    println!("💰 The balance for the wallet at address\n{:?}\nis: {:?} SOL",token_account, balance/LAMPORTS_PER_SOL);
}

fn load_secrets(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let is_ok = dotenv().is_ok();
    if is_ok {
        println!("Environment variables loaded successfully.");
    } else {
        println!("Failed to load environment variables.");
    }
    let secret_key = env::var(name)?;

    Ok(secret_key)
}

fn str_to_u8_array(data_str: &str) -> [u8; 64] {
    let data_parts: Vec<&str> = data_str[1..data_str.len() - 1].split(",").collect();

    // Parse each string to a u8
    let data: Vec<u8> = data_parts.iter()
        .map(|part| part.trim().parse::<u8>().unwrap())
        .collect();
    // Convert Vec<u8> to [u8; 64]
    let res: [u8; 64] = data.try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!("Expected a Vec of length 64 but it was {}", v.len())
    });
    
    res
}