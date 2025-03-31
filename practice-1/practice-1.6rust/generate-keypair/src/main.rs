use solana_sdk::signature::{Keypair, Signer};

fn main() {
    let keypair = Keypair::new();
    let public_key = keypair.pubkey();

    println!("public key: {}", public_key);
    println!("private key: {:?}", keypair.to_bytes());
}
