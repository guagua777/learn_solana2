use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, signature::Signer, signer::keypair::Keypair,
    transaction::Transaction,
};
use solana_system_interface::instruction::transfer;

#[tokio::main]
async fn main() -> Result<()> {
    let connection = RpcClient::new_with_commitment(
        "http://localhost:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    // Fetch the latest blockhash and last valid block height
    let blockhash = connection.get_latest_blockhash().await?;

    // Generate sender and recipient keypairs
    let sender = Keypair::new();
    let recipient = Keypair::new();

    // Create a transfer instruction for transferring SOL from sender to recipient
    let transfer_instruction = transfer(
        &sender.pubkey(),
        &recipient.pubkey(),
        LAMPORTS_PER_SOL / 100, // 0.01 SOL
    );

    let mut transaction =
        Transaction::new_with_payer(&[transfer_instruction], Some(&sender.pubkey()));
    transaction.sign(&[&sender], blockhash);

    println!("{:#?}", transaction);

    Ok(())
}



// {
//   "signatures": [
//     "2fPXZtQGWWj6suxfc55FBQiexS8hEhNELqasSL5DRYa1RB1GChHz86Cyy8ukiVwA6qbq91P4cY1FuvTuYtmTHmJP"
//   ],
//   "message": {
//     "header": {
//       "num_required_signatures": 1,
//       "num_readonly_signed_accounts": 0,
//       "num_readonly_unsigned_accounts": 1
//     },
//     "account_keys": [
//       "9CpbtdXfUTgLMJL8DEAeEm8thERJPwDuruohjvUuzY7m",
//       "6jELNgS8Q35sF4QZCvwgyKGaKrbcm8P5QcNWUyAb5ekJ",
//       "11111111111111111111111111111111"
//     ],
//     "recent_blockhash": "3P7CVQ9nwXx4B37MvBzghzbcM9K9p5xo7ivDE8W78dCi",
//     "instructions": [
//       {
//         "program_id_index": 2, // 注意这个是索引，不是程序ID，代表account_keys中的相应程序ID，此处为第三个元素，即全1的系统程序
//         "accounts": [0, 1], // 代表account_keys中的相应账户，此处为第一个和第二个元素，即sender和recipient
//         "data": [2, 0, 0, 0, 128, 150, 152, 0, 0, 0, 0, 0]
//       }
//     ]
//   }
// }



// {
//   "blockTime": 1745196488,
//   "meta": {
//     "computeUnitsConsumed": 150,
//     "err": null,
//     "fee": 5000,
//     "innerInstructions": [],
//     "loadedAddresses": {
//       "readonly": [],
//       "writable": []
//     },
//     "logMessages": [
//       "Program 11111111111111111111111111111111 invoke [1]",
//       "Program 11111111111111111111111111111111 success"
//     ],
//     "postBalances": [989995000, 10000000, 1],
//     "postTokenBalances": [],
//     "preBalances": [1000000000, 0, 1],
//     "preTokenBalances": [],
//     "rewards": [],
//     "status": {
//       "Ok": null
//     }
//   },
//   "slot": 13049,
//   "transaction": {
//     "message": {
//       "header": {
//         "numReadonlySignedAccounts": 0,
//         "numReadonlyUnsignedAccounts": 1,
//         "numRequiredSignatures": 1
//       },
//       "accountKeys": [
//         "8PLdpLxkuv9Nt8w3XcGXvNa663LXDjSrSNon4EK7QSjQ",
//         "7GLg7bqgLBv1HVWXKgWAm6YoPf1LoWnyWGABbgk487Ma",
//         "11111111111111111111111111111111"
//       ],
//       "recentBlockhash": "7ZCxc2SDhzV2bYgEQqdxTpweYJkpwshVSDtXuY7uPtjf",
//       "instructions": [
//         {
//           "accounts": [0, 1],
//           "data": "3Bxs4NN8M2Yn4TLb",
//           "programIdIndex": 2,
//           "stackHeight": null
//         }
//       ],
//       "indexToProgramIds": {}
//     },
//     "signatures": [
//       "3jUKrQp1UGq5ih6FTDUUt2kkqUfoG2o4kY5T1DoVHK2tXXDLdxJSXzuJGY4JPoRivgbi45U2bc7LZfMa6C4R3szX"
//     ]
//   },
//   "version": "legacy"
// }