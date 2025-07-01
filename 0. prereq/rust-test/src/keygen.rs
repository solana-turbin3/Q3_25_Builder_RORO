use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};

#[test]
fn keygen() {
    let kp = Keypair::new();
    println!("Solana wallet: {}", kp.pubkey().to_string());
    println!("{:?}", kp.to_bytes());
}