use std::thread::sleep;
use std::time::Duration;

use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signer::{
        keypair::{self, Keypair},
        Signer,
    },
};
use std::{env, path};


fn main() {
    // let keypair: Keypair = Keypair::new();
    let home_path = env::var_os("HOME").unwrap();
    let default_keypair_path = ".config/solana/id.json"; // ! update if you want to use a different path
    let default_keypair_path = path::PathBuf::from(home_path).join(default_keypair_path);
    let keypair: Keypair = keypair::read_keypair_file(default_keypair_path).expect("error reading keypair from path");

    // 2、连接到本地的Solana节点
    let rpc_client = RpcClient::new("http://localhost:8899".to_string());

    //3、查询请求空投前的账户余额
    let balance: u64 = rpc_client.get_balance(&keypair.pubkey()).unwrap();
    println!("空投交易发起前的账户余额: {:?}", balance);

    //4、向生成的地址请求空投1SOL
    let signature: Signature = rpc_client.request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL).unwrap();
    println!("空投交易成功，交易签名: {:?}", signature);

    //5、确认交易是否成功
    let mut confirmed: bool = rpc_client.confirm_transaction(&signature).unwrap();
    while !confirmed {
        println!("交易确认失败，继续等待...");
        sleep(Duration::from_secs(5));
        confirmed = rpc_client.confirm_transaction(&signature).unwrap();
    }
    if confirmed {
        println!("交易已成功确认");
    }

    //6、查询请求空投成功后的账户余额
    let balance: u64 = rpc_client.get_balance(&keypair.pubkey()).unwrap();
    println!("空投交易发起后的账户余额: {:?}", balance);
}
