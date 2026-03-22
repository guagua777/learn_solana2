use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction}, 
    pubkey::Pubkey, 
    signature::{Keypair, Signer}, 
    signer::EncodableKey, 
    transaction::Transaction
};
use std::str::FromStr;
use solana_commitment_config::CommitmentConfig;

fn main() {
    // Replace with your actual program ID from deployment
    let program_id = Pubkey::from_str("74aPmsMJt2zMUBAB5tDkguSkNHdoE138tDvCvdzvcrti")
        .expect("Invalid program ID");

    // Connect to local cluster
    let rpc_url = String::from("http://localhost:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Generate a new keypair for paying fees
    // let payer = Keypair::new();
    let payer = Keypair::read_from_file("./keys/DevAN8oX8J58rsnaBMDaCtAsQMEu5hBUUYtjdVApMFD8.json")
        .expect("Failed to read keypair from file");

    // let instruction = Instruction::new_with_bytes(
    //     program_id,
    //     &[], // empty instruction data
    //     vec![], // no accounts needed
    // );

    // let instruction = Instruction::new_with_bytes(
    //     program_id,
    //     &[], // empty instruction data
    //     vec![
    //         AccountMeta {
    //             pubkey: payer.pubkey(),
    //             is_signer: false,
    //             is_writable: false, 
    //         },
    //     ], // no accounts needed
    // );


    let new_account = Keypair::new();
    // let instruction = Instruction::new_with_bytes(
    //     program_id,
    //     &[], // empty instruction data
    //     vec![
    //         AccountMeta {
    //             pubkey: new_account.pubkey(),
    //             is_signer: false,
    //             is_writable: false, 
    //         },
    //     ], // no accounts needed
    // );

    let instruction = Instruction::new_with_bytes(
        program_id,
        &[], // empty instruction data
        vec![
            AccountMeta {
                pubkey: new_account.pubkey(),
                is_signer: true,
                is_writable: false, 
            },
        ], // no accounts needed
    );


    let mut transaction =
        Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));

    let blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    // transaction.sign(&[&payer], blockhash);
    transaction.sign(&[&payer, &new_account], blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("Counter initialized!");
            println!("Transaction: {}", signature);
        }
        Err(err) => {
            eprintln!("Failed to initialize counter: {}", err);
        }
    }
}