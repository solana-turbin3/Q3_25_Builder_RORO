use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;

// Constants
const RPC_URL: &str = "https://api.devnet.solana.com";
const PROGRAM_ID: &str = "TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM";
const COLLECTION_ADDRESS: &str = "5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2";
const MPL_CORE_PROGRAM_ID: &str = "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d";

// PDA account for "prereqs"
fn derive_account_address(user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"prereqs", user.as_ref()],
        &Pubkey::from_str(PROGRAM_ID).unwrap(),
    )
}

// PDA authority for "collection"
fn derive_authority_address(collection: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"collection", collection.as_ref()],
        &Pubkey::from_str(PROGRAM_ID).unwrap(),
    )
}

// submit_rs instruction
fn create_submit_rs_instruction(
    user: &Pubkey,
    account: &Pubkey,
    mint: &Pubkey,
    collection: &Pubkey,
    authority: &Pubkey,
) -> Instruction {
    // discriminator for 'submit_rs' from IDL
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    Instruction {
        program_id: Pubkey::from_str(PROGRAM_ID).unwrap(),
        accounts: vec![
            AccountMeta::new(*user, true),              // user (writable + signer)
            AccountMeta::new(*account, false),          // account (writable)
            AccountMeta::new(*mint, true),              // mint (writable + signer)
            AccountMeta::new(*collection, false),       // collection (writable)
            AccountMeta::new_readonly(*authority, false), // authority
            AccountMeta::new_readonly(Pubkey::from_str(MPL_CORE_PROGRAM_ID).unwrap(), false), // mpl_core_program
            AccountMeta::new_readonly(system_program::id(), false), // system_program
        ],
        data,
    }
}

#[test]
fn enroll_q3() {
    let client = RpcClient::new(RPC_URL);
    
    let signer = read_keypair_file("Turbin3-wallet.json")
        .expect("Couldn't find wallet file");
    
    // PDA account for "prereqs"
    let (account, _bump) = derive_account_address(&signer.pubkey());
    
    // Collection address
    let collection = Pubkey::from_str(COLLECTION_ADDRESS).unwrap();

    // PDA authority for "collection"
    let (authority, _authority_bump) = derive_authority_address(&collection);
    
    // mint keypair
    let mint = Keypair::new();
    
    println!("User: {}", signer.pubkey());
    println!("Account PDA: {} (already initialized from TypeScript)", account);
    println!("Collection: {}", collection);
    println!("Authority PDA: {}", authority);
    println!("Mint: {}", mint.pubkey());
    
    println!("\n Executing Submit RS Transaction");
    
    let submit_rs_instruction = create_submit_rs_instruction(
        &signer.pubkey(),
        &account,
        &mint.pubkey(),
        &collection,
        &authority,
    );
    
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    
    let submit_rs_transaction = Transaction::new_signed_with_payer(
        &[submit_rs_instruction],
        Some(&signer.pubkey()),
        &[&signer, &mint], 
        recent_blockhash,
    );
    
    match client.send_and_confirm_transaction(&submit_rs_transaction) {
        Ok(signature) => {
            println!("Submit Success!");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);
            println!("NFT minted at: {}", mint.pubkey());
        },
        Err(e) => {
            println!("Submit RS failed: {}", e);
        }
    }
} 