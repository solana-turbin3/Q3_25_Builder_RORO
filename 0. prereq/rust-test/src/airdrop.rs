use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
use std::path::Path;

const RPC_URL: &str = "https://api.devnet.solana.com";

#[test]
fn airdrop() {
    let wallet_path = Path::new("src/Turbin3-wallet.json");
    let keypair = read_keypair_file(wallet_path).expect("Couldn't find wallet file");
    let client = RpcClient::new(RPC_URL);

    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(s) => {
            println!("Success");
            println!("https://explorer.solana.com/tx/{}?zcluster=devnet", s.to_string());
        },
        Err(e) => println!("something went wrong: {}", e.to_string())
    }
}