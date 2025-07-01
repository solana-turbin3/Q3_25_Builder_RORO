import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import * as fs from "fs";

const secret = JSON.parse(fs.readFileSync("./dev-wallet.json", "utf-8"));
const keypair = Keypair.fromSecretKey(new Uint8Array(secret));

// Set up the connection to Solana Devnet
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// Create a wallet and provider
const wallet = new Wallet(keypair);
const provider = new AnchorProvider(connection, wallet, { commitment: "confirmed" });

// Program ID from the provided address
const programId = new PublicKey("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM");

// Your github username
const github = "rxrxrxrx"; // Replace with your actual github username

// Collection address from Q3 subject
const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");

// MPL Core Program ID
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

// Generate mint keypair for the NFT
const mintTs = Keypair.generate();

// Main function
(async () => {
  try {
    // Fetch the IDL directly from the blockchain
    const idl = await Program.fetchIdl(programId, provider);
    if (!idl) {
      throw new Error("IDL not found for the program");
    }
    
    console.log("IDL fetched successfully");
    
    // Create the program with the fetched IDL
    const program = new Program(idl, provider);
    
    // Create the PDA for the account with correct seed "prereqs"
    const account_seeds = [
      Buffer.from("prereqs"),
      keypair.publicKey.toBuffer(),
    ];
    
    const [account_key, _account_bump] = PublicKey.findProgramAddressSync(
      account_seeds,
      programId
    );
    
    // Create the authority PDA for the collection
    const authority_seeds = [
      Buffer.from("collection"),
      mintCollection.toBuffer(),
    ];
    
    const [authority, _authority_bump] = PublicKey.findProgramAddressSync(
      authority_seeds,
      programId
    );
    
    console.log("Account PDA address:", account_key.toString());
    console.log("Authority PDA address:", authority.toString());
    console.log("Mint address:", mintTs.publicKey.toString());
    
    // Execute the initialize transaction
    console.log("\n=== Executing Initialize Transaction ===");
    try {
      const txhash1 = await program.methods
        .initialize(github)
        .accountsPartial({
          user: keypair.publicKey,
          account: account_key,
          system_program: SystemProgram.programId,
        })
        .signers([keypair])
        .rpc();
        
      console.log(`Success:
      https://explorer.solana.com/tx/${txhash1}?cluster=devnet`);
    } catch (e) {
      console.error(`Initialize failed: ${e}`);
      return;
    }
    
    // Wait a bit between transactions
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    // Execute the submit_ts transaction
    console.log("\n=== Executing Submit TS Transaction ===");
    try {
      const txhash2 = await program.methods
        .submitTs()
        .accountsPartial({
          user: keypair.publicKey,
          account: account_key,
          mint: mintTs.publicKey,
          collection: mintCollection,
          authority: authority,
          mpl_core_program: MPL_CORE_PROGRAM_ID,
          system_program: SystemProgram.programId,
        })
        .signers([keypair, mintTs])
        .rpc();
        
      console.log(`Submit:
      https://explorer.solana.com/tx/${txhash2}?cluster=devnet`);
      
      console.log(`NFT minted at: ${mintTs.publicKey.toString()}`);
      
    } catch (e) {
      console.error(`Submit TS failed: ${e}`);
    }
    
  } catch(e) {
    console.error(`Something went wrong: ${e}`);
  }
})(); 